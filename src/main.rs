mod scanner;

fn main() {
    let text = r#"fn main2() { print("test")}"#;
    let res = scanner::Scanner::new(text);
    println!("{:?}", res);
}
