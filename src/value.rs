use crate::parser::Rule;
use crate::{qexpr::Qexpr, sexpr::Sexpr};
use anyhow::Context;
use pest::iterators::Pair;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(i64),
    Err(String), // TODO thiserror :)
    Sym(String),
    Sexpr(Sexpr),
    Qexpr(Qexpr),
}

impl Value {
    pub fn eval(self) -> Self {
        match self {
            Value::Sexpr(s) => Sexpr::eval(s),
            v => v,
        }
    }

    pub fn from_pair(pair: Pair<Rule>) -> anyhow::Result<Option<Self>> {
        let val = match pair.as_rule() {
            Rule::WHITESPACE => return Ok(None),
            Rule::Expr => pair
                .into_inner()
                .map(Self::from_pair)
                .find_map(Result::transpose)
                .unwrap() // Expression must contain exactly one value as per grammar.
                .context("Failed to parse expression")?,
            Rule::Sexpr | Rule::Slipstream => Self::Sexpr(Sexpr(
                pair.into_inner()
                    .map(Self::from_pair)
                    .filter_map(Result::transpose)
                    .collect::<Result<VecDeque<_>, _>>()
                    .context("Failed to parse s-expression")?,
            )),
            Rule::Qexpr => Self::Qexpr(Qexpr(
                pair.into_inner()
                    .map(Self::from_pair)
                    .filter_map(Result::transpose)
                    .collect::<Result<VecDeque<_>, _>>()
                    .context("Failed to parse Qexpr")?,
            )),
            Rule::Symbol => Self::Sym(pair.as_str().to_string()),
            Rule::Number => Self::Num(
                str::parse(pair.as_str())
                    .context(format!("Failed to parse number from {}", pair.as_str()))?,
            ),
        };
        Ok(Some(val))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tail_qexpr() {
        let sexpr = Value::Sexpr(Sexpr(
            [
                Value::Sym("tail".to_string()),
                Value::Qexpr(Qexpr(
                    [
                        Value::Sym("tail".to_string()),
                        Value::Sym("join".to_string()),
                        Value::Sym("head".to_string()),
                    ]
                    .into_iter()
                    .collect::<VecDeque<_>>(),
                )),
            ]
            .into_iter()
            .collect::<VecDeque<_>>(),
        ));
        let result = Value::eval(sexpr);
        assert_eq!(
            result,
            Value::Qexpr(Qexpr(
                [
                    Value::Sym("join".to_string()),
                    Value::Sym("head".to_string()),
                ]
                .into_iter()
                .collect::<VecDeque<_>>(),
            )),
        );
    }

    #[test]
    fn list_sexpr_to_qexpr() {
        let value = Value::Sexpr(Sexpr(
            [
                Value::Sym("list".to_string()),
                Value::Num(1),
                Value::Num(2),
                Value::Num(3),
            ]
            .into_iter()
            .collect::<VecDeque<_>>(),
        ));
        dbg!(&value);
        let result = Value::eval(value);
        assert_eq!(
            result,
            Value::Qexpr(Qexpr(
                [Value::Num(1), Value::Num(2), Value::Num(3)]
                    .into_iter()
                    .collect::<VecDeque<_>>()
            )),
        );
    }
}
