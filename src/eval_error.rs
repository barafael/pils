use crate::value::Value;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EvalError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("S-expression does not start with symbol: {0}")]
    SExprDoesNotStartWithSymbol(Value),

    #[error("Function 'head' passed {{}}")]
    HeadOnEmpty,

    #[error("Function 'head' passed too many arguments.")]
    HeadOnTooManyArgs,

    #[error("Function 'head' passed incorrect type.")]
    HeadOnNonQexpr,

    #[error("Function 'tail' passed {{}}")]
    TailOnEmpty,

    #[error("Function 'tail' passed too many arguments.")]
    TailOnTooManyArgs,

    #[error("Function 'tail' passed incorrect type.")]
    TailOnNonQexpr,

    #[error("Function 'join' passed incorrect type")]
    JoinOnNonQexpr,

    #[error("Function 'join' passed too many arguments.")]
    JoinOnTooManyArgs,

    #[error("Function 'eval' passed too many arguments.")]
    EvalOnTooManyArgs,

    #[error("Function 'eval' passed incorrect type.")]
    EvalOnNonQexpr,

    #[error("Invalid operator {0}")]
    InvalidOperator(String),

    #[error("Cannot operate on non-number: {0}")]
    NonNumber(Value),
}
