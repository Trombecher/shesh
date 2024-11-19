mod scope;
mod eval;
mod resolve;

use crate::runtime::scope::Scope;
use crossterm::style::{Color, SetForegroundColor};
use std::fmt::{Display, Formatter};

pub use scope::*;
pub use eval::*;

#[derive(Debug)]
pub struct Variable {
    pub mutable: bool,
    pub value: Value,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Nil,
    Function(fn(scope: &mut Scope) -> Result<Value, RuntimeError>),
    String(String),
}

impl Value {
    pub fn get_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(&s),
            _ => None
        }
    }
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
    UnimplementedError
}