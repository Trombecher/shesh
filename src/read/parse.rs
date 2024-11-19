use std::ops::Range;
use crate::read::ast::{BinaryOperation, Expression};
use crate::read::bp;
use crate::read::buffered::Buffered;
use crate::read::bytes::Span;
use crate::read::tokens::Token;

pub fn parse<'a>(iter: &mut Buffered<'a>, min_bp: u8) -> Result<Span<Expression<'a>>, ()> {
    let Span {
        range: Range {
            start: start_index,
            end: end_index
        },
        value
    } = iter.next()?;

    let mut first_term = Span {
        value: match value {
            Token::Identifier(id) => Expression::CommandInvocation(id),
            Token::Number(num) => Expression::Number(num),
            Token::String(string) => Expression::String(string),
            _ => return Err(()),
        },
        range: Range {
            start: start_index,
            end: end_index
        }
    };

    macro_rules! op {
        ($op: expr, $bp: expr) => {{
            if $bp.0 < min_bp {
                break;
            }

            iter.next()?;

            let right = parse(iter, $bp.1)?;

            (
                right.range.end,
                Expression::Binary {
                    left: Box::new(first_term),
                    operation: $op,
                    right: Box::new(right)
                }
            )
        }};
    }

    loop {
        let token = iter.peek()?;

        let (end, value) = match &token.value {
            Token::Plus => op!(BinaryOperation::Add, bp::ADDITIVE),
            Token::Star => op!(BinaryOperation::Multiply, bp::MULTIPLICATIVE),
            Token::Slash => op!(BinaryOperation::Divide, bp::MULTIPLICATIVE),
            Token::EndOfInput => break,
            _ => return Err(())
        };

        first_term = Span {
            value,
            range: start_index..end,
        };
    }

    Ok(first_term)
}