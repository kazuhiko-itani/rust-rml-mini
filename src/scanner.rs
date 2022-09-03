use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    FuncDef,
    ParenthesOpen,
    ParenthesClose,
    Begin,
    End,
    Semicolon,
    String,
    Ident,
    CallFunc,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<Token>,
    pos: usize
}

impl Scanner {
    pub fn new(text: &str) -> Self {
        Scanner { tokens: split(text), pos: 0}
    }

    pub fn is_not_end(&self) -> bool {
        self.pos < self.tokens.len() -1
    }

    pub fn next(&mut self) -> Token {
        self.pos += 1;
        self.tokens.get(self.pos).unwrap().clone()
    }

    pub fn peek(&self) -> Token {
        self.tokens.get(self.pos).unwrap().clone()
    }
}

fn tokenize(word: &str) -> Token { 
    match word {
        "fn" => Token {kind: TokenKind::FuncDef, value: word.to_string()},
        "(" => Token {kind: TokenKind::ParenthesOpen, value: word.to_string()},
        ")" => Token {kind: TokenKind::ParenthesClose, value: word.to_string()},
        "{" => Token {kind: TokenKind::Begin, value: word.to_string()},
        "}" => Token {kind: TokenKind::End, value: word.to_string()},
        ";" => Token {kind: TokenKind::Semicolon, value: word.to_string()},
        x =>{
            match x.chars().next().unwrap() {
                '"' => Token {kind: TokenKind::String, value: word.to_string()},
                _ => Token {kind: TokenKind::Ident, value: word.to_string()}
            }


        } 
    }
}

fn split(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let text_chars: Vec<char> = text.chars().collect();
    let mut idx: usize = 0;

    for (i, c) in text_chars.iter().enumerate() {
        if idx > i {
            continue
        }

        let mut str: Vec<char> = Vec::new();

        if text_chars[idx] == '"' {
            str.push(*c);
            idx += 1;

            while text_chars[idx] != '"' {
                str.push(text_chars[idx]);
                idx += 1;
            }

            str.push('"');
            idx += 1;
            let token = tokenize(&str.iter().collect::<String>());
            tokens.push(token);
        } else if Regex::new(r"\d").unwrap().is_match(&c.to_string()) {
            while idx < text_chars.len() && Regex::new(r"\d").unwrap().is_match(&text_chars[idx].to_string()) {
                str.push(text_chars[idx]);
                idx += 1;
            }
            idx += 1;
            let token = tokenize(&str.iter().collect::<String>());
            tokens.push(token);
        } else if Regex::new(r"[a-z|A-Z]").unwrap().is_match(&c.to_string()) {
            while idx < text_chars.len() && Regex::new(r"[a-z|A-Z]|[0-9]").unwrap().is_match(&text_chars[idx].to_string()) {
                str.push(text_chars[idx]);
                idx += 1;
            }

            let token = tokenize(&str.iter().collect::<String>());
            tokens.push(token);
        } else if text_chars[idx] == ' ' || text_chars[idx] == '\n' {
            idx += 1;
        } else {
            let token = tokenize(&c.to_string());
            tokens.push(token);
            idx += 1;
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn scanner_hello_world() {
        let text = r#"
            fn main() {
                print("Hello World");
            }
        "#;
        let mut scanner = Scanner::new(text);

        assert!(scanner.is_not_end());

        assert_eq!(
            scanner.peek(),
            Token {kind: TokenKind::FuncDef, value: "fn".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::Ident, value: "main".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::ParenthesOpen, value: "(".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::ParenthesClose, value: ")".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::Begin, value: "{".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::Ident, value: "print".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::ParenthesOpen, value: "(".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::String, value: "\"Hello World\"".to_string()},
        );

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::ParenthesClose, value: ")".to_string()},
        );
        
        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::Semicolon, value: ";".to_string()},
        );

        assert!(scanner.is_not_end());

        assert_eq!(
            scanner.next(),
            Token {kind: TokenKind::End, value: "}".to_string()},
        );

        assert!(!scanner.is_not_end());
    }
}