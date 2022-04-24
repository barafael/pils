use crate::{eval_error::EvalError, qexpr::Qexpr, value::Value};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sexpr(pub(crate) VecDeque<Value>);

impl Sexpr {
    pub fn eval(self) -> Result<Value, EvalError> {
        let mut evaluated = self
            .0
            .into_iter()
            .map(Value::eval)
            .collect::<Result<VecDeque<_>, EvalError>>()?;

        if evaluated.len() == 1 {
            return Ok(evaluated[0].clone());
        }

        let sym = match evaluated.pop_front() {
            Some(Value::Sym(str)) => str,
            Some(v) => return Err(EvalError::SExprDoesNotStartWithSymbol(v)),
            None => {
                return Ok(Value::Sexpr(Self(evaluated)));
            }
        };

        let operator = match sym.as_str() {
            "list" => return Ok(Value::Qexpr(Qexpr(evaluated))),
            "+" | "-" | "/" | "*" => return Self(evaluated).op(sym.as_str()),
            o => o,
        };

        match operator {
            "head" => {
                if evaluated.len() != 1 {
                    Err(EvalError::HeadOnTooManyArgs)
                } else if let Value::Qexpr(q) = evaluated[0].clone() {
                    q.head()
                } else {
                    Err(EvalError::HeadOnNonQexpr)
                }
            }
            "tail" => {
                if evaluated.len() != 1 {
                    Err(EvalError::TailOnTooManyArgs)
                } else if let Value::Qexpr(q) = evaluated[0].clone() {
                    q.tail()
                } else {
                    Err(EvalError::TailOnNonQexpr)
                }
            }
            "join" => {
                if evaluated.len() != 1 {
                    Err(EvalError::JoinOnTooManyArgs)
                } else if let Value::Qexpr(q) = evaluated[0].clone() {
                    q.join()
                } else {
                    Err(EvalError::JoinOnNonQexpr)
                }
            }
            "eval" => {
                if evaluated.len() != 1 {
                    Err(EvalError::EvalOnTooManyArgs)
                } else if let Value::Qexpr(q) = evaluated[0].clone() {
                    q.eval()
                } else {
                    Err(EvalError::EvalOnNonQexpr)
                }
            }
            op => Err(EvalError::InvalidOperator(op.to_string())),
        }
    }

    pub fn op(self, operator: &str) -> Result<Value, EvalError> {
        let mut results = VecDeque::new();
        for num in self.0 {
            match num {
                Value::Num(num) => results.push_back(num),
                val => return Err(EvalError::NonNumber(val)),
            }
        }

        match operator {
            "+" => Ok(Value::Num(results.into_iter().sum())),
            "-" => Ok(Value::Num(
                results.into_iter().fold(0, |acc, val| acc - val),
            )),
            "*" => Ok(Value::Num(results.into_iter().product())),
            "/" => {
                if results.iter().any(|num| *num == 0) {
                    return Err(EvalError::DivisionByZero);
                }
                if results.len() == 1 {
                    return Ok(Value::Num(results[0]));
                }
                let mut fst = results[0];
                for val in results.iter().skip(1) {
                    fst /= val;
                }
                Ok(Value::Num(fst))
            }
            op => Err(EvalError::InvalidOperator(op.to_string())),
        }
    }
}

impl std::fmt::Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        write!(f, " {} ", self.0.iter().join(" "))?;
        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eval_error::EvalError;

    #[test]
    fn sexpr_multiplication() {
        let operands = Sexpr(
            [Value::Num(1), Value::Num(2), Value::Num(4)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let result = operands.op("*").unwrap();
        assert_eq!(result, Value::Num(8));
    }

    #[test]
    fn division() {
        let operands = Sexpr(
            [Value::Num(12), Value::Num(1), Value::Num(4)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let result = operands.op("/").unwrap();
        assert_eq!(result, Value::Num(3));
    }

    #[test]
    fn rejects_to_divide_by_zero() {
        let operands = Sexpr(
            [Value::Num(12), Value::Num(0), Value::Num(4)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let result = operands.op("/").unwrap_err();
        assert_eq!(result, EvalError::DivisionByZero);
    }

    #[test]
    fn unary_minus() {
        let operands = Sexpr([Value::Num(12)].into_iter().collect::<VecDeque<_>>());
        assert_eq!(operands.op("-").unwrap(), Value::Num(-12));
    }
}
