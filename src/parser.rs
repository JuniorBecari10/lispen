use crate::{token, expr, util};

pub struct Parser {
  input: Vec<token::Token>,
  current: usize,
}

impl Parser {
  pub fn new(input: Vec<token::Token>) -> Self {
    Self {
      input,
      current: 0,
    }
  }

  pub fn parse(&mut self) -> (Vec<expr::Expr>, bool) {
    let mut exprs: Vec<expr::Expr> = Vec::new();
    let mut had_error = false;

    while !self.is_at_end() {
      match self.expr() {
        Some(e) => exprs.push(e),
        None => { had_error = true; }
      }
    }

    (exprs, had_error)
  }

  // ---

  fn is_at_end(&self) -> bool {
    self.current >= self.input.len()
  }

  fn peek(&self) -> Option<token::Token> {
    self.input.get(self.current).cloned()
  }

  fn advance(&mut self) -> Option<token::Token> {
    let t = self.peek();
    self.current += 1;

    t
  }

  // ---

  fn expr(&mut self) -> Option<expr::Expr> {
    let t = self.peek()?;

    match self.advance()?.kind {
      token::TokenKind::LParen => self.composite(),
      token::TokenKind::Identifier => Some(expr::Expr::new(t.pos, expr::ExprData::Identifier(t.lexeme))),
      token::TokenKind::String => Some(expr::Expr::new(t.pos, expr::ExprData::String(t.lexeme))),
      token::TokenKind::Number => Some(expr::Expr::new(t.pos, expr::ExprData::Number(t.lexeme.parse().ok()?))),

      _ => {
        util::print_error(&format!("Invalid expression: '{}'", t.lexeme), t.pos);
        None
      }
    }
  }

  fn composite(&mut self) -> Option<expr::Expr> {
    let t = self.peek()?;
    if !matches!(self.advance()?.kind, token::TokenKind::Identifier) {
      util::print_error(&format!("Expected identifier in composite expression, got '{}'", t.lexeme), t.pos);
      return None;
    }

    let mut args: Vec<expr::Expr> = Vec::new();
    while !matches!(self.peek()?.kind, token::TokenKind::RParen) {
      args.push(self.expr()?);
    }

    Some(expr::Expr::new(t.pos, expr::ExprData::Composite(expr::CompositeExpr {
      name: t.lexeme,
      args,
    })))
  }
}
