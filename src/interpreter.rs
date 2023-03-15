use super::lexer::Lexer;

pub struct Interpreter {
    error_handler: ErrorHandler,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { error_handler: ErrorHandler::new()}
    }

    pub fn run_src(&mut self, path: &str) {
        let path = std::path::Path::new(path);
        let src = std::fs::read_to_string(path).unwrap();
        self.run(&src);
        if self.error_handler.error {/*panic!()*/}
    }

    pub fn run_prompt(&mut self) {
        let mut input = String::new();

        loop {
            print!("> ");
            use std::io::Write;
            std::io::stdout().flush().unwrap();
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            self.run(input.trim());
            self.error_handler.error = false;
        }
    }

    fn run(&mut self, src: &str) {
        let mut lexer = Lexer::new(src);
        lexer.scan_tokens(&mut self.error_handler);

        for token in lexer.tokens {
            println!("{}", token.to_string());
        }
    }
}

pub struct ErrorHandler {
    pub error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {error: false}
    }

    pub fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
        self.error = true;
    }

    pub fn report(&mut self, line: usize, location: &str, msg: &str) {
        eprintln!("[line {line}] Error {location}: {msg}");
    }
}