use super::scanner;

type Program = Vec<FuncDef>;
type FuncDef = (scanner::Token, Vec<scanner::Token>, FuncArgs, StatList);
type FuncArgs = Vec<scanner::Token>;
type StatList = Vec<Statement>;
type Statement = Vec<CallFunc>;
type CallFunc = (scanner::Token, Vec<scanner::Token>, Vec<scanner::Token>);

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

    fn take(&mut self, kind: scanner::TokenKind) -> scanner::Token {
        let token = self.scanner.peek();

        if self.scanner.is_not_end() {
            self.scanner.next();
        }

        if token.kind != kind {
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
        let func_def = self.take(scanner::TokenKind::FuncDef);
        let ident = self.take(scanner::TokenKind::Ident);
        self.take(scanner::TokenKind::ParenthesOpen);
        let func_args: FuncArgs = self.func_args();
        self.take(scanner::TokenKind::ParenthesClose);
        self.take(scanner::TokenKind::Begin);
        let stat_list: StatList = self.stat_list();
        self.take(scanner::TokenKind::End);

        (
            func_def,
            vec![ident],
            func_args,
            stat_list,
        )
    }

    fn func_args(&self) -> FuncArgs {
        let func_args: FuncArgs = Vec::new();
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
        let name = self.take(scanner::TokenKind::Ident);
        statement.push(self.call_func(name));
        self.take(scanner::TokenKind::Semicolon);

        statement
    }

    fn call_func(&mut self, name: scanner::Token) -> CallFunc {
        let call_func = scanner::Token {kind: scanner::TokenKind::CallFunc, value: "".to_string()};
        self.take(scanner::TokenKind::ParenthesOpen);
        let string = self.take(scanner::TokenKind::String);
        self.take(scanner::TokenKind::ParenthesClose);

        (
            call_func,
            vec![name],
            vec![string]
        )
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