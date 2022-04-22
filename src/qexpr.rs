use crate::{sexpr::Sexpr, value::Value};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qexpr(pub(crate) VecDeque<Value>);

impl Qexpr {
    pub fn head(mut self) -> Value {
        if self.0.is_empty() {
            return Value::Err("Function 'head' passed {}".to_string());
        }
        self.0.pop_front().unwrap()
    }

    pub fn tail(mut self) -> Value {
        if self.0.is_empty() {
            return Value::Err("Function 'tail' passed {}".to_string());
        }
        let _ = self.0.pop_front().unwrap();
        Value::Qexpr(Self(self.0))
    }

    pub fn join(self) -> Value {
        let mut joined = VecDeque::new();
        for child in self.0 {
            let mut child = match child {
                Value::Qexpr(child) => child,
                _ => return Value::Err("Function 'join' passed incorrect type".to_string()),
            };
            joined.append(&mut child.0);
        }
        Value::Qexpr(Self(joined))
    }

    pub fn eval(self) -> Value {
        let sexpr = Sexpr(self.0);
        sexpr.eval()
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
        let head = qexpr.head();
        assert_eq!(head, Value::Num(1));
    }

    #[test]
    fn tail() {
        let qexpr = Qexpr(
            [Value::Num(1), Value::Num(2), Value::Num(3)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let tail = qexpr.tail();
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

        let result = qexpr.join();
        assert_eq!(expected, result);
    }
}
