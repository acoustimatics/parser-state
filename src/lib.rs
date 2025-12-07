use anyhow::anyhow;
use std::cmp::PartialEq;
use std::fmt::Display;

pub trait Token {
    type Tag: Display + PartialEq;
    fn tag(&self) -> Self::Tag;
    fn lexeme(&self) -> &str;
}

pub trait Lexer {
    type Token: Token;
    fn next_token(&mut self) -> anyhow::Result<Self::Token>;
}

pub struct ParserState<L: Lexer> {
    lexer: L,
    pub current: L::Token,
}

impl<L: Lexer> ParserState<L> {
    pub fn new(mut lexer: L) -> anyhow::Result<Self> {
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    pub fn advance(&mut self) -> anyhow::Result<()> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    pub fn advance_keep_current(&mut self) -> anyhow::Result<L::Token> {
        let kept = std::mem::replace(&mut self.current, self.lexer.next_token()?);
        Ok(kept)
    }

    pub fn matches(&mut self, expected_tag: <L::Token as Token>::Tag) -> anyhow::Result<bool> {
        let is_match = self.current.tag() == expected_tag;
        if is_match {
            self.advance()?;
        }
        Ok(is_match)
    }

    pub fn expect(&mut self, expected_tag: <L::Token as Token>::Tag) -> anyhow::Result<()> {
        if self.current.tag() == expected_tag {
            self.advance()?;
            Ok(())
        } else {
            Err(anyhow!(
                "expected `{}` but got `{}`",
                expected_tag,
                self.current.lexeme()
            ))
        }
    }

    pub fn expect_lexeme(&mut self, tag: <L::Token as Token>::Tag) -> anyhow::Result<String> {
        if self.current.tag() == tag {
            let token = self.advance_keep_current()?;
            Ok(token.lexeme().to_string())
        } else {
            Err(anyhow!(
                "expected `{}` but found `{}`",
                tag,
                self.current.lexeme()
            ))
        }
    }
}
