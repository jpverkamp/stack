use std::collections::HashMap;

use regex::Regex;
use sha2::{Digest, Sha256};

use crate::arity::calculate_arity;
use crate::debug;
use crate::numbers::Number;
use crate::types::{Expression, Value};

/// A helper macro to generate functions that operate on two integers and floats
macro_rules! numeric_binop {
    ($lines:expr, $op:literal) => {{
        let op = stringify!($op).to_string().trim_matches('"').to_string();

        $lines.push(format!("
    {{
        Value *b = stack_ptr--;
        Value *a = stack_ptr--;
        coerce(a, b);
        
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
            | Expression::Literal(_)
            | Expression::Bang(_)
            | Expression::Dollar(_) => {
                // Do nothing, no names possible
            }
            Expression::List(_) => todo!(),
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
    let mut lines = vec![];
    lines.push(include_str!("../compile_c_includes/header.c").to_string());

    let names = collect_names(&ast);
    log::debug!("collected names: {:?}", names);

    for (name, index) in names.iter() {
        lines.push(format!("#define NAME_{name} {index}"));
    }

    unsafe {
        if debug::ENABLED {
            lines.push("char* get_name(int index) {".to_string());
            for (name, index) in names.iter() {
                lines.push(format!("    if (index == {index}) {{ return \"{name}\"; }}"));
            }
            lines.push("    return \"<unknown>\";".to_string());
            lines.push("}".to_string());
        }
    }

    lines.push(include_str!("../compile_c_includes/types.c").to_string());
    lines.push(include_str!("../compile_c_includes/globals.c").to_string());
    lines.push(include_str!("../compile_c_includes/coerce.c").to_string());
    lines.push(include_str!("../compile_c_includes/names.c").to_string());

    unsafe {
        if debug::ENABLED {
            lines.push(include_str!("../compile_c_includes/stack_dump.c").to_string()); // DEBUG
        }
    }

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
        lines.push(format!("\n    // Store the current stack pointer with arity_in={arity_in}"));
        lines.push(format!("    *(++frame_ptr) = (stack_ptr - {arity_in});\n"));

        // Compile the block itself
        for expr in body {
            for line in compile_expr(expr.clone(), blocks) {
                lines.push(line);
            }
        }

        // Pop the block off the stack
        lines.push(format!("    // Pop the block off the stack, preserving arity_out={arity_out} values"));
        lines.push(format!("    Value* return_ptr = (stack_ptr - {arity_out});"));
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
                    "%" => numeric_binop!(lines, "%"),

                    // Built in numeric comparisons
                    "<" => numeric_compare!(lines, "<"),
                    "<=" => numeric_compare!(lines, "<="),
                    "=" => numeric_compare!(lines, "=="),
                    "!=" => numeric_compare!(lines, "!="),
                    ">=" => numeric_compare!(lines, ">="),
                    ">" => numeric_compare!(lines, ">"),

                    // Built ins
                    "write" => lines.push(include_str!("../compile_c_includes/builtins/write.c").to_string()),
                    "writeln" => {
                        lines.push(include_str!("../compile_c_includes/builtins/write.c").to_string());
                        lines.push("printf(\"\\n\");".to_string());
                    },
                    "newline" => lines.push("printf(\"\\n\");".to_string()),
                    "loop" => lines.push(include_str!("../compile_c_includes/builtins/loop.c").to_string()),
                    "if" => lines.push(include_str!("../compile_c_includes/builtins/if.c").to_string()),
                    "to_float" => lines.push(include_str!("../compile_c_includes/builtins/to_float.c").to_string()),
                    "to_int" => lines.push(include_str!("../compile_c_includes/builtins/to_int.c").to_string()),

                    // Attempt to lookup in names table
                    id => {
                        let id = sanitize_name(id);
                        lines.push(format!(
                            "
    {{
        Value* v = lookup(names, NAME_{id});
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
            Expression::Literal(value) => {
                let (tag, field, value) = match value {
                    // TODO: additional numeric tyhpes
                    Value::Number(Number::Integer(v)) => {
                        ("TAG_NUMBER_INTEGER", "integer", v.to_string())
                    }
                    Value::Number(Number::Float(v)) => ("TAG_NUMBER_FLOAT", "float", v.to_string()),
                    Value::String(v) => ("TAG_STRING", "string", format!("{v:?}")),
                    Value::Boolean(v) => ("TAG_BOOLEAN", "boolean", format!("{v:?}")),
                    Value::Block { .. } => todo!(),
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
                let arity = calculate_arity(&expr)
                    .expect(format!("Unable to calculate arity for block: {:?}", expr).as_str());
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
            Expression::List(_) => todo!(),
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
        Value *v = stack_ptr;
        names = bind(names, NAME_{id}, v);
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
        Value *v = (stack_ptr - {id_count} + {i} + 1);
        names = bind(names, NAME_{id}, v);
    }}
"
                                    ));
                                }
                                _ => panic!("Unexpected @ expression when compiling: {}", expr),
                            }
                        }
                    }
                    Expression::Literal(Value::Number(Number::Integer(_))) => {} // ignore numeric @ expressions
                    _ => panic!("Unexpected @ expression when compiling: {}", expr),
                }
            }
            Expression::Bang(v) => {
                match v.as_ref() {
                    Expression::Literal(Value::Number(Number::Integer(_))) => {}, // Used only for arity out expressions
                    _ => todo!(),
                }
            }
            Expression::Dollar(expr) => match expr.as_ref() {
                Expression::Identifier(id) => {
                    lines.push(format!(
                        "
    {{
        Value* v = lookup(names, NAME_{id});
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

    let mut blocks = vec![];

    match ast {
        Expression::Group(body) => {
            compile_block((0, 0), &body, &mut blocks);
        }
        _ => panic!("Unexpected top level expression: {:?}", ast),
    }

    // Forward declare all blocks
    lines.push("\n// Forward declare all blocks".to_string());
    for (i, _) in blocks.iter().enumerate() {
        lines.push(format!("void block_{i}(Name *block_names);", i = i).to_string());
    }

    // Generate block functions
    lines.push("\n// Actual block definitions".to_string());
    for (i, block) in blocks.iter().enumerate() {
        lines.push(format!("void block_{i}(Name *block_names) {{").to_string());
        lines.push(format!("    if (block_names != NULL) block_names->boundary = true;").to_string());
        lines.push(format!("    Name* names = block_names;").to_string());
        
        unsafe {
            if debug::ENABLED {
                lines.push(format!("    printf(\"[DEBUG] block_{i} called --\");").to_string()); // DEBUG
                lines.push(format!("    stack_dump(names);\n")); // DEBUG
            }
        }

        lines.push(block.join("\n"));

        unsafe {
            if debug::ENABLED {
                lines.push(format!("    printf(\"[DEBUG] block_{i} return --\");").to_string()); // DEBUG
                lines.push(format!("    stack_dump(names);\n")); // DEBUG
            }
        }

        lines.push("    // Free names bound in this block".to_string());
        lines.push("    
    while (names != NULL && block_names != names) {
        Name *next = names->prev;
        free(names);
        names = next;
    }
        ".to_string());
        lines.push("}".to_string());
    }

    lines.push(include_str!("../compile_c_includes/main.c").to_string());

    lines.join("\n")
}
