use crate::value::Value;
use anyhow::anyhow;
use anyhow::{Context, Error};
use environment::Environment;
use once_cell::sync::Lazy;
use parser::{Pils, Rule};
use pest::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

pub mod builtin;
pub mod environment;
mod function;
mod parser;
mod qexpr;
mod sexpr;
mod value;

static ENVIRONMENT: Lazy<Mutex<Environment>> = Lazy::new(|| Mutex::new(Environment::default()));

pub fn process(input: &str) -> Result<Value, Error> {
    let mut pairs =
        Pils::parse(Rule::Pils, input).with_context(|| format!("Failed to parse input {input}"))?;
    let pair = pairs.next().ok_or_else(|| anyhow::anyhow!("Empty pair"))?;

    if pairs.next().is_some() {
        return Err(anyhow::anyhow!("More than one element in pair"));
    }

    let val = Value::from_pair(pair)
        .map_err(|_| anyhow::anyhow!("Failed to create value"))? // accenture disapproves
        .unwrap();

    let mut env = ENVIRONMENT.lock().unwrap();

    Value::eval(val, &mut env)
}

#[wasm_bindgen]
#[must_use]
pub fn process_str(line: &str) -> String {
    let result = process(line.trim());
    match result {
        Ok(v) => format!("{v}"),
        Err(e) => format!("Error: {e}"),
    }
}

#[wasm_bindgen]
#[must_use]
pub fn get_env() -> String {
    let Ok(env) = ENVIRONMENT.lock() else {
        return "Failed to acquire environment".to_string()
    };
    let env = &env.0;
    let env: HashMap<&String, &Value> = env
        .iter()
        .filter(|(_k, v)| !matches!(v, Value::Fun(_f)))
        .collect();
    serde_json::to_string_pretty(&env).unwrap_or("".to_string())
}

#[wasm_bindgen]
#[must_use]
pub fn help_text() -> String {
    // TODO include_str here and put to docs and readme maybe
    r#"Welcome to pils, a simple lisp :)

example expressions:
    * 1 2 3 4 ( + 3 4) 0
    ( / 100 3 10 )
    eval { tail ( list 1 2 3 4 ) }
    eval (tail {tail tail {5 6 7}})
    tail { tail join eval head }
    eval {head (list 1 2 3 4)}

+, -, *, / work as prefix operators on numbers
    and s-expressions that evaluate to numbers.

'(' and ')' create an s-expression like so: '(* 1 2 3 )'
    An s-expression always starts with an operator and
    is followed by numbers or other s-expressions.

'{' and '}' create a q-expression like so: '{ 1 2 3 tail }'
    A q-expression is not evaluated and can contain anything.
    Special operators act on q-expressions:

'head' takes the first element of a q-expression.
'tail' takes all elements of a q-expression, except the first.
'join' takes a q-expression with q-expressions inside, and
    creates one q-expression with their contents.
'eval' pretends a q-expression is an s-expression and
    evaluates it normally.

'list' creates a q-expression from an s-expression.

For a detailed reference, see: https://buildyourownlisp.com/.
Thanks and credits to Daniel Holden for this brilliant resource.
"#
    .to_string()
}

#[wasm_bindgen]
#[must_use]
pub fn get_example_environment() -> String {
    let line = "eval { tail ( list 1 2 3 4 ) }";
    let mut pairs = Pils::parse(Rule::Pils, line)
        .context("Failed to parse input")
        .unwrap();
    let pair = pairs.next().ok_or_else(|| anyhow!("empty pair")).unwrap();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_simple_sexpr() {
        assert_eq!(process_str("* 2 (+ 4 5) (/ 10 2) (-2) (- 1 2 3)"), "1080");
    }

    #[test]
    fn process_simple_qexpr() {
        assert_eq!(process_str("{ * 1 2 3 }"), "{ * 1 2 3 }");
    }

    #[test]
    fn process_join() {
        assert_eq!(
            process_str("join { { 1 2 3 } { 4 ( 5 6 ) } }"),
            "{ 1 2 3 4 ( 5 6 ) }" // disagrees with variables.c, wtf?
        );
    }

    #[test]
    fn example_11a() {
        assert_eq!(process_str("+"), "<function>");
    }

    #[test]
    fn example_11b() {
        assert_eq!(process_str("eval (head {5 10 11 15})"), "5");
    }

    #[test]
    fn example_11d() {
        assert_eq!(process_str("(eval (head {+ - + - * /})) 10 20"), "30");
    }

    #[test]
    fn example_11e() {
        assert_eq!(process_str("hello"), "Error: unbound symbol");
    }

    #[test]
    fn process_eval() {
        assert_eq!(process_str("eval { tail ( list 1 2 3 4 ) }"), "{ 2 3 4 }");
    }

    #[test]
    fn process_def() {
        let _ = process_str("def {x} 100");
        let _ = process_str("def {y} 200");
        assert_eq!(process_str("+ x y"), "300".to_string());
        let _ = process_str("def {a b} 5 6");
        assert_eq!(process_str("+ a b"), "11".to_string());
        let _ = process_str("def {arglist} {a b x y}");
        let _ = process_str("def arglist 1 2 3 4");
        assert_eq!(process_str("list a b x y"), "{ 1 2 3 4 }".to_string());
    }

    #[test]
    fn displays() {
        let line = "eval { tail ( list 1 2 3 4 ) }";
        let mut pairs = Pils::parse(Rule::Pils, line).unwrap();
        let pair = pairs.next().unwrap();

        let val = Value::from_pair(pair).unwrap().unwrap();

        let result = format!("{val}");
        assert_eq!(format!("( {line} )"), result);
    }
}
