use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType {
    FuncDef,
    ParenthesOpen,
    ParenthesClose,
    Begin,
    End,
    Semicolon,
    String,
    Ident,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: String
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

    pub fn next(&mut self) -> &Token {
        self.pos += 1;
        self.tokens.get(self.pos).unwrap()
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap()
    }
}

fn tokenize(word: &str) -> Token { 
    match word {
        "fn" => Token {kind: TokenType::FuncDef, value: word.to_string()},
        "(" => Token {kind: TokenType::ParenthesOpen, value: word.to_string()},
        ")" => Token {kind: TokenType::ParenthesClose, value: word.to_string()},
        "{" => Token {kind: TokenType::Begin, value: word.to_string()},
        "}" => Token {kind: TokenType::End, value: word.to_string()},
        ";" => Token {kind: TokenType::Semicolon, value: word.to_string()},
        x =>{
            match x.chars().next().unwrap() {
                '"' => Token {kind: TokenType::String, value: word.to_string()},
                _ => Token {kind: TokenType::Ident, value: word.to_string()}
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
            fn main2() {
                print("Hello World");
            }
        "#;
        let mut scanner = Scanner::new(text);

        assert!(scanner.is_not_end());

        // @TODO eq traitを実装する
        let mut token: &Token;
        token = scanner.peek();
        
        assert_eq!(token.kind, TokenType::FuncDef);
        assert_eq!(token.value, "fn".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::Ident);
        assert_eq!(token.value, "main2".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::ParenthesOpen);
        assert_eq!(token.value, "(".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::ParenthesClose);
        assert_eq!(token.value, ")".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::Begin);
        assert_eq!(token.value, "{".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::Ident);
        assert_eq!(token.value, "print".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::ParenthesOpen);
        assert_eq!(token.value, "(".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::String);
        assert_eq!(token.value, "\"Hello World\"".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::ParenthesClose);
        assert_eq!(token.value, ")".to_string());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::Semicolon);
        assert_eq!(token.value, ";".to_string());

        assert!(scanner.is_not_end());

        token = scanner.next();
        
        assert_eq!(token.kind, TokenType::End);
        assert_eq!(token.value, "}".to_string());

        assert!(!scanner.is_not_end());
    }
}