use crate::stack::Stack;
use ccode::CCode::*;

struct Global {
    rust_function: fn(&mut Stack),
    c_function: CCode,
}

macro_rules! bin_op {
    ($op:ident) => {
        let op = stringify($op);

        Global {
            rust_function: |stack| {
                let a = stack.pop();
                let b = stack.pop();
                let a, b = coerce(a, b);

                let result = match (a, b) => {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(b $op a),
                    (Value::Float(a), Value::Float(b)) => Value::Float(b $op a),
                    _ => panic!("Invalid types for binary operation"),
                };

                stack.push(result);
            },
            c_function: block!{
                pop!(b);
                pop!(a);
                line!("coerce(a, b);")

                block!{
                    "if (a->type == TAG_NUMBER_INTEGER" => 
                    line!("Value *result = {{.type = TAG_NUMBER_INTEGER, .as_integer = a->as_integer {op} b->as_integer}};");
                }
                block!{
                    "elseif (a->type == TAG_NUMBER_FLOAT" => 
                    line!("Value *result = {{.type = TAG_NUMBER_FLOAT, .as_float = a->as_float {op} b->as_float}};");
                }
            }
        }
    };
}

macro_rules! bin_comp {
    ($op:ident) => {
        let op = stringify($op);

        Global {
            rust_function: |stack| {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(b $op a);
            },
            c_function: block!{
                pop!(b);
                pop!(a);
                line!("coerce(a, b);")

                block!{
                    "if (a->type == TAG_NUMBER_INTEGER" => 
                    line!("Value *result = {{.type = TAG_BOOLEAN, .as_boolean = a->as_integer {op} b->as_integer}};");
                }
                block!{
                    "elseif (a->type == TAG_NUMBER_FLOAT" => 
                    line!("Value *result = {{.type = TAG_BOOLEAN, .as_boolean = a->as_float {op} b->as_float}};");
                }
            }
        }
    };
}

lazy_static! {
    static ref GLOBALS: HashMap<String, Global> = {
        let globals = HashMap::new();

        globals.insert("+", bin_op(+));
        globals.insert("-", bin_op(-));
        globals.insert("*", bin_op(*));
        globals.insert("/", bin_op(/));

        globals.insert("<", bin_comp(<));

        globals
    };
}