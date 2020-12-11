use calculator::*;
use std::io::{prelude::*, stdout};

fn main() {
    loop {
        println!("Welcome to the Calculator!");
        println!("Select an option");
        println!("--- 1 --- Addition");
        println!("--- 2 --- Subtraction");
        println!("--- 3 --- Multiplication");
        println!("--- 4 --- Division");
        println!("--- 0 --- Exit");
        print!(">>");
        stdout().flush().ok().expect("Could not flush stdout");
        let op = option_input(0, 4);

        if op == 0 {
            break;
        };

        println!("Enter the first number");
        print!(">>");
        stdout().flush().ok().expect("Could not flush stdout");
        let n1 = number_input();

        println!("Enter the second number");
        print!(">>");
        stdout().flush().ok().expect("Could not flush stdout");
        let n2 = number_input();

        let res = match op {
            1 => add(n1, n2),
            2 => subtract(n1, n2),
            3 => multiply(n1, n2),
            4 => divide(n1, n2),
            _ => panic!("Unexpected option received."),
        };

        println!("Result: {}", res);
    }
}
