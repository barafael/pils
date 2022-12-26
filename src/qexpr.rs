use crate::{eval_error::EvalError, sexpr::Sexpr, value::Value};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qexpr(pub(crate) VecDeque<Value>);

impl Qexpr {
    pub fn head(mut self) -> Result<Value, EvalError> {
        if self.0.is_empty() {
            return Err(EvalError::HeadOnEmpty);
        }
        Ok(Value::Qexpr(Self(
            std::iter::once(self.0.pop_front().unwrap()).collect::<VecDeque<_>>(),
        )))
    }

    pub fn tail(mut self) -> Result<Value, EvalError> {
        if self.0.is_empty() {
            return Err(EvalError::TailOnEmpty);
        }
        self.0.pop_front().unwrap();
        Ok(Value::Qexpr(Self(self.0)))
    }

    pub fn join(self) -> Result<Value, EvalError> {
        let mut joined = VecDeque::new();
        for child in self.0 {
            let Value::Qexpr(mut child) = child else {
                return  Err(EvalError::JoinOnNonQexpr);
            };
            joined.append(&mut child.0);
        }
        Ok(Value::Qexpr(Self(joined)))
    }

    pub fn eval(self) -> Result<Value, EvalError> {
        let sexpr = Sexpr(self.0);
        sexpr.eval()
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

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn head() {
        let qexpr = Qexpr(
            [Value::Num(1), Value::Num(2), Value::Num(3)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let head = qexpr.head().unwrap();
        assert_eq!(
            head,
            Value::Qexpr(Qexpr([Value::Num(1)].into_iter().collect::<VecDeque<_>>()))
        );
    }

    #[test]
    fn tail() {
        let qexpr = Qexpr(
            [Value::Num(1), Value::Num(2), Value::Num(3)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let tail = qexpr.tail().unwrap();
        assert_eq!(
            tail,
            Value::Qexpr(Qexpr(
                [Value::Num(2), Value::Num(3)]
                    .into_iter()
                    .collect::<VecDeque<_>>()
            ))
        );
    }

    #[test]
    fn join() {
        let qexpr = Qexpr(
            [
                Value::Qexpr(Qexpr(
                    [Value::Num(1), Value::Num(2), Value::Num(3)]
                        .into_iter()
                        .collect::<VecDeque<_>>(),
                )),
                Value::Qexpr(Qexpr(
                    [Value::Num(4), Value::Num(5), Value::Num(6)]
                        .into_iter()
                        .collect::<VecDeque<_>>(),
                )),
                Value::Qexpr(Qexpr(
                    [Value::Num(7), Value::Num(8), Value::Num(9)]
                        .into_iter()
                        .collect::<VecDeque<_>>(),
                )),
            ]
            .into_iter()
            .collect::<VecDeque<_>>(),
        );
        let expected = Value::Qexpr(Qexpr(
            [
                Value::Num(1),
                Value::Num(2),
                Value::Num(3),
                Value::Num(4),
                Value::Num(5),
                Value::Num(6),
                Value::Num(7),
                Value::Num(8),
                Value::Num(9),
            ]
            .into_iter()
            .collect::<VecDeque<_>>(),
        ));

        let result = qexpr.join().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn eval() {
        let value = Qexpr(
            [
                Value::Sym("head".to_string()),
                Value::Qexpr(Qexpr(
                    [Value::Num(1), Value::Num(2), Value::Num(3), Value::Num(4)]
                        .into_iter()
                        .collect::<VecDeque<_>>(),
                )),
            ]
            .into_iter()
            .collect::<VecDeque<_>>(),
        );
        let result = Qexpr::eval(value).unwrap();

        let expected = Value::Qexpr(Qexpr([Value::Num(1)].into_iter().collect::<VecDeque<_>>()));
        assert_eq!(expected, result);
    }
}
