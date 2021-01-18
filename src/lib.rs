#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Token {
    Operator(Command),
    Number(i32),
}

pub struct RPNCalc {
    tokens: Vec<Token>,
    stack: Vec<i32>,
}

impl RPNCalc {
    pub fn new(input: Vec<String>) -> RPNCalc {
        RPNCalc {
            tokens: RPNCalc::tokenize(input),
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<i32, String> {
        for token in self.tokens.clone() {
            use Token::*;
            match token {
                Operator(op) => {
                    self.execute(op)?;
                }
                Number(n) => self.stack.push(n),
            }
        }

        self.stack
            .pop()
            .ok_or(String::from("Unable to pop final result from stack"))
    }

    fn execute(&mut self, command: Command) -> Result<(), String> {
        let y = self
            .stack
            .pop()
            .ok_or(String::from("Unable to pop value from stack"))?;
        let x = self
            .stack
            .pop()
            .ok_or(String::from("Unable to pop value from stack"))?;
        use Command::*;
        let r = match command {
            Add => x + y,
            Sub => x - y,
            Mul => x * y,
            Div => x / y,
        };

        self.stack.push(r);

        Ok(())
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
}

#[cfg(test)]
mod test {
    use crate::{Command, RPNCalc, Token};

    #[test]
    fn test_tokenize() {
        let calc = RPNCalc::new(vec![
            String::from("3"),
            String::from("+"),
            String::from("2"),
            String::from("-"),
            String::from("/"),
            String::from("*"),
        ]);

        assert_eq!(
            calc.tokens,
            vec![
                Token::Number(3),
                Token::Operator(Command::Add),
                Token::Number(2),
                Token::Operator(Command::Sub),
                Token::Operator(Command::Div),
                Token::Operator(Command::Mul)
            ]
        );
    }
}
