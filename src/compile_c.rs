use std::collections::HashMap;

use regex::Regex;
use sha2::{Digest, Sha256};

use crate::arity::calculate_arity;
use crate::debug;
use crate::numbers::Number;
use crate::types::{Expression, Value};

pub enum CCode {
    Block(Vec<String>),
    Expression(Vec<String>),
}

/// A helper macro to generate functions that operate on two integers and floats
macro_rules! numeric_binop {
    ($lines:expr, $op:literal) => {{
        let op = stringify!($op).to_string().trim_matches('"').to_string();

        $lines.push(format!("
    {{
        Value *b = stack_ptr--;
        Value *a = stack_ptr--;

        if (a->type != b->type) {{
            
        }}
        
        if (a->type == TAG_NUMBER_INTEGER) {{
            Value result = {{.type=TAG_NUMBER_INTEGER, .as_integer=a->as_integer {op} b->as_integer}};
            *(++stack_ptr) = result;
        }} else if (a->type == TAG_NUMBER_FLOAT) {{
            Value result = {{.type=TAG_NUMBER_FLOAT, .as_float=a->as_float {op} b->as_float}};
            *(++stack_ptr) = result;
        }}
    }}
"));
    }};
}

/// A helper macro to generate functions that operate on two integers and floats
macro_rules! numeric_compare {
    ($lines:expr, $op:literal) => {{
        let op = stringify!($op).to_string().trim_matches('"').to_string();

        $lines.push(format!("
    {{
        Value *b = stack_ptr--;
        Value *a = stack_ptr--;
        coerce(a, b);
        
        if (a->type == TAG_NUMBER_INTEGER) {{
            Value result = {{.type=TAG_BOOLEAN, .as_boolean=a->as_integer {op} b->as_integer}};
            *(++stack_ptr) = result;
        }} else if (a->type == TAG_NUMBER_FLOAT) {{
            Value result = {{.type=TAG_BOOLEAN, .as_boolean=a->as_float {op} b->as_float}};
            *(++stack_ptr) = result;
        }}
    }}
"));
    }};
}

/// Sanitize names
fn sanitize_name(name: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
    let cleaned_name = re.replace_all(name, "").to_string();

    if cleaned_name == name {
        cleaned_name
    } else {
        let mut hasher = Sha256::new();
        hasher.update(name);
        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);
        format!("{}_{}", cleaned_name, &hash_hex[0..4])
    }
}

/// Collect the names used so we can assign each an integer value
fn collect_names(ast: &Expression) -> HashMap<String, usize> {
    let mut names = HashMap::new();

    fn collect_names_expr(expr: &Expression, names: &mut HashMap<String, usize>) {
        match expr {
            Expression::Identifier(_)
            | Expression::DottedIdentifier(_)
            | Expression::Literal(_)
            | Expression::Bang(_)
            | Expression::Dollar(_) => {
                // Do nothing, no names possible
            }
            Expression::List(values) => {
                for value in values {
                    collect_names_expr(value, names);
                }
            }
            Expression::Block(exprs) => {
                for expr in exprs {
                    collect_names_expr(expr, names);
                }
            }
            Expression::Group(exprs) => {
                for expr in exprs {
                    collect_names_expr(expr, names);
                }
            }
            Expression::At(expr) => {
                match expr.as_ref() {
                    Expression::Identifier(id) => {
                        let id = sanitize_name(id);
                        if !names.contains_key(&id) {
                            log::debug!("Adding name: {} @ {}", id, names.len());
                            names.insert(id.clone(), names.len());
                        }
                    }
                    Expression::List(id_exprs) => {
                        for id_expr in id_exprs {
                            match id_expr {
                                Expression::Identifier(id) => {
                                    let id = sanitize_name(id);
                                    if !names.contains_key(&id) {
                                        log::debug!("Adding name: {} @ {}", id, names.len());
                                        names.insert(id.clone(), names.len());
                                    }
                                }
                                _ => panic!(
                                    "Unexpected @ expression when collecting names: {}",
                                    expr
                                ),
                            }
                        }
                    }
                    Expression::Literal(Value::Number(Number::Integer(_))) => {} // ignore numeric @ expressions
                    _ => panic!("Unexpected @ expression when collecting names: {}", expr),
                }
            }
        }
    }

    collect_names_expr(ast, &mut names);
    names
}

/// Compile the AST into C code
pub fn compile(ast: Expression) -> String {
    let mut template = include_str!("../compile_c_includes/template.c").to_string();

    // Debug flag
    {
        unsafe {
            if debug::ENABLED {
                template = template.replace("/*{DEBUG}*/", "#define DEBUG 1");
            } else {
                template = template.replace("/*{DEBUG}*/", "// #define DEBUG 1");
            }
        }
    }

    // Generate the NAMES
    {
        let names = collect_names(&ast);
        log::debug!("collected names: {:?}", names);

        let mut names_block = String::new();

        for (name, index) in names.iter() {
            names_block.push_str(&format!("#define NAME_{name} {index}\n").to_string());
        }
        names_block.push('\n');

        for (name, index) in names.iter() {
            names_block.push_str(&format!("#define NAME_INDEX_{index} {name}\n").to_string());
        }
        names_block.push('\n');

        names_block.push_str("char* get_name(uint32_t index) {\n");
        names_block.push_str("\tswitch (index) {\n");
        for (name, index) in names.iter() {
            names_block.push_str(&format!("\t\tcase {index}: return \"{name}\";\n").to_string());
        }
        names_block.push_str("\t\tdefault:\n\t\t\tfprintf(stderr, \"Undefined name for index %u\", index);\n\t\t\texit(1);\n");
        names_block.push_str("\t}\n}\n");

        template = template.replace("/*{NAMES}*/", &names_block);
    }

    // Generate the BLOCKS
    {
        let blocks = {
            /// Helper function to compile a specific block to be output later
            fn compile_block(
                arity: (usize, usize),
                body: &Vec<Expression>,
                blocks: &mut Vec<Vec<String>>,
            ) -> usize {
                log::debug!("compile_block({arity:?}, {body:?})");

                let index = blocks.len();
                blocks.push(vec![]); // Throwaway vec to hold the index
                let mut lines = vec![];

                let (arity_in, arity_out) = arity;

                lines.push(format!(
                    "    // block: {}",
                    body.iter()
                        .map(|ex| ex.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ));
                lines.push(format!(
                    "\n    // Store the current stack pointer with arity_in={arity_in}"
                ));
                lines.push(format!("    *(++frame_ptr) = (stack_ptr - {arity_in});\n"));

                // Compile the block itself
                for expr in body {
                    for line in compile_expr(expr.clone(), blocks) {
                        lines.push(line);
                    }
                }

                // Pop the block off the stack
                lines.push(format!(
                    "    // Pop the block off the stack, preserving arity_out={arity_out} values"
                ));
                lines.push(format!(
                    "    Value* return_ptr = (stack_ptr - {arity_out});"
                ));
                lines.push(format!("    stack_ptr =  *(frame_ptr--);"));
                for _ in 0..arity_out {
                    lines.push(format!("    *(++stack_ptr) = *(++return_ptr);"));
                }

                blocks[index] = lines;
                index
            }

            /// Compile a single expression into strings
            fn compile_expr(expr: Expression, blocks: &mut Vec<Vec<String>>) -> Vec<String> {
                log::debug!("compile_expr({expr})");

                let mut lines = vec![];
                lines.push(format!("    // {expr}")); // TODO: Flag for verbose mode

                match expr {
                    Expression::Identifier(id) => {
                        match id.as_str() {
                            // Built in numeric functions
                            "+" => numeric_binop!(lines, "+"),
                            "-" => numeric_binop!(lines, "-"),
                            "*" => numeric_binop!(lines, "*"),
                            "/" => numeric_binop!(lines, "/"),

                            // % only allows integers
                            "%" => lines.push(
                                include_str!("../compile_c_includes/builtins/mod.c").to_string(),
                            ),

                            // Built in numeric comparisons
                            "<" => numeric_compare!(lines, "<"),
                            "<=" => numeric_compare!(lines, "<="),
                            "=" => numeric_compare!(lines, "=="),
                            "!=" => numeric_compare!(lines, "!="),
                            ">=" => numeric_compare!(lines, ">="),
                            ">" => numeric_compare!(lines, ">"),

                            // Built ins
                            "read" => lines.push(
                                include_str!("../compile_c_includes/builtins/read.c").to_string(),
                            ),
                            "write" => lines.push(
                                include_str!("../compile_c_includes/builtins/write.c").to_string(),
                            ),
                            "writeln" => {
                                lines.push(
                                    include_str!("../compile_c_includes/builtins/write.c")
                                        .to_string(),
                                );
                                lines.push("printf(\"\\n\");".to_string());
                            }
                            "newline" => lines.push("printf(\"\\n\");".to_string()),
                            "loop" => lines.push(
                                include_str!("../compile_c_includes/builtins/loop.c").to_string(),
                            ),
                            "if" => lines.push(
                                include_str!("../compile_c_includes/builtins/if.c").to_string(),
                            ),
                            "cond" => lines.push(
                                include_str!("../compile_c_includes/builtins/cond.c").to_string(),
                            ),
                            "to_float" => lines.push(
                                include_str!("../compile_c_includes/builtins/to_float.c")
                                    .to_string(),
                            ),
                            "to_int" => lines.push(
                                include_str!("../compile_c_includes/builtins/to_int.c").to_string(),
                            ),
                            "make-stack" => lines.push(
                                include_str!("../compile_c_includes/builtins/stack-new.c")
                                    .to_string(),
                            ),
                            "stack-ref" => lines.push(
                                include_str!("../compile_c_includes/builtins/stack-ref.c")
                                    .to_string(),
                            ),
                            "stack-set!" => lines.push(
                                include_str!("../compile_c_includes/builtins/stack-set.c")
                                    .to_string(),
                            ),
                            "stack-push!" => lines.push(
                                include_str!("../compile_c_includes/builtins/stack-push.c")
                                    .to_string(),
                            ),
                            "stack-pop!" => lines.push(
                                include_str!("../compile_c_includes/builtins/stack-pop.c")
                                    .to_string(),
                            ),
                            "stack-size" => lines.push(
                                include_str!("../compile_c_includes/builtins/stack-size.c")
                                    .to_string(),
                            ),

                            // Attempt to lookup in names table
                            id => {
                                let id = sanitize_name(id);
                                lines.push(format!(
                                    "
        {{
            Value* v = names_lookup(names, NAME_{id});
            if (v->type == TAG_BLOCK) {{
                void *f = v->as_block;
                ((void (*)(Name*))f)(names);
            }} else {{
                *(++stack_ptr) = *v;
            }}
        }}
                    "
                                ));
                            }
                        }
                    }
                    Expression::DottedIdentifier(ids) => {
                        unimplemented!("compile_expr for dotted identifiers: {:?}", ids)
                    }
                    Expression::Literal(value) => {
                        let (tag, field, value) = match value {
                            // TODO: additional numeric tyhpes
                            Value::Number(Number::Integer(v)) => {
                                ("TAG_NUMBER_INTEGER", "integer", v.to_string())
                            }
                            Value::Number(Number::Rational { .. }) => {
                                unimplemented!()
                            }
                            Value::Number(Number::Float(v)) => {
                                ("TAG_NUMBER_FLOAT", "float", v.to_string())
                            }
                            Value::Number(Number::Complex { .. }) => {
                                unimplemented!()
                            }
                            Value::String(v) => ("TAG_STRING", "string", format!("{v:?}")),
                            Value::Boolean(v) => ("TAG_BOOLEAN", "boolean", format!("{v:?}")),
                            Value::Block { .. } => panic!("Blocks should be compiled separately"),
                            Value::Stack(_) => unimplemented!(),
                            Value::Hash(_) => unimplemented!(),
                            Value::IntHash(_) => unimplemented!(),
                        };

                        lines.push(format!(
                            "
        {{
            Value v = {{.type={tag}, .as_{field}={value}}};
            *(++stack_ptr) = v;
        }}
    "
                        ));
                    }
                    Expression::Block(ref body) => {
                        let arity = calculate_arity(&expr).expect(
                            format!("Unable to calculate arity for block: {:?}", expr).as_str(),
                        );
                        let index = compile_block(arity, body, blocks);
                        lines.push(format!(
                            "
        {{
            Value v = {{.type=TAG_BLOCK, .as_block=(void*)block_{index}}};
            *(++stack_ptr) = v;
        }}
    "
                        ));
                    }
                    Expression::List(values) => {
                        lines.push("\t{{".to_string());
                        lines.push(
                            "\t\tValue s = {.type=TAG_STACK, .as_stack=vs_init()};".to_string(),
                        );
                        for value in values {
                            for line in compile_expr(value.clone(), blocks) {
                                lines.push(line);
                            }
                            lines.push(
                                format!(
                                    "\t\tvs_push(s.as_stack, *(stack_ptr--)); // Push {}",
                                    value
                                )
                                .to_string(),
                            );
                        }
                        lines.push("\t\t*(++stack_ptr) = s;".to_string());
                        lines.push("\n\t}}".to_string());
                    }
                    Expression::Group(exprs) => {
                        for expr in exprs {
                            for line in compile_expr(expr, blocks) {
                                lines.push(line);
                            }
                        }
                    }
                    Expression::At(expr) => {
                        match expr.as_ref() {
                            Expression::Identifier(id) => {
                                let id = sanitize_name(id);
                                lines.push(format!(
                                    "
        {{
            Value *p = stack_ptr;
            names = names_bind(names, NAME_{id}, p);
        }}
    "
                                ));
                            }
                            Expression::List(id_exprs) => {
                                let id_count = id_exprs.len();
                                for (i, id_expr) in id_exprs.iter().enumerate() {
                                    match id_expr {
                                        Expression::Identifier(id) => {
                                            let id = sanitize_name(id);

                                            lines.push(format!(
                                                "
        {{ 
            Value *p = (stack_ptr - {id_count} + {i} + 1);
            names = names_bind(names, NAME_{id}, p);
        }}
    "
                                            ));
                                        }
                                        _ => panic!(
                                            "Unexpected @ expression when compiling: {}",
                                            expr
                                        ),
                                    }
                                }
                            }
                            Expression::Literal(Value::Number(Number::Integer(_))) => {} // ignore numeric @ expressions
                            _ => panic!("Unexpected @ expression when compiling: {}", expr),
                        }
                    }
                    Expression::Bang(ref v) => {
                        match v.as_ref() {
                            Expression::Literal(Value::Number(Number::Integer(_))) => {} // Used only for arity out expressions
                            Expression::Identifier(id) => {
                                let id = sanitize_name(id);
                                lines.push(format!(
                                    "
        {{ 
            Value *v = stack_ptr--;
            names_update(names, NAME_{id}, v);
        }}
    "
                                ));
                            }
                            _ => panic!("Unexpected ! expression when compiling: {}", expr),
                        }
                    }
                    Expression::Dollar(expr) => match expr.as_ref() {
                        Expression::Identifier(id) => {
                            lines.push(format!(
                                "
        {{
            Value* v = names_lookup(names, NAME_{id});
            *(++stack_ptr) = *v;
        }}
            "
                            ));
                        }
                        _ => panic!("Unexpected $ expression when compiling: {}", expr),
                    },
                }

                lines
            }

            // Compile the top level expression
            let mut blocks = vec![];
            match ast {
                Expression::Group(body) => {
                    compile_block((0, 0), &body, &mut blocks);
                }
                _ => panic!("Unexpected top level expression: {:?}", ast),
            }

            blocks
        };

        let mut block_block = String::new();

        // Forward declarations
        {
            for (i, _) in blocks.iter().enumerate() {
                block_block.push_str(&format!("void block_{i}(Name *block_names);\n").to_string());
            }
        }

        // Generate block functions
        {
            for (i, block) in blocks.iter().enumerate() {
                // Push the comment for the block
                // TODO: This is a bit ugly, what if I don't generate the comment any more?
                block_block.push('\n');
                block_block.push_str(block.first().unwrap().trim());

                block_block.push_str(&format!(
                    "
void block_{i}(Name *block_names) {{
    if (block_names != NULL) block_names->boundary = true;
    Name* names = block_names;

    #ifdef DEBUG
    fprintf(stderr, \"[DEBUG] block_{i} called --\");
    stack_dump(names);
    #endif
"
                ));

                // Push the block body, skip the leading comment
                // TODO: Again, this assumes that we pushed a comment to the block
                for line in block.iter().skip(1) {
                    block_block.push_str(line);
                    block_block.push('\n');
                }

                block_block.push_str(&format!(
                    "
    #ifdef DEBUG
    fprintf(stderr, \"[DEBUG] block_{i} returning --\");
    stack_dump(names);
    #endif

    // Free names bound in this block
    while (names != NULL && block_names != names) {{
        Name *next = names->prev;
        free(names);
        names = next;
    }}
}}
"
                ));
            }
        }

        template = template.replace("/*{BLOCKS}*/", &block_block);
    }

    template
}
