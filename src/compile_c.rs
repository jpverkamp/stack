use crate::types::{Value, Expression};

/// A helper macro to generate functions that operate on two integers and floats
macro_rules! numeric_binop {
    ($lines:expr, $op:literal) => {{
        let ty_integer = Value::Integer as u8;
        let ty_float = Value::Float as u8;
        let op = stringify!($op).to_string().trim_matches('"').to_string();

        $lines.push(format!("
    {{
        Value b = *stack_ptr--;
        Value a = *stack_ptr--;

        if (a.type == {ty_integer} && b.type == {ty_integer}) {{
            Value v = {{.type={ty_integer}, .as_integer=a.as_integer {op} b.as_integer}};
            *(++stack_ptr) = v;
        }} else if (a.type == {ty_integer} && b.type == {ty_float}) {{
            Value v = {{.type={ty_float}, .as_float=(double)a.as_integer {op} b.as_float}};
            *(++stack_ptr) = v;
        }} else if (a.type == {ty_float} && b.type == {ty_integer}) {{
            Value v = {{.type={ty_float}, .as_float=a.as_float {op} (double)b.as_integer}};
            *(++stack_ptr) = v;
        }} else if (a.type == {ty_float} && b.type == {ty_float}) {{
            Value v = {{.type={ty_float}, .as_float=a.as_float {op} b.as_float}};
            *(++stack_ptr) = v;
        }}
    }}
"));
    }};
}

pub fn compile(ast: Expression) -> String {
    let mut lines = vec![];
    lines.push("
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t type;
    union {
        int64_t as_integer;
        double as_float;
        char *as_string;
        uint8_t as_boolean;
        uint8_t as_block;
    };
} Value;

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
                        let ty_integer = Value::Integer as u8;
                        let ty_float = Value::Integer as u8;
                        let ty_string = Value::Integer as u8;
                        let ty_boolean = Value::Integer as u8;

                        lines.push(format!("
    {{
        Value v = *(stack_ptr--);
        if (v.type == {ty_integer}) {{
            printf(\"%lld\", v.as_integer);
        }} else if (v.type == {ty_float}) {{
            printf(\"%f\", v.as_float);
        }} else if (v.type == {ty_string}) {{
            printf(\"%s\", v.as_string);
        }} else if (v.type == {ty_boolean}) {{
            printf(\"%s\", v.as_boolean ? \"true\" : \"false\");
        }} else {{
            // TODO: Error
        }}
    }}
"));
                    }

                    // Unknown identifier
                    // _ => panic!("Unknown identifier: {}", id),
                    _ => {},
                }
                
            },
            Expression::Literal(value) => {
                let (tag, ty, value) = match value {
                    Value::Null => todo!(),
                    Value::Integer(v) => (
                        Value::Integer as u8,
                        "integer",
                        v.to_string()
                    ),
                    Value::Float(v) => (
                        Value::Float as u8,
                        "float",
                        v.to_string()
                    ),
                    Value::String(_v) => todo!(),
                    Value::Boolean(_v) => todo!(),
                    Value::Block { .. } => todo!(),
                };

                lines.push(format!("
    {{
        Value v = {{.type={tag}, .as_{ty}={value}}};
        *(++stack_ptr) = v;
    }}
"));
            },
            Expression::Block(id, children) => {
                lines.push(format!("
{id}:
"));
                for child in children {
                    for line in compile_expr(child) {
                        lines.push(line);
                    }
                }
            },
            Expression::List(_) => todo!(),
            Expression::Group(exprs) => {
                for expr in exprs {
                    for line in compile_expr(expr) {
                        lines.push(line);
                    }
                }
            },
            Expression::At(value) => {
                let ty_integer = Value::Integer as u8;
                let ty_float = Value::Float as u8;
                let ty_string = Value::String as u8;
                let ty_boolean = Value::Boolean as u8;

                lines.push(format!("
    {{ 
        Value v = {{.type={ty_integer}, .as_integer={value}}};
        *(++stack_ptr) = v;
    }}
"));
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