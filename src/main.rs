mod scanner;
mod parser;

fn main() {
    let text = r#"fn main2() { print("test"); }"#;
    let scanner = scanner::Scanner::new(text);
    let mut parser = parser::Parser::new(scanner);
    let ast = parser.parse();
    println!("{:?}", ast);
}
