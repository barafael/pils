use crate::{parser::Slipstream, value::Value};
use error::Error;
use parser::Rule;
use pest::Parser;
use wasm_bindgen::prelude::*;

mod error;
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

    Ok(Value::eval(val))
}

#[wasm_bindgen]
pub fn process_str(line: &str) -> String {
    let result = process(line);
    match result {
        Ok(v) => format!("{}", v),
        Err(e) => format!("{:?}", e),
    }
}
