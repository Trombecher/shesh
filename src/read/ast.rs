use crate::read::bytes::Span;

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponentiate,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Binary {
        left: Box<Span<Expression<'a>>>,
        operation: BinaryOperation,
        right: Box<Span<Expression<'a>>>,
    },
    CommandInvocation(&'a str),
    Number(f64),
    String(&'a str),
}