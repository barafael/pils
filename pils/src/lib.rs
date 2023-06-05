use crate::value::Value;
use anyhow::{anyhow, Context, Error};
use environment::Environment;
use once_cell::sync::Lazy;
use parser::{Pils, Rule};
use pest::Parser;
use std::{collections::HashMap, sync::Mutex};

pub mod builtin;
pub mod environment;
mod function;
pub mod help;
mod parser;
mod qexpr;
mod sexpr;
mod value;

#[cfg(test)]
mod test;

static ENVIRONMENT: Lazy<Mutex<Environment>> = Lazy::new(|| Mutex::new(Environment::default()));

pub fn process(input: &str) -> Result<Value, Error> {
    let mut pairs =
        Pils::parse(Rule::Pils, input).with_context(|| format!("Failed to parse input {input}"))?;
    let pair = pairs.next().ok_or_else(|| anyhow!("Empty pair"))?;

    if pairs.next().is_some() {
        return Err(anyhow::anyhow!("More than one element in pair"));
    }

    let val = Value::from_pair(pair)
        .map_err(|_| anyhow::anyhow!("Failed to create value"))? // accenture disapproves
        .unwrap();

    let mut env = ENVIRONMENT.lock().unwrap();

    Value::eval(val, &mut env)
}

#[must_use]
pub fn process_str(line: &str) -> String {
    let result = process(line.trim());
    match result {
        Ok(v) => format!("{v}"),
        Err(e) => format!("Error: {e}"),
    }
}

#[must_use]
pub fn get_env_json() -> String {
    let Ok(env) = ENVIRONMENT.lock() else {
        return "Failed to acquire environment".to_string()
    };
    let env = &env.0;
    let env: HashMap<&String, &Value> = env
        .iter()
        .filter(|(_k, v)| !matches!(v, Value::Fun(_f)))
        .collect();
    serde_json::to_string_pretty(&env).unwrap_or("Serialization fault".to_string())
}

#[must_use]
pub fn get_env_tuples() -> String {
    let Ok(env) = ENVIRONMENT.lock() else {
        return "Failed to acquire environment".to_string()
    };
    let env = &env.0;
    let env: HashMap<&String, String> = env
        .iter()
        .filter(|(_k, v)| !matches!(v, Value::Fun(_f)))
        .map(|(k, v)| (k, format!("{v}")))
        .collect();
    serde_json::to_string_pretty(&env).unwrap_or("Serialization fault".to_string())
}

#[must_use]
pub fn get_example_environment() -> String {
    let line = "eval { tail ( list 1 2 3 4 ) }";
    let mut pairs = Pils::parse(Rule::Pils, line)
        .context("Failed to parse input")
        .unwrap();
    let pair = pairs
        .next()
        .ok_or_else(|| anyhow::anyhow!("empty pair"))
        .unwrap();

    let val = Value::from_pair(pair).unwrap().unwrap();
    let env = Environment::from_iter([
        ("key1".to_string(), val),
        ("key2".to_string(), Value::Sym("function1".to_string())),
    ]);
    let env: HashMap<String, String> = env
        .0
        .iter()
        .map(|(k, v)| (k.to_string(), format!("{v}")))
        .collect();
    serde_json::to_string(&env).unwrap()
}
