use chumsky::{prelude::*, Parser};

use crate::{
    ast::{Expr, Item},
    lexer::Token,
};

fn expr() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let value = select! {
            Token::Int(int) => Expr::Int(int),
            Token::Ident(ident) => Expr::Ident(ident),
        }
        .boxed();

        let ident = select! {
            Token::Ident(ident) => ident
        }
        .boxed();

        let let_expr = just(Token::Let)
            .ignore_then(ident.clone())
            .then_ignore(just(Token::Eq))
            .then(expr.clone())
            .then_ignore(just(Token::Semi))
            .map_with_span(|(name, value): (String, Expr), span| {
                Expr::Assign(name, Box::new(value), span)
            })
            .boxed();

        value.or(let_expr)
    })
    .boxed()
}

fn item() -> impl Parser<Token, Item, Error = Simple<Token>> {
    todo()
}
