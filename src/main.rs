mod ast;
mod chumsky;
mod nom;
use ::chumsky::Parser;

use crate::chumsky::expr_parser as chumsky_expr_parser;
use crate::nom::expr_parser as nom_expr_parser;

fn main() {
    let input = "([10])";
    let nom_expr = nom_expr_parser(input).unwrap().1;
    let chumsky_expr = chumsky_expr_parser().parse(input).into_result().unwrap();

    dbg!(nom_expr);
    dbg!(chumsky_expr);
}
