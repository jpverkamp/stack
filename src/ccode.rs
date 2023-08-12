
pub enum CCode {
    Block {
        prefix: Option<String>, 
        body: Vec<CCode>
    },
    Line(String),
}

macro_rules! block {
    ($prefix:expr => $($body:tt)*) => {
        CCode::Block {
            prefix: Some($prefix.to_string()),
            body: vec![$($body)*]
        }
    };
    ($($body:tt)*) => {
        CCode::Block {
            prefix: None,
            body: vec![$($body)*]
        }
    };
}

macro_rules! line {
    ($($body:tt)*) => {
        CCode::Line(format!($($body)*))
    };
}

macro_rules! push {
    ($id:ident) => {
        let id = stringify!($id);
        line!("*(++stack_ptr) = {id};")
    };
}

macro_rules! pop {
    ($body:ident) => {
        line!("Value *{id} = stack_ptr--;")
    };
}

impl CCode {
    pub fn to_string(&self, depth: usize) -> String {
        let indent = "\t".repeat(depth);
        let mut result = String::new();

        match self {
            CCode::Block { prefix, body  } => {
                result.push_str(&indent);
                if let Some(prefix) = prefix {
                    result.push_str(prefix);
                    result.push_str(" ");
                }
                result.push_str("{\n");
                for line in body {
                    result.push_str(&line.to_string(depth + 1));
                }
                result.push_str(&indent);
                result.push_str("}\n");
            },
            CCode::Line(line) => {
                result.push_str(&indent);
                result.push_str(line);
                result.push_str("\n");
            }
        };

        result
    }
}