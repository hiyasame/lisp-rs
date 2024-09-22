use env::Env;
use eval::eval_str;
use exp::LispExp;
use linefeed::{Interface, ReadResult};

mod lexer;
mod parser;
mod exp;
mod env;
mod eval;

mod error;

const PROMPT: &str = "lisp-rs> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = Env::new();

    reader.set_prompt(PROMPT).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let val = eval_str(&input, &mut env);
        if let Ok(val) = val {
            match val {
                LispExp::Number(n) => println!("{}", n),
                LispExp::Bool(b) => println!("{}", b),
                LispExp::Symbol(s) => println!("{}", s),
                _ => println!("{:?}", val),
            }
        } else {
            println!("error: {:?}", val);
        }
    }

    println!("Good bye");
    Ok(())
}
