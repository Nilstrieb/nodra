use std::ops::Range;

type Span = Range<usize>;
type Ident = String;

pub enum Item {
    Fn(Fn),
}

pub enum FnKind {
    Fn,
    Task,
}

struct Fn {
    pub kind: FnKind,
    pub span: Span,
}

pub enum Expr {
    Int(u64),
    Ident(Ident),
    Binary(BinaryOp, Box<Expr>, Box<Expr>, Span),
    Assign(Ident, Box<Expr>, Span),
}

pub enum BinaryOp {
    Add,
    Sub,
}
