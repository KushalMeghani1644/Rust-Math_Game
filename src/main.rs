use rand::Rng;
use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_number(prompt: &str) -> i32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, please try again!"),
        }
    }
}

fn main() {
    println!("----MATH-GAME----");
    let repeat = read_number("Enter how many times the the game should be played: ");
    let mut rng = rand::rng();
    for _ in 0..repeat {
        let num1 = rng.random_range(1..=100);
        let num2 = rng.random_range(1..=100);
        let op = read_input("Enter the operation you want to perform (+, -, *, /): ");
        let result = match op.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0 {
                    println!("Cannot divide by zero!");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Invalid operation! '{}'. Skipping this round!", op);
                continue;
            }
        };
        let question = format!("what is {} {} {}: ", num1, op, num2);
        let user_answer = read_number(&question);
        if user_answer == result {
            println!("CORRECT!");
        } else {
            println!("Incorrect, the answer is: {}", result);
        }
        println!("----------------------------------");
    }
    println!("Thanks for playing!");
}
