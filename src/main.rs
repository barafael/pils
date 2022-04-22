use crate::{parser::Slipstream, value::Value};
use anyhow::Context;
use parser::Rule;
use pest::Parser;
use rustyline::{error::ReadlineError, Editor};

mod parser;
mod qexpr;
mod sexpr;
mod value;

fn main() -> anyhow::Result<()> {
    let mut prompt = Editor::<()>::new();
    loop {
        match prompt.readline("slipstream >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }
                prompt.add_history_entry(&line);
                process(line.as_str()).context("Failed to process input")?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn process(line: &str) -> anyhow::Result<()> {
    let mut pairs = Slipstream::parse(Rule::Slipstream, line).context("Failed to parse input")?;
    println!("{:#?}", pairs);
    let pair = pairs.next().context("Can't make value of empty pair")?;

    if pairs.next().is_some() {
        anyhow::bail!("Can't make value of pairs with more than one element");
    }

    let val = Value::from_pair(pair)
        .context("Failed to make value of pairs")?
        .unwrap();
    println!("{:#?}", val);

    let result = Value::eval(val);
    println!("result: {:#?}", result);

    Ok(())
}
