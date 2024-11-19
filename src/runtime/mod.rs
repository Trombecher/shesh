mod functions;

use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Color, Print, PrintStyledContent, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::read::ast::{BinaryOperation, Expression};
use crate::read::bytes::Span;

pub struct Variable {
    mutable: bool,
    value: Value,
}

#[derive(PartialEq)]
pub enum Value {
    Number(f64),
    Nil,
    Function(fn(args: SplitWhitespace) -> Result<Value, RuntimeError>),
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

pub type Scope = HashMap<String, Variable>;

pub fn new_root_scope() -> Scope {
    let mut scope = Scope::new();

    scope.insert("clear".to_string(), Variable {
        mutable: false,
        value: Value::Function(|_| {
            execute!(stdout(), Clear(ClearType::All), Clear(ClearType::Purge), MoveTo(0, 0))
                .unwrap();

            Ok(Value::Nil)
        })
    });

    scope
}

#[derive(Debug)]
#[repr(u8)]
pub enum RuntimeError {
    UndefinedVariable,
    UnimplementedFeature,
}

pub fn eval(
    root_scope: &mut Scope,
    root_expression: &Span<Expression>
) -> Result<Value, RuntimeError> {
    match &root_expression.value {
        Expression::Binary { left, operation, right } => {
            let left = eval(root_scope, left)?;
            let right = eval(root_scope, right)?;

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
            let f = match root_scope.get(*command) {
                Some(Variable { value: Value::Function(f), .. }) => f,
                _ => return Err(RuntimeError::UndefinedVariable),
            };

            f("".split_whitespace())
        }
        Expression::Number(num) => Ok(Value::Number(*num)),
        Expression::String(s) => Ok(Value::String(s.to_string())),
    }
}