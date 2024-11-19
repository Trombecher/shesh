mod functions;
mod scope;

use crate::read::ast::{BinaryOperation, Expression};
use crate::read::bytes::Span;
use crate::runtime::scope::Scope;
use crossterm::style::{Color, SetForegroundColor};
use std::fmt::{Display, Formatter};
use std::str::SplitWhitespace;

pub use scope::*;

#[derive(Debug)]
pub struct Variable {
    mutable: bool,
    value: Value,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Nil,
    Function(fn(scope: &mut Scope) -> Result<Value, RuntimeError>),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(
                f,
                "{}{}{}",
                SetForegroundColor(Color::Blue),
                num,
                SetForegroundColor(Color::Reset)
            ),
            Self::Nil => write!(
                f,
                "{}Nil{}",
                SetForegroundColor(Color::Grey),
                SetForegroundColor(Color::Reset)
            ),
            Self::Function(_) => write!(
                f,
                "{}[function]{}",
                SetForegroundColor(Color::Yellow),
                SetForegroundColor(Color::Reset)
            ),
            Self::String(s) => write!(
                f,
                "{}\"{}\"{}",
                SetForegroundColor(Color::DarkGreen),
                s,
                SetForegroundColor(Color::Reset)
            ),
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum RuntimeError {
    UndefinedVariable,
    UnimplementedFeature,
}

pub fn eval(
    scope: &mut Scope,
    root_expression: &Span<Expression>
) -> Result<Value, RuntimeError> {
    match &root_expression.value {
        Expression::Binary { left, operation, right } => {
            let left = eval(scope, left)?;
            let right = eval(scope, right)?;

            match operation {
                BinaryOperation::Add => Ok(Value::Number(
                    match (left, right) {
                        (Value::Number(left), Value::Number(right)) => left + right,
                        _ => return Err(RuntimeError::UnimplementedFeature),
                    }
                )),
                _ => Err(RuntimeError::UnimplementedFeature),
            }
        }
        Expression::CommandInvocation(command) => {
            match scope.get(*command) {
                Some(Variable { value: Value::Function(f), .. }) => f(scope),
                Some(Variable { value: Value::String(s), .. }) => Ok(Value::String(s.clone())),
                _ => Err(RuntimeError::UndefinedVariable),
            }
        }
        Expression::Number(num) => Ok(Value::Number(*num)),
        Expression::String(s) => Ok(Value::String(s.to_string())),
    }
}