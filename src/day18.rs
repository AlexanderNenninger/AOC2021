#![allow(unused)]
use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, ops::Rem, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserToken {
    BrOpen,
    BrClose,
    Sep,
    Lit(isize),
}

impl ParserToken {
    fn tokenize(s: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        lazy_static! {
            static ref RE_TOKEN: Regex = Regex::new(r"\[|\]|-?\d+|,").unwrap();
        }
        let mut tokens = vec![];
        for re_match in RE_TOKEN.find_iter(s) {
            let token = match re_match.as_str() {
                "[" => Self::BrOpen,
                "]" => Self::BrClose,
                "," => Self::Sep,
                s => Self::Lit(s.parse::<isize>()?),
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SValue {
    Num(Box<SNumber>),
    Lit(isize),
}

impl SValue {
    fn from_tokens(tokens: &[ParserToken]) -> Result<Self, Box<dyn Error>> {
        if tokens.is_empty() {
            return Err(Box::new("Token stream empty."));
        }

        match token {
            ParserToken::Lit(n) => Self::Lit(n),
            ParserToken::BrOpen => Self::from_tokens(t),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SNumber(SValue, SValue);

impl SNumber {}

impl FromStr for SNumber {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = ParserToken::tokenize(s)?;

        let mut nesting_depth = 0;
        for token in tokens.iter() {
            match token {
                ParserToken::BrOpen => SValue,
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tokenize() {
        let data = "[[1,2],-133]";
        let tokens = ParserToken::tokenize(data).unwrap();
        assert_eq!(
            tokens,
            vec![
                ParserToken::BrOpen,
                ParserToken::BrOpen,
                ParserToken::Lit(1),
                ParserToken::Sep,
                ParserToken::Lit(2),
                ParserToken::BrClose,
                ParserToken::Sep,
                ParserToken::Lit(-133),
                ParserToken::BrClose
            ]
        )
    }
}
