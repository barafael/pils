use crate::{qexpr::Qexpr, value::Value};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sexpr(pub(crate) VecDeque<Value>);

impl Sexpr {
    pub fn eval(self) -> Value {
        dbg!(&self);
        let mut evaluated = self.0.into_iter().map(Value::eval).collect::<VecDeque<_>>();
        dbg!(&evaluated);
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

        match evaluated.pop_front() {
            Some(Value::Sexpr(s)) => match sym.as_str() {
                "list" => Value::Qexpr(Qexpr(s.0)),
                "+" | "-" | "/" | "*" => s.op(sym.as_str()),
                _ => Value::Err("Invalid operator".to_string()),
            },
            Some(Value::Qexpr(q)) => match sym.as_str() {
                "head" => q.head(),
                "tail" => q.tail(),
                "join" => q.join(),
                "eval" => q.eval(),
                _ => Value::Err("Invalid operator".to_string()),
            },
            _ => {
                return Value::Err("Invalid type".to_string());
            }
        }
    }

    pub fn op(self, sym: &str) -> Value {
        let mut results = VecDeque::new();
        for num in self.0 {
            match num {
                Value::Num(num) => results.push_back(num),
                _ => return Value::Err("Cannot operate on non-number!".to_string()),
            }
        }

        match sym {
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
