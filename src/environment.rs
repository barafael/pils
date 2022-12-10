use crate::{builtin, function::Function, value::Value};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment(pub(crate) HashMap<String, Value>);

impl Default for Environment {
    fn default() -> Self {
        let mut m = Self(HashMap::new());
        m.0.insert("list".to_string(), Value::Fun(Function(builtin::list)));
        m.0.insert("head".to_string(), Value::Fun(Function(builtin::head)));
        m.0.insert("tail".to_string(), Value::Fun(Function(builtin::tail)));
        m.0.insert("join".to_string(), Value::Fun(Function(builtin::join)));
        m.0.insert("eval".to_string(), Value::Fun(Function(builtin::eval)));
        m.0.insert("def".to_string(), Value::Fun(Function(builtin::def)));

        m.0.insert("+".to_string(), Value::Fun(Function(builtin::add)));
        m.0.insert("-".to_string(), Value::Fun(Function(builtin::sub)));
        m.0.insert("*".to_string(), Value::Fun(Function(builtin::mul)));
        m.0.insert("/".to_string(), Value::Fun(Function(builtin::div)));
        m
    }
}

impl FromIterator<(String, Value)> for Environment {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
