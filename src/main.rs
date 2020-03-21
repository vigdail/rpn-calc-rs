use std::env;

#[derive(Debug)]
enum Command {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Token {
    Operator(Command),
    Number(i32),
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let tokens: Vec<Token> = tokenize(args);
    let mut stack: Vec<i32> = Vec::new();
    for item in tokens {
        use Token::*;
        match item {
            Operator(op) => {
                stack = execute(op, &stack);
            }
            Number(n) => stack.push(n),
        }
    }

    let r = stack.pop().expect("Unable to pop value from stack");
    println!("{}", r);
}

fn execute(command: Command, stack: &Vec<i32>) -> Vec<i32> {
    let mut stack = stack.clone();
    let y = stack.pop().expect("Unable to pop value from stack");
    let x = stack.pop().expect("Unable to pop value from stack");
    use Command::*;
    let r = match command {
        Add => x + y,
        Sub => x - y,
        Mul => x * y,
        Div => x / y,
    };

    stack.push(r);
    stack
}

fn tokenize(input: Vec<String>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    for item in input {
        match item.as_str() {
            "+" => stack.push(Token::Operator(Command::Add)),
            "-" => stack.push(Token::Operator(Command::Sub)),
            "*" => stack.push(Token::Operator(Command::Mul)),
            "/" => stack.push(Token::Operator(Command::Div)),
            _ => {
                if let Some(x) = item.parse::<i32>().ok() {
                    stack.push(Token::Number(x));
                }
            }
        }
    }

    stack
}
