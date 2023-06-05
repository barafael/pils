use crate::{environment::Environment, value::Value};

#[derive(Clone)]
pub struct Function(pub(crate) fn(Value, &mut Environment) -> Result<Value, anyhow::Error>);

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}
