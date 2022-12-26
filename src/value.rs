use crate::error::Error;
use crate::eval_error::EvalError;
use crate::parser::Rule;
use crate::{qexpr::Qexpr, sexpr::Sexpr};
use pest::iterators::Pair;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(i64),
    Sym(String),
    Sexpr(Sexpr),
    Qexpr(Qexpr),
}

impl Value {
    pub fn eval(self) -> Result<Self, EvalError> {
        match self {
            Self::Sexpr(s) => Sexpr::eval(s),
            v => Ok(v),
        }
    }

    pub fn from_pair(pair: Pair<Rule>) -> Result<Option<Self>, Error> {
        let val = match pair.as_rule() {
            Rule::WHITESPACE => return Ok(None),
            Rule::Expr => pair
                .into_inner()
                .map(Self::from_pair)
                .find_map(Result::transpose)
                .unwrap() // Expression must contain exactly one value as per grammar.
                .map_err(|_| Error::ParseExpression)?,
            Rule::Sexpr | Rule::Pils => Self::Sexpr(Sexpr(
                pair.into_inner()
                    .map(Self::from_pair)
                    .filter_map(Result::transpose)
                    .collect::<Result<VecDeque<_>, _>>()
                    .map_err(|_| Error::ParseSExpression)?,
            )),
            Rule::Qexpr => Self::Qexpr(Qexpr(
                pair.into_inner()
                    .map(Self::from_pair)
                    .filter_map(Result::transpose)
                    .collect::<Result<VecDeque<_>, _>>()
                    .map_err(|_| Error::ParseQExpression)?,
            )),
            Rule::Symbol => Self::Sym(pair.as_str().to_string()),
            Rule::Number => Self::Num(
                str::parse(pair.as_str())
                    .map_err(|_| Error::ParseNumber(pair.as_str().to_string()))?,
            ),
        };
        Ok(Some(val))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Sym(s) => write!(f, "{s}"),
            Self::Sexpr(s) => write!(f, "{s}"),
            Self::Qexpr(q) => write!(f, "{q}"),
        }
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
        let result = Value::eval(sexpr).unwrap();
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
        let result = Value::eval(value).unwrap();
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
