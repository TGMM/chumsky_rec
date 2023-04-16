use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map, multi::many0,
    sequence::delimited, IResult,
};

use crate::ast::{Expr, List};

pub fn expr_parser<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let digit = map(digit1, |d| Expr::Digit(d));
    let list = map(many0(list_parser), |ls| Expr::List(ls));

    delimited(tag("("), alt((digit, list)), tag(")"))(input)
}

pub fn list_parser<'a>(input: &'a str) -> IResult<&'a str, List<'a>> {
    let digit = map(digit1, |d| List::Digit(d));
    let expr = map(many0(expr_parser), |es| List::Expr(es));

    delimited(tag("["), alt((digit, expr)), tag("]"))(input)
}
