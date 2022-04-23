use crate::{parser::Slipstream, value::Value};
use anyhow::Context;
use parser::Rule;
use pest::Parser;

mod parser;
mod qexpr;
mod sexpr;
mod value;

pub fn process(line: &str) -> anyhow::Result<Value> {
    let mut pairs = Slipstream::parse(Rule::Slipstream, line).context("Failed to parse input")?;
    let pair = pairs.next().context("Can't make value of empty pair")?;

    if pairs.next().is_some() {
        anyhow::bail!("Can't make value of pairs with more than one element");
    }

    let val = Value::from_pair(pair)
        .context("Failed to make value of pairs")?
        .unwrap();

    Ok(Value::eval(val))
}
