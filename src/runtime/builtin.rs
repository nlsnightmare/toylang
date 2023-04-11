use super::Value;

pub fn is_builtin(name: &str) -> bool {
    let bultitin = ["print", "add", "subtract", "print_version", "subtract", "len"];

    bultitin.contains(&name)
}

pub fn execute_builtin(name: &str, args: Vec<Value>) -> Value {
    match name {
        "len" => match &args[0] {
            Value::Array(a) => Value::Number(a.length as f64),
            Value::String(ref s) => Value::Number(s.len() as f64),
            a => panic!("object {:?} doen't have a length", a),
        },
        "print" => {
            if let Some(v) = args.first() {
                println!("{}", v.to_string());
            }

            Value::Void
        }
        "add" => match &args[0..2] {
            [Value::String(s1), Value::String(s2)] => Value::String(format!("{}{}", s1, s2)),
            [Value::String(s), Value::Number(n)] => Value::String(format!("{}{}", s, n)),
            [Value::Number(n1), Value::Number(n2)] => Value::Number(n1 + n2),

            // TODO: make runtime error stuff
            _ => panic!("Unable to add {:?} and {:?}", args[0], args[1]),
        },
        "subtract" => match &args[0..2] {
            [Value::Number(n1), Value::Number(n2)] => Value::Number(n1 - n2),
            _ => Value::Void,
        },

        "print_version" => {
            println!("mylang version 0.0.1");
            Value::Void
        }
        _ => panic!(
            "Attempting to execute unimplemented builtin function {:?}",
            name
        ),
    }
}
