use std::io::{stdin, stdout, Write};
use std::process::exit;

fn read(input: &mut String) {
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read");
}

fn main() {
    println!("Welcome to Axmouth's calculator!");
    println!("============");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("What is the operator? [+-*/^x] : ");
        read(&mut operator);
        let operator: char = operator.trim().chars().next().unwrap();
        if operator == 'x' {
            exit(0);
        }

        let operators = String::from("+-*/^%x");

        if !operators.contains(operator) {
            println!("Unknown operator");
            continue;
        }

        print!("What is the first number?: ");
        read(&mut num1);

        print!("What is the second number?: ");
        read(&mut num2);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            '^' => num1.powf(num2),
            '%' => num1 % num2,
            'x' => exit(0),
            _ => panic!("Error in operator"),
        };

        println!("The result of {} {} {} = {}", num1, operator, num2, result);
    }
}
