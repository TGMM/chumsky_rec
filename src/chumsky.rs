use chumsky::{
    extra,
    input::{SliceInput, ValueInput},
    prelude::Simple,
    primitive::{just, one_of},
    span::SimpleSpan,
    IterParser, Parser,
};

use crate::ast::{Expr, List};

pub fn digits_parser<
    'a,
    I: ValueInput<'a, Token = char, Span = SimpleSpan> + SliceInput<'a, Slice = &'a str>,
>() -> impl Parser<'a, I, &'a str, extra::Err<Simple<'a, char>>> + Clone {
    one_of('0'..='9').repeated().slice()
}

pub fn expr_parser<
    'a,
    I: ValueInput<'a, Token = char, Span = SimpleSpan> + SliceInput<'a, Slice = &'a str>,
>() -> impl Parser<'a, I, Expr<'a>, extra::Err<Simple<'a, char>>> + Clone {
    let digit = digits_parser().map(|d| Expr::Digit(d));
    let list = list_parser()
        .repeated()
        .collect::<Vec<_>>()
        .map(|l| Expr::List(l));

    digit.or(list).delimited_by(just("("), just(")"))
}

pub fn list_parser<
    'a,
    I: ValueInput<'a, Token = char, Span = SimpleSpan> + SliceInput<'a, Slice = &'a str>,
>() -> impl Parser<'a, I, List<'a>, extra::Err<Simple<'a, char>>> + Clone {
    let digit = digits_parser().map(|d| List::Digit(d));
    let expr = expr_parser()
        .repeated()
        .collect::<Vec<_>>()
        .map(|e| List::Expr(e));

    digit.or(expr).delimited_by(just("["), just("]"))
}
