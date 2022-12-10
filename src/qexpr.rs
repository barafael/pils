use crate::{environment::Environment, sexpr::Sexpr, value::Value};
use anyhow::{Context, Error};
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Qexpr(pub(crate) VecDeque<Value>);

impl Qexpr {
    pub fn into_sexpr(self) -> Sexpr {
        Sexpr(self.0)
    }

    pub fn head(mut self) -> Result<Value, anyhow::Error> {
        self.0.pop_front().context("'head' on empty qexpr")
    }

    pub fn tail(mut self) -> Result<Value, anyhow::Error> {
        self.0.pop_front().context("'tail' on empty qexpr")?;
        Ok(Value::Qexpr(Self(self.0)))
    }

    pub fn join(self) -> Result<Value, anyhow::Error> {
        let mut joined = VecDeque::new();
        // TODO flat_map
        for child in self.0 {
            let Value::Qexpr(mut child) = child else {
                return Err(anyhow::anyhow!("Join on non-qexpr"));
            };
            joined.append(&mut child.0);
        }
        Ok(Value::Qexpr(Self(joined)))
    }

    pub fn eval(self, env: &mut Environment) -> Result<Value, Error> {
        let sexpr = Sexpr(self.0);
        sexpr.eval(env)
    }
}

impl std::fmt::Display for Qexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, " {} ", self.0.iter().join(" "))?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl FromIterator<Value> for Qexpr {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn head() {
        let qexpr = Qexpr::from_iter([Value::Num(1), Value::Num(2), Value::Num(3)]);
        let head = qexpr.head().unwrap();
        assert!(matches!(head, Value::Num(1)));
    }

    #[test]
    fn tail() {
        let qexpr = Qexpr::from_iter([Value::Num(1), Value::Num(2), Value::Num(3)]);
        let tail = qexpr.tail().unwrap();
        assert_eq!(
            tail,
            Value::Qexpr(Qexpr::from_iter([Value::Num(2), Value::Num(3)]))
        );
    }

    #[test]
    fn join() {
        let qexpr = Qexpr::from_iter([
            Value::Qexpr(Qexpr::from_iter([
                Value::Num(1),
                Value::Num(2),
                Value::Num(3),
            ])),
            Value::Qexpr(Qexpr::from_iter([
                Value::Num(4),
                Value::Num(5),
                Value::Num(6),
            ])),
            Value::Qexpr(Qexpr::from_iter([
                Value::Num(7),
                Value::Num(8),
                Value::Num(9),
            ])),
        ]);

        let result = qexpr.join().unwrap();

        let expected = Value::Qexpr(Qexpr::from_iter([
            Value::Num(1),
            Value::Num(2),
            Value::Num(3),
            Value::Num(4),
            Value::Num(5),
            Value::Num(6),
            Value::Num(7),
            Value::Num(8),
            Value::Num(9),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn into_sexpr() {
        let inner = [
            Value::Sym("head".to_string()),
            Value::Qexpr(Qexpr::from_iter([
                Value::Num(1),
                Value::Num(2),
                Value::Num(3),
                Value::Num(4),
            ])),
        ]
        .into_iter()
        .collect::<VecDeque<_>>();
        let value = Qexpr(inner.clone());
        let result = Qexpr::into_sexpr(value);
        let expected = Sexpr(inner);
        assert_eq!(result, expected);
    }

    #[test]
    fn eval() {
        let value = Qexpr::from_iter([
            Value::Sym("head".to_string()),
            Value::Qexpr(Qexpr::from_iter([
                Value::Num(1),
                Value::Num(2),
                Value::Num(3),
                Value::Num(4),
            ])),
        ]);
        let result = Qexpr::eval(value, &mut Environment::default()).unwrap();
        assert_eq!(result, Value::Num(1));
    }
}
