use interpreter::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut interpreter = Interpreter::new();
    interpreter::run(&mut interpreter);
    if args.len() > 2 {
        println!("Usage: <interpreter> <source>");
    } else if args.len() == 2 {
        interpreter.run_src(&args[1]);
    } else {
        interpreter.run_prompt();
    }
}