use crate::{parser::Slipstream, value::Value};
use error::Error;
use parser::Rule;
use pest::Parser;
use wasm_bindgen::prelude::*;

mod error;
mod eval_error;
mod parser;
mod qexpr;
mod sexpr;
mod value;

pub fn process(line: &str) -> Result<Value, Error> {
    let mut pairs = Slipstream::parse(Rule::Slipstream, line)
        .map_err(|_| Error::ParseInput(line.to_string()))?;
    let pair = pairs.next().ok_or(Error::EmptyPair)?;

    if pairs.next().is_some() {
        return Err(Error::MoreThanOneElementInPair);
    }

    let val = Value::from_pair(pair)
        .map_err(|_| Error::MakeValue)?
        .unwrap();

    Value::eval(val).map_err(Error::Eval)
}

#[wasm_bindgen]
#[must_use]
pub fn process_str(line: &str) -> String {
    let result = process(line);
    match result {
        Ok(v) => format!("{}", v),
        Err(e) => format!("{:?}", e),
    }
}

#[wasm_bindgen]
#[must_use]
pub fn help_text() -> String {
    r#"
Welcome to slipstream, a simple lisp :)

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
            "{ 1 2 3 4 ( 5 6 ) }"
        );
    }

    #[test]
    fn process_eval() {
        assert_eq!(process_str("eval { tail ( list 1 2 3 4 ) }"), "{ 2 3 4 }");
    }
}
