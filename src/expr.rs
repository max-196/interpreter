use crate::token::Token;

pub trait Visitor<'a, T> {
    fn visit_binary  (&mut self, bin: &Bin<'a>) -> T;
    fn visit_grouping(&mut self, grp: &Grp<'a>) -> T;
    fn visit_literal (&mut self, lit: &Lit<'a>) -> T;
    fn visit_unary   (&mut self, unr: &Unr<'a>) -> T;
}

pub enum ExprType<'a> {
    Binary(Bin<'a>),
    Grouping(Grp<'a>),
    Literal(Lit<'a>),
    Unary(Unr<'a>),
}

pub trait Expr<'a, T> {
    fn accept(&self, visitor: &mut dyn Visitor<'a, T>) -> T;
}

impl <'a, T> Expr<'a, T> for ExprType<'a> {
    fn accept(&self, visitor: &mut dyn Visitor<'a, T>) -> T {
        match self {
            ExprType::Binary(b) => visitor.visit_binary(b),
            ExprType::Grouping(g) => visitor.visit_grouping(g),
            ExprType::Literal(l) => visitor.visit_literal(l),
            ExprType::Unary(u) => visitor.visit_unary(u),
            _ => panic!(),
        }
    }
}

pub struct Bin<'a> {
    pub left: &'a ExprType<'a>,
    pub operator: Token<'a>,
    pub right: &'a ExprType<'a>,
}

pub struct Grp<'a> {
    pub expr: &'a ExprType<'a>,
}

pub struct Lit<'a> {
    pub value: &'a str,
}

pub struct Unr<'a> {
    pub operator: Token<'a>,
    pub right: &'a ExprType<'a>,
}
