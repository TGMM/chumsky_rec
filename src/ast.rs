#[derive(Debug)]
pub enum Expr<'a> {
    Digit(&'a str),
    List(Vec<List<'a>>),
}

#[derive(Debug)]
pub enum List<'a> {
    Digit(&'a str),
    Expr(Vec<Expr<'a>>),
}
