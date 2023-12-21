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
      token::TokenKind::LParen => self.list(false),
      token::TokenKind::Identifier => {
        if t.lexeme == "'" {
          self.advance();
          self.list(true)
        }
        else {
          Some(expr::Expr::new(t.pos, expr::ExprData::Identifier(t.lexeme)))
        }
      },
      token::TokenKind::String => Some(expr::Expr::new(t.pos, expr::ExprData::String(t.lexeme))),
      token::TokenKind::Number => Some(expr::Expr::new(t.pos, expr::ExprData::Number(t.lexeme.parse().ok()?))),
      token::TokenKind::Keyword => {
        match t.lexeme.as_str() {
          "true" => Some(expr::Expr::new(t.pos, expr::ExprData::Bool(true))),
          "false" => Some(expr::Expr::new(t.pos, expr::ExprData::Bool(false))),
          "nil" => Some(expr::Expr::new(t.pos, expr::ExprData::Nil)),

          k => {
            util::print_error(&format!("Invalid keyword: '{}'", k), t.pos)?;
            None
          }
        }
      }

      _ => {
        util::print_error(&format!("Invalid expression: '{}'", t.lexeme), t.pos);
        None
      }
    }
  }

  fn list(&mut self, is_quote: bool) -> Option<expr::Expr> {
    let pos = self.peek()?.pos;
    let mut args: Vec<expr::Expr> = Vec::new();

    while !matches!(
      match self.peek() {
        Some(t) => t.kind,
        None => {
          util::print_error("Expected ')' after list", pos);
          return None;
        }
    }, token::TokenKind::RParen) {
      args.push(self.expr()?);
    }

    self.advance();
    Some(expr::Expr::new(pos, expr::ExprData::List(args, is_quote)))
  }
}
