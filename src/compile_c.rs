use crate::numbers::Number;
use crate::types::{Value, Expression};

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

pub fn compile(ast: Expression) -> String {
    let mut lines = vec![];
    lines.push("
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define TAG_NUMBER          0
#define TAG_NUMBER_INTEGER  1
#define TAG_NUMBER_RATIONAL 2
#define TAG_NUMBER_FLOAT    3
#define TAG_NUMBER_COMPLEX  4

#define TAG_STRING          16
#define TAG_BOOLEAN         17
#define TAG_BLOCK           18

typedef struct {
    uint8_t type;
    union {
        int64_t as_integer;
        double as_float;

        char *as_string;
        bool as_boolean;
        uint8_t as_block;
    };
} Value;

void coerce(Value *a, Value *b) {
    if (a->type == b->type) {
        return;
    }

    if (a->type == TAG_NUMBER_INTEGER && b->type == TAG_NUMBER_FLOAT) {
        a->type = TAG_NUMBER_FLOAT;
        a->as_float = (double)a->as_integer;
    }

    if (a->type == TAG_NUMBER_FLOAT && b->type == TAG_NUMBER_INTEGER) {
        b->type = TAG_NUMBER_FLOAT;
        b->as_float = (double)a->as_integer;
    }
}

int main(int argc, char *argv[]) {
    // The stack holding all values
    Value* stack = malloc(1024 * sizeof(Value));
    Value* stack_ptr = stack;

    // Stack frames holding block scopes
    Value** frames = malloc(1024 * sizeof(Value*));
    Value** frame_ptr = frames;

".to_string());

    fn compile_expr(expr: Expression) -> Vec<String> {
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
                    "writeln" => {
                        lines.push("
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
".to_string());
                    }

                    // Unknown identifier
                    // _ => panic!("Unknown identifier: {}", id),
                    _ => {},
                }
                
            },
            Expression::Literal(value) => {
                let (tag, field, value) = match value {
                    // TODO: additional numeric tyhpes
                    Value::Number(Number::Integer(v)) => (
                        "TAG_NUMBER_INTEGER",
                        "integer",
                        v.to_string()
                    ),
                    Value::Number(Number::Float(v)) => (
                        "TAG_NUMBER_FLOAT",
                        "float",
                        v.to_string()
                    ),
                    Value::String(_v) => todo!(),
                    Value::Boolean(_v) => todo!(),
                    Value::Block { .. } => todo!(),
                };

                lines.push(format!("
    {{
        Value v = {{.type={tag}, .as_{field}={value}}};
        *(++stack_ptr) = v;
    }}
"));
            },
            Expression::Block(_) => {
                todo!();
            },
            Expression::List(_) => todo!(),
            Expression::Group(exprs) => {
                for expr in exprs {
                    for line in compile_expr(expr) {
                        lines.push(line);
                    }
                }
            },
            Expression::At(_) => {
                todo!();
            },
            Expression::Bang(_) => todo!(),
            Expression::Dollar(_) => todo!(),
        }

        lines
    }

    for line in compile_expr(ast) {
        lines.push(line);
    }

    lines.push("

    return 0;
}
".to_string());

    lines.join("\n")
}