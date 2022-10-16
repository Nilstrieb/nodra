#[derive(Debug, Clone, PartialEq, Eq, Hash, logos::Logos)]
pub enum Token {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("=")]
    Eq,
    #[token(";")]
    Semi,
    #[token("(")]
    POpen,
    #[token(")")]
    PClose,
    #[token("{")]
    BOpen,
    #[token("}")]
    BClose,
    #[regex(r"\d+", |lex| lex.slice().parse())]
    Int(u64),
    #[regex(r"[a-zA-Z_]\w+", |lex| lex.slice().to_string())]
    Ident(String),
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("task")]
    Task,
    #[error]
    Error,
}
