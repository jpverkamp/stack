use std::collections::HashMap;

use crate::arity::calculate_arity;
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
            Value result = {{.type=TAG_NUMBER_FLOAT, .as_integer=a->as_integer {op} b->as_integer}};
            *(++stack_ptr) = result;
        }}
    }}
"));
    }};
}

fn collect_names(ast: &Expression) -> HashMap<String, usize> {
    let mut names = HashMap::new();

    fn collect_names_expr(expr: &Expression, names: &mut HashMap<String, usize>) {
        match expr {
            Expression::Identifier(_) | Expression::Literal(_) | Expression::Bang(_) | Expression::Dollar(_) => {
                // Do nothing, no names possible
            },
            Expression::List(_) => todo!(),
            Expression::Block(exprs) => {
                for expr in exprs {
                    collect_names_expr(expr, names);
                }
            },
            Expression::Group(exprs) => {
                for expr in exprs {
                    collect_names_expr(expr, names);
                }
            },
            Expression::At(expr) => {
                match expr.as_ref() {
                    Expression::Identifier(id) => {
                        if !names.contains_key(id) {
                            log::debug!("Adding name: {} @ {}", id, names.len());
                            names.insert(id.clone(), names.len());
                        }
                    },
                    Expression::List(id_exprs) => {
                        for id_expr in id_exprs {
                            match id_expr {
                                Expression::Identifier(id) => {
                                    if !names.contains_key(id) {
                                        log::debug!("Adding name: {} @ {}", id, names.len());
                                        names.insert(id.clone(), names.len());
                                    }
                                },
                                _ => panic!("Unexpected @ expression when collecting names: {}", expr),
                            }
                        }
                    },
                    Expression::Literal(Value::Number(Number::Integer(_))) => {}, // ignore numeric @ expressions
                    _ => panic!("Unexpected @ expression when collecting names: {}", expr),
                }
            },
        }
    }

    collect_names_expr(ast, &mut names);
    names
}

pub fn compile(ast: Expression) -> String {
    let mut lines = vec![];
    lines.push(include_str!("../compile_c_includes/header.c").to_string());

    let names = collect_names(&ast);
    log::debug!("collected names: {:?}", names);

    for (name, index) in names {
        lines.push(format!("#define NAME_{: <15} {}", name, index));
    }

    fn compile_expr(expr: Expression, block_count: &mut usize) -> Vec<String> {
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

                    // Built ins
                    "writeln" => lines.push("
{
    Value v = *(stack_ptr--);
    if (v.type == TAG_NUMBER_INTEGER) {
        printf(\"%lld\\n\", v.as_integer);
    } else if (v.type == TAG_NUMBER_FLOAT) {
        printf(\"%f\\n\", v.as_float);
    } else if (v.type == TAG_STRING) {
        printf(\"%s\\n\", v.as_string);
    } else if (v.type == TAG_BOOLEAN) {
        printf(\"%s\\n\", v.as_boolean ? \"true\" : \"false\");
    } else {
        // TODO: Error
    }
}
".to_string()),
                    // Attempt to lookup in names table
                    id => {
                        lines.push(format!("
    {{
        Value* v = lookup(stack, stack_ptr, NAME_{id});
        if (v->type == TAG_BLOCK) {{
            // TODO
        }} else {{
            *(++stack_ptr) = *v;
        }}
    }}
                        "));
                    }

                    // Unknown identifier
                    _ => panic!("Unknown identifier: {}", id),
                    
                }
            }
            Expression::Literal(value) => {
                let (tag, field, value) = match value {
                    // TODO: additional numeric tyhpes
                    Value::Number(Number::Integer(v)) => {
                        ("TAG_NUMBER_INTEGER", "integer", v.to_string())
                    }
                    Value::Number(Number::Float(v)) => ("TAG_NUMBER_FLOAT", "float", v.to_string()),
                    Value::String(_v) => todo!(),
                    Value::Boolean(_v) => todo!(),
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
            Expression::Block(ref block) => {
                *block_count += 1;

                // TODO: Better panic
                let (arity_in, arity_out) = calculate_arity(&expr).expect(format!("Unable to calculate arity for block: {:?}", expr).as_str());

                lines.push(format!("goto skip_block_{block_count};"));
                lines.push(format!("block_{block_count}:"));
                lines.push(format!("    {{"));

                lines.push(format!("    *(++frame_ptr) = (stack_ptr - {arity_in});"));
                
                // Compile the block itself
                for expr in block {
                    for line in compile_expr(expr.clone(), block_count) {
                        lines.push(line);
                    }
                }

                // Pop the block off the stack
                lines.push(format!("    Value* return_ptr = (stack_ptr - {arity_out});"));
                lines.push(format!("    stack_ptr =  *(frame_ptr--);"));
                for _ in 0..arity_out {
                    lines.push(format!("    *(stack_ptr++) = *(return_ptr++);"));
                }

                lines.push(format!("    }}"));
                lines.push(format!("skip_block_{block_count}:"));
            }
            Expression::List(_) => todo!(),
            Expression::Group(exprs) => {
                for expr in exprs {
                    for line in compile_expr(expr, block_count) {
                        lines.push(line);
                    }
                }
            }
            Expression::At(expr) => {
                match expr.as_ref() {
                    Expression::Identifier(id) => {
                        lines.push(format!("
    {{
        Value *v = stack_ptr;
        if (v->name_count == STACKED_NAME_MAX) {{
            printf(\"Too many names in @ expression\");
            exit(1);
        }}
        v->names[v->name_count++] = NAME_{id};
    }}
"
                        ));
                    }
                    Expression::List(id_exprs) => {
                        let id_count = id_exprs.len();
                        for (i, id_expr) in id_exprs.iter().enumerate() {
                            match id_expr {
                                Expression::Identifier(id) => {
                                    lines.push(format!("
    {{
        Value *v = (stack_ptr - {id_count} + {i});
        if (v->name_count == STACKED_NAME_MAX) {{
            printf(\"Too many names in @ expression\");
            exit(1);
        }}
        v->names[v->name_count++] = NAME_{id};
    }}
"
                                    ));
                                },
                                _ => panic!("Unexpected @ expression when compiling: {}", expr),
                            }
                        }
                    },
                    Expression::Literal(Value::Number(Number::Integer(_))) => {}, // ignore numeric @ expressions
                    _ => panic!("Unexpected @ expression when compiling: {}", expr),
                }
            },
            Expression::Bang(_) => {
                todo!()
            },
            Expression::Dollar(expr) => {
                todo!()
            },
        }

        lines
    }

    lines.push(include_str!("../compile_c_includes/main_start.c").to_string());
    let mut block_count = 0;
    for line in compile_expr(ast, &mut block_count) {
        lines.push(line);
    }
    lines.push(include_str!("../compile_c_includes/main_end.c").to_string());

    lines.join("\n")
}
