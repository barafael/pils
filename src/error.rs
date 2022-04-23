use crate::eval_error::EvalError;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Failed to make value of pairs")]
    MakeValue,

    #[error("Failed to parse input {0}")]
    ParseInput(String),

    #[error("Can't make value of empty pair")]
    EmptyPair,

    #[error("Can't make value of pairs with more than one element")]
    MoreThanOneElementInPair,

    #[error("Failed to parse expression")]
    ParseExpression,

    #[error("Failed to parse S-Expression")]
    ParseSExpression,

    #[error("Failed to parse Qexpr")]
    ParseQExpression,

    #[error("Failed to parse number from {0}")]
    ParseNumber(String),

    #[error("Failed to evaluate expression: {0}")]
    Eval(#[from] EvalError),
}
