use crate::environment::Environment;
use crate::function::Function;
use crate::parser::Rule;
use crate::{qexpr::Qexpr, sexpr::Sexpr};
use anyhow::Context;
use pest::iterators::Pair;
use serde_derive::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Num(i64),
    Sym(String),
    Sexpr(Sexpr),
    Qexpr(Qexpr),
    #[serde(skip)]
    Fun(Function),
}

impl Value {
    pub fn eval(self, env: &mut Environment) -> Result<Self, anyhow::Error> {
        match self {
            Self::Sym(ref sym) => {
                let value = env.0.get(sym).context("unbound symbol")?;
                Ok(value.clone())
            }
            Self::Sexpr(s) => Sexpr::eval(s, env),
            v => Ok(v),
        }
    }

    pub fn from_pair(pair: Pair<Rule>) -> Result<Option<Self>, anyhow::Error> {
        let val = match pair.as_rule() {
            Rule::WHITESPACE => return Ok(None),
            Rule::Expr => pair
                .into_inner()
                .map(Self::from_pair)
                .find_map(Result::transpose)
                .unwrap() // Expression must contain exactly one value as per grammar.
                .map_err(|_| anyhow::anyhow!("Failed to parse expression"))?,
            Rule::Sexpr | Rule::Pils => Self::Sexpr(Sexpr(
                pair.into_inner()
                    .map(Self::from_pair)
                    .filter_map(Result::transpose)
                    .collect::<Result<VecDeque<_>, _>>()
                    .map_err(|_| anyhow::anyhow!("Failed to parse S-Expression"))?,
            )),
            Rule::Qexpr => Self::Qexpr(Qexpr(
                pair.into_inner()
                    .map(Self::from_pair)
                    .filter_map(Result::transpose)
                    .collect::<Result<VecDeque<_>, _>>()
                    .map_err(|_| anyhow::anyhow!("Failed to parse Q-Expression"))?,
            )),
            Rule::Symbol => Self::Sym(pair.as_str().to_string()),
            Rule::Number => {
                let x = str::parse::<i64>(pair.as_str())
                    .map_err(|_| anyhow::anyhow!("Failed to parse number"))?;
                Self::Num(x)
            }
        };
        Ok(Some(val))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Sym(s) => write!(f, "{s}"),
            // TODO why no '()'?
            Self::Sexpr(s) => write!(f, "{s}"),
            // TODO why no '{}'?
            Self::Qexpr(q) => write!(f, "{q}"),
            Self::Fun(_fun) => write!(f, "<function>"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tail_qexpr() {
        let sexpr = Value::Sexpr(Sexpr::from_iter([
            Value::Sym("tail".to_string()),
            Value::Qexpr(Qexpr::from_iter([
                Value::Sym("tail".to_string()),
                Value::Sym("join".to_string()),
                Value::Sym("head".to_string()),
            ])),
        ]));
        let mut env = Environment::default();
        let result = Value::eval(sexpr, &mut env).unwrap();
        assert_eq!(
            result,
            Value::Qexpr(Qexpr::from_iter([
                Value::Sym("join".to_string()),
                Value::Sym("head".to_string()),
            ]))
        );
    }

    #[test]
    fn list_sexpr_to_qexpr() {
        let value = Value::Sexpr(Sexpr::from_iter([
            Value::Sym("list".to_string()),
            Value::Num(1),
            Value::Num(2),
            Value::Num(3),
        ]));
        let mut env = Environment::default();
        let result = Value::eval(value, &mut env).unwrap();
        assert_eq!(
            result,
            Value::Qexpr(Qexpr::from_iter([
                Value::Num(1),
                Value::Num(2),
                Value::Num(3)
            ]))
        );
    }
}
