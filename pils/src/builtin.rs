use anyhow::{Context, Ok};

use crate::{
    environment::Environment, function::Function, qexpr::Qexpr, sexpr::Sexpr, value::Value,
};

pub fn list(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    match val {
        Value::Sexpr(s) => Ok(Value::Qexpr(Qexpr(s.0))),
        Value::Qexpr(q) => Ok(Value::Qexpr(q)),
        _ => Err(anyhow::anyhow!("Wrong type passed to 'list'")),
    }
}

pub fn head(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    match val {
        Value::Sexpr(Sexpr(q)) => {
            if let Some(Value::Qexpr(Qexpr(q))) = q.get(0) {
                let value = Qexpr(q.clone()).head()?;
                Ok(value)
            } else {
                Qexpr(q).head()
            }
        }
        Value::Qexpr(q) => q.head(),
        _ => Err(anyhow::anyhow!("Wrong type passed to 'head'")),
    }
}

pub fn tail(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    match val {
        Value::Sexpr(Sexpr(q)) => {
            if let Some(Value::Qexpr(Qexpr(q))) = q.get(0) {
                let value = Qexpr(q.clone()).tail()?;
                Ok(value)
            } else {
                Qexpr(q).tail()
            }
        }
        Value::Qexpr(q) => q.tail(),
        _ => Err(anyhow::anyhow!("Wrong type passed to 'tail'")),
    }
}

pub fn join(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    match val {
        Value::Sexpr(Sexpr(q)) => {
            if let Some(Value::Qexpr(Qexpr(q))) = q.get(0) {
                let value = Qexpr(q.clone()).join()?;
                Ok(value)
            } else {
                Qexpr(q).join()
            }
        }
        Value::Qexpr(q) => q.join(),
        _ => Err(anyhow::anyhow!("Wrong type passed to 'join'")),
    }
}

pub fn eval(val: Value, env: &mut Environment) -> Result<Value, anyhow::Error> {
    match val {
        Value::Sexpr(Sexpr(q)) => {
            if let Some(Value::Qexpr(Qexpr(q))) = q.get(0) {
                let value = Qexpr(q.clone()).eval(env)?;
                Ok(value)
            } else {
                Qexpr(q).eval(env)
            }
        }
        Value::Qexpr(Qexpr(q)) => Sexpr(q).eval(env),
        _ => Err(anyhow::anyhow!("Wrong type passed to 'eval'")),
    }
}

pub fn def(val: Value, env: &mut Environment) -> Result<Value, anyhow::Error> {
    let Value::Sexpr(Sexpr(mut s)) = val else {
        return Err(anyhow::anyhow!("non-qexpr passed to def"));
    };
    let syms = s.pop_front().context("'def' called without arguments")?;
    let Value::Qexpr(q) = syms else {
        return Err(anyhow::anyhow!("Function 'def' passed incorrect type"));
    };
    let syms =
        q.0.iter()
            .cloned()
            .map(|sym| match sym {
                Value::Sym(s) => Ok(s),
                _ => Err(anyhow::anyhow!("'def' cannot define non-symbol")),
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
    for (sym, val) in syms.iter().zip(s.into_iter()) {
        env.0.insert(sym.clone(), val);
    }
    Ok(Value::Fun(Function(crate::builtin::eval)))
}

pub fn add(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    let Value::Sexpr(s) = val else {
        return Err(anyhow::anyhow!("non-sexpr passed to 'add'"));
    };
    s.add()
}

pub fn sub(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    let Value::Sexpr(s) = val else {
        return Err(anyhow::anyhow!("non-sexpr passed to 'sub'"));
    };
    s.sub()
}

pub fn mul(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    let Value::Sexpr(s) = val else {
        return Err(anyhow::anyhow!("non-sexpr passed to 'mul'"));
    };
    s.mul()
}

pub fn div(val: Value, _env: &mut Environment) -> Result<Value, anyhow::Error> {
    let Value::Sexpr(s) = val else {
        return Err(anyhow::anyhow!("non-sexpr passed to 'div'"));
    };
    s.div()
}
