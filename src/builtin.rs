use crate::Value;

pub(crate) fn builtin(evaluated: &mut Vec<Value>, sym: String) -> Value {
    match sym.as_str() {
        "list" => builtin_list(evaluated),
        "head" => builtin_head(evaluated),
        "tail" => builtin_tail(evaluated),
        "join" => builtin_join(evaluated),
        "eval" => builtin_eval(evaluated),
        "+" | "-" | "/" | "*" => builtin_op(evaluated, sym.as_str()),
        _ => Value::Err("Unknown function".to_string()),
    }
}

fn builtin_list(evaluated: &mut Vec<Value>) -> Value {
    Value::Qexpr(evaluated.clone())
}

fn builtin_head(evaluated: &mut Vec<Value>) -> Value {
    let children = match &evaluated[0] {
        Value::Qexpr(children) => children,
        _ => return Value::Err("Function 'head' passed incorrect type!".to_string()),
    };
    if children.is_empty() {
        return Value::Err("Function 'head' passed {}".to_string());
    }
    evaluated.pop().unwrap()
}

fn builtin_tail(evaluated: &mut Vec<Value>) -> Value {
    let children = match &evaluated[0] {
        Value::Qexpr(children) => children,
        _ => return Value::Err("Function 'tail' passed incorrect type!".to_string()),
    };
    if children.is_empty() {
        return Value::Err("Function 'tail' passed {}".to_string());
    }
    let fst = evaluated.pop().unwrap();
    todo!()
}

fn builtin_join(evaluated: &mut Vec<Value>) -> Value {
    todo!()
}

fn builtin_eval(evaluated: &mut Vec<Value>) -> Value {
    todo!()
}

fn builtin_op(evaluated: &mut Vec<Value>, as_str: &str) -> ! {
    todo!()
}
