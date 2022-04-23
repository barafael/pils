use crate::{qexpr::Qexpr, value::Value};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sexpr(pub(crate) VecDeque<Value>);

impl Sexpr {
    pub fn eval(self) -> Value {
        let mut evaluated = self.0.into_iter().map(Value::eval).collect::<VecDeque<_>>();
        match evaluated.iter().find(|elem| matches!(elem, Value::Err(_))) {
            Some(err) => {
                return err.clone();
            }
            None => {}
        }

        if evaluated.len() == 1 {
            return evaluated[0].clone();
        }

        let sym = match evaluated.pop_front() {
            Some(Value::Sym(str)) => str,
            Some(v) => {
                return Value::Err(format!("S-expression does not start with symbol: {:?}", v));
            }
            None => {
                return Value::Sexpr(Self(evaluated));
            }
        };

        let operator = match sym.as_str() {
            "list" => return Value::Qexpr(Qexpr(evaluated)),
            "+" | "-" | "/" | "*" => return Self(evaluated).op(sym.as_str()),
            o => o,
        };

        match operator {
            "head" => {
                if evaluated.len() != 1 {
                    Value::Err("Function 'head' passed too many arguments.".to_string())
                } else if let Value::Qexpr(q) = evaluated[0].clone() {
                    q.head()
                } else {
                    Value::Err("Function 'head' passed incorrect type.".to_string())
                }
            }
            "tail" => {
                if evaluated.len() != 1 {
                    Value::Err("Function 'tail' passed too many arguments.".to_string())
                } else if let Value::Qexpr(q) = evaluated[0].clone() {
                    q.tail()
                } else {
                    Value::Err("Function 'tail' passed incorrect type.".to_string())
                }
            }
            "join" => Qexpr(evaluated).join(),
            "eval" => Qexpr(evaluated).eval(),
            _ => Value::Err("Invalid operator".to_string()),
        }
    }

    pub fn op(self, operator: &str) -> Value {
        let mut results = VecDeque::new();
        for num in self.0 {
            match num {
                Value::Num(num) => results.push_back(num),
                _ => return Value::Err("Cannot operate on non-number!".to_string()),
            }
        }

        match operator {
            "+" => Value::Num(results.into_iter().sum()),
            "-" => Value::Num(results.into_iter().fold(0, |acc, val| acc - val)),
            "*" => Value::Num(results.into_iter().product()),
            "/" => {
                if results.iter().any(|num| *num == 0) {
                    return Value::Err("Division by zero!".to_string());
                }
                if results.len() == 1 {
                    return Value::Num(results[0]);
                }
                let mut fst = results[0];
                for val in results.iter().skip(1) {
                    fst /= val;
                }
                Value::Num(fst)
            }
            _ => Value::Err("Invalid operator".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evals_op_sexpr() {
        let operands = Sexpr(
            [Value::Num(1), Value::Num(2), Value::Num(4)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let result = operands.op("*");
        assert_eq!(result, Value::Num(8));
    }

    #[test]
    fn rejects_to_divide_by_zero() {
        let operands = Sexpr(
            [Value::Num(12), Value::Num(0), Value::Num(4)]
                .into_iter()
                .collect::<VecDeque<_>>(),
        );
        let result = operands.op("/");
        assert_eq!(result, Value::Err("Division by zero!".to_string()));
    }

    #[test]
    fn unary_minus() {
        let operands = Sexpr([Value::Num(12)].into_iter().collect::<VecDeque<_>>());
        assert_eq!(operands.op("-"), Value::Num(-12));
    }
}
