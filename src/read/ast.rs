use crate::read::bytes::Span;

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