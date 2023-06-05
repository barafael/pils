use crate::{environment::Environment, value::Value};
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sexpr(pub(crate) VecDeque<Value>);

impl Sexpr {
    pub fn eval(self, env: &mut Environment) -> Result<Value, anyhow::Error> {
        let mut evaluated = self
            .0
            .into_iter()
            .map(|v| v.eval(env))
            .collect::<Result<VecDeque<_>, anyhow::Error>>()?;

        if evaluated.is_empty() {
            return Ok(Value::Sexpr(Self(VecDeque::default())));
        }

        if evaluated.len() == 1 {
            return Ok(evaluated[0].clone());
        }

        let Value::Fun(fun) = evaluated.pop_front().unwrap() else {
            return Err(anyhow::anyhow!("First element is not a function"));
        };

        fun.0(Value::Sexpr(Self(evaluated)), env)
    }

    pub fn add(self) -> Result<Value, anyhow::Error> {
        let n = self
            .0
            .into_iter()
            .map(|n| match n {
                Value::Num(n) => Ok(n),
                _ => Err(anyhow::anyhow!("'add' on non-number")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Value::Num(n.into_iter().sum()))
    }

    pub fn sub(self) -> Result<Value, anyhow::Error> {
        let n = self
            .0
            .into_iter()
            .map(|n| match n {
                Value::Num(n) => Ok(n),
                _ => Err(anyhow::anyhow!("'sub' on non-number")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Value::Num(n.into_iter().fold(0, |acc, val| acc - val)))
    }

    pub fn mul(self) -> Result<Value, anyhow::Error> {
        let n = self
            .0
            .into_iter()
            .map(|n| match n {
                Value::Num(n) => Ok(n),
                _ => Err(anyhow::anyhow!("'mul' on non-number")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Value::Num(n.into_iter().product()))
    }

    pub fn div(self) -> Result<Value, anyhow::Error> {
        let n = self
            .0
            .into_iter()
            .map(|n| match n {
                Value::Num(0) => Err(anyhow::anyhow!("Division by zero")),
                Value::Num(n) => Ok(n),
                _ => Err(anyhow::anyhow!("'div' on non-number")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        // TODO elegantify
        if n.len() == 1 {
            return Ok(Value::Num(n[0]));
        }
        let mut fst = n[0];
        for val in n.iter().skip(1) {
            fst /= val;
        }
        Ok(Value::Num(fst))
    }
}

impl FromIterator<Value> for Sexpr {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
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

    #[test]
    fn sexpr_multiplication() {
        let mut env = Environment::default();
        let mul = env.0.get("*").unwrap();

        let operands = Sexpr::from_iter([mul.clone(), Value::Num(1), Value::Num(2), Value::Num(4)]);
        let num = operands.eval(&mut env).unwrap();
        assert!(matches!(num, Value::Num(8)));
    }

    #[test]
    fn division() {
        let mut env = Environment::default();
        let div = env.0.get("/").unwrap();
        let operands =
            Sexpr::from_iter([div.clone(), Value::Num(12), Value::Num(1), Value::Num(4)]);
        assert!(matches!(operands.eval(&mut env).unwrap(), Value::Num(3)));
    }

    #[test]
    fn rejects_to_divide_by_zero() {
        let mut env = Environment::default();
        let div = env.0.get("/").unwrap();
        let operands =
            Sexpr::from_iter([div.clone(), Value::Num(12), Value::Num(0), Value::Num(4)]);
        let result = operands.eval(&mut env).unwrap_err();
        assert_eq!(format!("{}", result), "Division by zero");
    }

    #[test]
    fn unary_minus() {
        let mut env = Environment::default();
        let sub = env.0.get("-").unwrap();
        let operands = Sexpr::from_iter([sub.clone(), Value::Num(12)]);
        assert!(matches!(operands.eval(&mut env).unwrap(), Value::Num(-12)));
    }
}
