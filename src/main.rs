use crate::parser::Slipstream;
use anyhow::Context;
use builtin::builtin;
use parser::Rule;
use pest::{iterators::Pair, Parser};
use rustyline::{error::ReadlineError, Editor};

mod builtin;
mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Num(i64),
    Err(String),
    Sym(String),
    Sexpr(Vec<Value>),
    Qexpr(Vec<Value>),
}

fn make_value(pair: Pair<Rule>) -> anyhow::Result<Option<Value>> {
    let val = match pair.as_rule() {
        Rule::WHITESPACE => return Ok(None),
        Rule::Slipstream => Value::Sexpr(
            pair.into_inner()
                .map(make_value)
                .filter_map(Result::transpose)
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to parse slipstream expressions")?,
        ),
        Rule::Number => {
            let num = str::parse(pair.as_str())
                .context(format!("Failed to parse number from {}", pair.as_str()))?;
            Value::Num(num)
        }
        Rule::Symbol => {
            let sym = pair.as_str();
            Value::Sym(sym.to_string())
        }
        Rule::Expr => pair
            .into_inner()
            .map(make_value)
            .find_map(Result::transpose)
            .unwrap() // Expression must contain exactly one value as per grammar.
            .context("Failed to parse expression")?,
        Rule::Sexpr => Value::Sexpr(
            pair.into_inner()
                .map(make_value)
                .filter_map(Result::transpose)
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to parse Sexpr")?,
        ),
        Rule::Qexpr => Value::Qexpr(
            pair.into_inner()
                .map(make_value)
                .filter_map(Result::transpose)
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to parse Qexpr")?,
        ),
    };

    Ok(Some(val))
}

fn eval(val: Value) -> Value {
    match val {
        Value::Sexpr(children) => eval_sexpr(children),
        v => v,
    }
}

fn eval_sexpr(children: Vec<Value>) -> Value {
    let mut evaluated = children.into_iter().map(eval).collect::<Vec<_>>();
    match evaluated.iter().find(|elem| matches!(elem, Value::Err(_))) {
        Some(err) => {
            return err.clone();
        }
        None => {}
    }

    if evaluated.len() == 1 {
        return evaluated[0].clone();
    }

    let sym = match evaluated.pop() {
        Some(Value::Sym(str)) => str,
        Some(_) => {
            return Value::Err("S-expression does not start with symbol.".to_string());
        }
        None => {
            return Value::Sexpr(evaluated);
        }
    };

    builtin(&mut evaluated, sym)
}

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

    let val = make_value(pair).context("Failed to make value of pairs")?;
    println!("{:#?}", val);
    Ok(())
}
