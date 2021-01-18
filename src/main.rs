use rpn_calc_rs::RPNCalc;
use std::env;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let mut calc = RPNCalc::new(args);
    match calc.run() {
        Ok(answer) => println!("{}", answer),
        Err(err) => eprint!("{}", err),
    }
}
