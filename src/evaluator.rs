use std::collections::HashMap;
use std::{rc::Rc, cell::RefCell};

use super::parser;

struct Env {
    func_table: HashMap<String, fn(Vec<String>)->()>,
}

struct Evaluator {
    env: Rc<RefCell<Env>>
}

impl Evaluator {
    pub fn new(&mut self) -> Self {
        let funcs: HashMap<String, fn(Vec<String>)->()> = HashMap::new();

        let env = Rc::new(RefCell::new(Env { func_table: funcs}));
        self.set("print".to_string(), |args| { syscall_stdout(&args[0])});

        Evaluator { env }
    }

    pub fn get(&self, key: &str) -> Option<fn(Vec<String>)->()> {
        self.env.borrow().func_table.get(key).copied()
    }

    pub fn set(&mut self, key: String, value: fn(Vec<String>)->()) {
        
        self.env.borrow_mut().func_table.insert(key, value);
    }

    pub fn apply(&self, ast: parser::Program) {
        self.eval_program(&ast);

        let main_func = self.get("main").unwrap();
        main_func(vec![]);
    }

    fn eval_program(&self, ast: &parser::Program) {
        for program in ast {
            self.eval_funcdef(program);
        }
    }

    fn eval_funcdef(&self, ast: &parser::FuncDef) {
        let name = &ast.0;
        let args = &ast.1;
        let statement_list = &ast.2;
        let func = move |statement_list| {self.eval_statement_list(statement_list)};

        self.set(name.value, func);
    }

    fn eval_statement_list(&self, ast: &parser::StatList) {

    }
}

fn syscall_stdout(text: &str) {

}