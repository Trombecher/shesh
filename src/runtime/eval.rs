use std::io::stdout;
use std::process::Command;
use crate::read::ast::{BinaryOperation, Expression};
use crate::read::bytes::Span;
use crate::runtime::scope::Scope;
use crate::runtime::{RuntimeError, Value, Variable};
use crate::runtime::resolve::search_program_in_path;

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
                        _ => return Err(RuntimeError::UnimplementedError),
                    }
                )),
                BinaryOperation::Multiply => Ok(Value::Number(
                    match (left, right) {
                        (Value::Number(left), Value::Number(right)) => left * right,
                        _ => return Err(RuntimeError::UnimplementedError)
                    }
                )),
                BinaryOperation::Divide => Ok(Value::Number(
                    match (left, right) {
                        (Value::Number(left), Value::Number(right)) => left / right,
                        _ => return Err(RuntimeError::UnimplementedError)
                    }
                )),
                _ => Err(RuntimeError::UnimplementedFeature),
            }
        }
        Expression::CommandInvocation(command) => {
            match Command::new(command)
                .stdout(stdout())
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        Ok(Value::Nil)
                    } else {
                        Ok(Value::Number(output.status.code().unwrap_or(0) as f64))
                    }
                }
                Err(_) => {
                    match scope.get(*command) {
                        Some(Variable { value: Value::Function(f), .. }) => f(scope),
                        Some(Variable { value: Value::String(s), .. }) => Ok(Value::String(s.clone())),
                        _ => Err(RuntimeError::UndefinedVariable),
                    }
                }
            }
        }
        Expression::Number(num) => Ok(Value::Number(*num)),
        Expression::String(s) => Ok(Value::String(s.to_string())),
    }
}