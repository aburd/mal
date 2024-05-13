use std::fmt::Display;

use crate::{environment::MalEnvironment, MalDataType, MalToken};
use regex::Regex;

#[derive(Debug)]
pub enum MalReaderError {
    LexingFailure(String),
    IllegalToken(String),
    IllegalString(String),
    IllegalSymbol(String),
    UnterminatedList,
}

impl Display for MalReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("MalReaderError: {:?}", self).as_str())
    }
}

pub type MalReaderResult<T> = Result<T, MalReaderError>;

impl MalDataType {
    pub fn to_string(&self) -> String {
        match self {
            MalDataType::Keyword(s) => format!(":{}", s[1..].to_owned()),
            MalDataType::Nil => "nil".to_owned(),
            MalDataType::Boolean(b) => b.to_string(),
            MalDataType::Int(n) => n.to_string(),
            MalDataType::String(s) => s.to_string(),
            MalDataType::Symbol(s) => s.to_string(),
            MalDataType::Vector(tokens) => {
                let content = tokens
                    .iter()
                    .map(|v| v.to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("[{}]", content)
            }
            MalDataType::List(tokens) => {
                let content = tokens
                    .iter()
                    .map(|v| v.to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("({})", content)
            }
        }
    }
}

impl MalToken {
    pub fn to_string(&self) -> String {
        match self {
            MalToken::OpenParen => "(".to_owned(),
            MalToken::CloseParen => ")".to_owned(),
            MalToken::OpenBracket => "[".to_owned(),
            MalToken::CloseBracket => "]".to_owned(),
            MalToken::Data(d) => d.to_string(),
        }
    }

    pub fn from_str(s: &str) -> MalReaderResult<MalToken> {
        match s {
            "(" => Ok(MalToken::OpenParen),
            ")" => Ok(MalToken::CloseParen),
            "[" => Ok(MalToken::OpenBracket),
            "]" => Ok(MalToken::CloseBracket),
            "nil" => Ok(MalToken::Data(MalDataType::Nil)),
            "true" => Ok(MalToken::Data(MalDataType::Boolean(true))),
            "false" => Ok(MalToken::Data(MalDataType::Boolean(false))),
            s if s.starts_with(":") => {
                if s.starts_with("::") {
                    return Err(MalReaderError::IllegalToken(s.to_owned()));
                }

                Ok(MalToken::Data(MalDataType::Keyword(s.to_owned())))
            }
            s if s.chars().all(|c| c.is_digit(10)) => Ok(MalToken::Data(MalDataType::Int(
                s.parse::<usize>().unwrap(),
            ))),
            s if s.starts_with("\"") => {
                if s.len() < 2 || !s.ends_with("\"") {
                    return Err(MalReaderError::IllegalString(s.to_owned()));
                }

                Ok(MalToken::Data(MalDataType::String(s.to_string())))
            }
            _ => {
                // Symbols must not contain certain characters
                if s.contains("\"") {
                    return Err(MalReaderError::IllegalSymbol(s.to_owned()));
                }
                // Illegal symbol starting character should panic
                if s.chars().next().unwrap().is_digit(10) {
                    return Err(MalReaderError::IllegalSymbol(s.to_owned()));
                }

                return Ok(MalToken::Data(MalDataType::Symbol(s.to_owned())));
            }
        }
    }
}

#[derive(Debug)]
struct Reader {
    tokens: Vec<MalToken>,
    pos: usize,
}

impl Reader {
    fn new<'a>(tokens: Vec<MalToken>) -> Self {
        Reader { tokens, pos: 0 }
    }
}

impl Reader {
    pub fn peek(&self) -> MalReaderResult<&MalToken> {
        if let Some(token) = self.tokens.get(self.pos) {
            return Ok(token);
        }
        Err(MalReaderError::UnterminatedList)
    }

    pub fn read_list(&mut self) -> MalReaderResult<MalToken> {
        let mut tokens = vec![];

        while let Ok(token) = self.read_form() {
            let is_list_end = token == MalToken::CloseParen;
            if is_list_end {
                return Ok(MalToken::Data(MalDataType::List(tokens)));
            }
            tokens.push(token);
            self.pos += 1;
        }

        Err(MalReaderError::UnterminatedList)
    }

    pub fn read_vector(&mut self) -> MalReaderResult<MalToken> {
        let mut tokens = vec![];

        while let Ok(token) = self.read_form() {
            let is_list_end = token == MalToken::CloseBracket;

            if is_list_end {
                return Ok(MalToken::Data(MalDataType::Vector(tokens)));
            }
            tokens.push(token);
            self.pos += 1;
        }

        Err(MalReaderError::UnterminatedList)
    }

    pub fn read_atom(&self) -> MalReaderResult<MalToken> {
        Ok(self.peek()?.clone())
    }

    pub fn read_form(&mut self) -> MalReaderResult<MalToken> {
        let token = self.peek()?;
        if token == &MalToken::OpenParen {
            self.pos += 1;
            self.read_list()
        } else if token == &MalToken::OpenBracket {
            self.pos += 1;
            self.read_vector()
        } else {
            self.read_atom()
        }
    }
}

fn lexer(s: &str) -> MalReaderResult<Vec<&str>> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
        .map_err(|e| MalReaderError::LexingFailure(e.to_string()))?;

    Ok(re
        .captures_iter(s.trim())
        .map(|c| {
            let (_, [s]) = c.extract();
            s
        })
        .filter(|s| !s.is_empty())
        .collect())
}

fn tokenize(lexemes: &[&str]) -> MalReaderResult<Vec<MalToken>> {
    let mut tokens = vec![];
    for l in lexemes {
        let token = MalToken::from_str(l)?;
        tokens.push(token);
    }

    Ok(tokens)
}

pub fn read_str(s: &str, mal_env: &MalEnvironment) -> MalReaderResult<MalDataType> {
    let lexemes = lexer(s)?;
    println!("lexemes: {:?}", lexemes);
    let tokens = tokenize(&lexemes)?;
    println!("tokens: {:?}", tokens);
    let mut reader = Reader::new(tokens);

    match reader.read_form()? {
        MalToken::Data(d) => {
            println!("d: {:?}", d);
            Ok(d)
        }
        _ => Err(MalReaderError::UnterminatedList),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_tokenize() -> MalReaderResult<()> {
        let lexemes = lexer("  (  + 2   ( *  3   4)   )   ")?;
        assert_eq!(lexemes, vec!["(", "+", "2", "(", "*", "3", "4", ")", ")"]);
        Ok(())
    }

    #[test]
    fn can_get_mal_tokens() -> MalReaderResult<()> {
        let mal = MalEnvironment::new();
        let mal_list = read_str("(+ 2 3 nil false)", &mal)?;
        assert_eq!(
            mal_list,
            MalDataType::List(vec![
                MalToken::Data(MalDataType::Symbol("+".to_owned())),
                MalToken::Data(MalDataType::Int(2)),
                MalToken::Data(MalDataType::Int(3)),
                MalToken::Data(MalDataType::Nil),
                MalToken::Data(MalDataType::Boolean(false)),
            ])
        );

        Ok(())
    }

    #[test]
    fn can_render_s() -> MalReaderResult<()> {
        let mal = MalEnvironment::new();
        let mal_list = read_str(" ( + 2   3 )  ", &mal)?;
        assert_eq!(mal_list.to_string(), "(+ 2 3)".to_owned());
        Ok(())
    }
}
