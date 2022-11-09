use super::scanner;

pub type Program = Vec<FuncDef>;
pub type FuncDef = (scanner::Token, Vec<scanner::Token>, FuncArgs, StatList);
pub type FuncArgs = Vec<scanner::Token>;
pub type StatList = Vec<Statement>;
pub type Statement = Vec<CallFunc>;
pub type CallFunc = Vec<scanner::Token>;
pub type Factor = Vec<scanner::Token>;
pub type Literal = Vec<scanner::Token>;
#[derive(Debug)]
pub struct Parser {
    scanner: scanner::Scanner
}

impl Parser {
    pub fn new(scanner: scanner::Scanner) -> Parser {
        Parser { scanner }
    }

    pub fn parse(&mut self) -> Program {
        self.program()
    }

    fn is_match(&self, kind: scanner::TokenKind) -> bool {
        let token = self.scanner.peek();
        token.kind == kind
    }

    fn take(&mut self, kind: Vec<scanner::TokenKind>) -> scanner::Token {
        let token = self.scanner.peek();

        if self.scanner.is_not_end() {
            self.scanner.next();
        }

        let mut result = false;
        for k in kind {
            if token.kind == k {
                result = true;
            }
        }

        if !result {
            panic!("Syntax Error: expect.kind={:?}, actual.kind={:?}, token.value={:?}", kind, token.kind, token.value);
        }

        token
    }

    fn program(&mut self) -> Program {
        let mut program: Program = Vec::new();
        while self.is_match(scanner::TokenKind::FuncDef) {
            program.push(self.func_def());
        }

        program
    }

    fn func_def(&mut self) -> FuncDef {
        let func_def = self.take(vec![scanner::TokenKind::FuncDef]);
        let ident = self.take(vec![scanner::TokenKind::Ident]);
        self.take(vec![scanner::TokenKind::ParenthesOpen]);
        let func_args: FuncArgs = self.func_args();
        self.take(vec![scanner::TokenKind::ParenthesClose]);
        self.take(vec![scanner::TokenKind::Begin]);
        let stat_list: StatList = self.stat_list();
        self.take(vec![scanner::TokenKind::End]);

        (
            func_def,
            vec![ident],
            func_args,
            stat_list,
        )
    }

    fn func_args(&mut self) -> FuncArgs {
        let mut func_args: FuncArgs = Vec::new();
        
        while self.is_match(scanner::TokenKind::Ident) {
            func_args.push(self.take(vec![scanner::TokenKind::Ident]));
            if self.is_match(scanner::TokenKind::Comma) {
                self.take(vec![scanner::TokenKind::Comma]);
            }
        }

        func_args
    }

    fn stat_list(&mut self) -> StatList {
        let mut stat_list: StatList = Vec::new();
        while self.is_match(scanner::TokenKind::Ident) {
            stat_list.push(self.statement());
        }

        stat_list
    }

    fn statement(&mut self) -> Statement {
        let mut statement: Statement = vec!();

        if self.is_match(scanner::TokenKind::While) {
            statement.push(self.call_while());
        }

        let name = self.take(vec![scanner::TokenKind::Ident]);
        statement.push(self.call_func(name));
        self.take(vec![scanner::TokenKind::Semicolon]);

        statement
    }

    fn call_func(&mut self, name: scanner::Token) -> CallFunc {
        let mut call_func: CallFunc = vec!(); 

        call_func.push(scanner::Token {kind: scanner::TokenKind::CallFunc, value: "".to_string()});
        call_func.push(name);
        self.take(vec![scanner::TokenKind::ParenthesOpen]);
        for arg in self.call_args() {
            call_func.push(arg);
        }
        self.take(vec![scanner::TokenKind::ParenthesClose]);

        call_func
    }

    fn call_args(&mut self) -> Vec<scanner::Token> {

    }

    fn call_while(&mut self) {}

    fn relation(&mut self) {}

    fn expr(&mut self) {}

    fn term(&mut self) {
        let mut term = self.factor();


        while self.is_match(scanner::TokenKind::OpMul) {
            let token = self.take(vec![scanner::TokenKind::OpMul]);

            match token.value.chars().nth(0).unwrap() {
                '*'=> {
                    term = [scanner::Token { kind: scanner::TokenKind::Mul, value: '*'.to_string()}, term, self.factor()];
                }
            }
        }
    }

    fn factor(&mut self) -> Factor {
        if self.is_match(scanner::TokenKind::ParenthesOpen) {
            self.take(vec![scanner::TokenKind::ParenthesOpen]);
            let factor = self.expr();
            self.take(vec![scanner::TokenKind::ParenthesClose]);

            return factor
        }

        self.literal()
    }

    fn literal(&mut self) -> Literal {
        let literalToken = self.take(vec![scanner::TokenKind::Int, scanner::TokenKind::String, scanner::TokenKind::Bool, scanner::TokenKind::Ident]);
        if literalToken.kind == scanner::TokenKind::Ident && self.is_match(scanner::TokenKind::ParenthesOpen) {
            return self.call_func(literalToken)
        }

        vec![literalToken]
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn parse_hello_world() {
        let text = r#"
            fn main() {
                print("Hello World");
            }
        "#;

        let scanner = scanner::Scanner::new(text);
        let mut parser = Parser::new(scanner);
        let actual = parser.parse();

        assert_eq!(actual, vec![
            (
                scanner::Token {kind: scanner::TokenKind::FuncDef, value: "fn".to_string()},
                vec![scanner::Token {kind: scanner::TokenKind::Ident, value: "main".to_string()}],
                vec![],
                vec![
                     vec![
                        (
                            scanner::Token {kind: scanner::TokenKind::CallFunc, value: "".to_string()},
                            vec![scanner::Token {kind: scanner::TokenKind::Ident, value: "print".to_string()}],
                            vec![scanner::Token {kind: scanner::TokenKind::String, value: "\"Hello World\"".to_string()}],
                        )
                     ]
                ]
            )
        ]);
    }
}