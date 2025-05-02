use rand::Rng;
use std::io;
fn main() {
    //Rust code for maths  game
    let mut input = String::new();
    println!("Enter how many times the code should run: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input_trimmed: i32 = input.trim().parse().expect("Failed to read line");
    let mut rng = rand::rng();
    for _ in 0..input_trimmed {
        let num1 = rng.random_range(1..=100);
        let num2 = rng.random_range(1..=100);
        let mut op = String::new();
        println!("Enter the operation you want to perform (+,-,*,/): ");
        io::stdin()
            .read_line(&mut op)
            .expect("failed to read line please enter (+,-,*,/)");
        let op_trimmed = op.trim();
        match op_trimmed {
            "+" => {
                let result = num1 + num2;
                let mut usr_answer = String::new();
                println!("What is {} + {} = ?", num1, num2);
                io::stdin()
                    .read_line(&mut usr_answer)
                    .expect("Failed to read line");
                let usr_answer_trimmed: i32 =
                    usr_answer.trim().parse().expect("Failed to read line");
                if usr_answer_trimmed == result {
                    println!("Correct!");
                } else {
                    println!("Incorrect! The correct answer is {}", result);
                }
            }
            "-" => {
                let result = num1 - num2;
                let mut usr_answer = String::new();
                println!("What is {} - {} = ?", num1, num2);
                io::stdin()
                    .read_line(&mut usr_answer)
                    .expect("Failed to read line");
                let usr_answer_trimmed: i32 =
                    usr_answer.trim().parse().expect("Failed to read line");
                if usr_answer_trimmed == result {
                    println!("Correct!");
                } else {
                    println!("Incorrect! The correct answer is {}", result);
                }
            }
            "*" => {
                let result = num1 * num2;
                let mut usr_answer = String::new();
                println!("What is {} * {} = ?", num1, num2);
                io::stdin()
                    .read_line(&mut usr_answer)
                    .expect("Failed to read line");
                let usr_answer_trimmed: i32 =
                    usr_answer.trim().parse().expect("Failed to read line");
                if usr_answer_trimmed == result {
                    println!("Correct!");
                } else {
                    println!("Incorrect! The correct answer is {}", result);
                }
            }
            "/" => {
                if num2 == 0 {
                    println!("Cannot divide by zero");
                } else {
                    let result = num1 / num2;
                    let mut usr_answer = String::new();
                    println!("What is {} / {} = ?", num1, num2);
                    io::stdin()
                        .read_line(&mut usr_answer)
                        .expect("Failed to read line");
                    let usr_answer_trimmed: i32 =
                        usr_answer.trim().parse().expect("Failed to read line");
                    if usr_answer_trimmed == result {
                        println!("Correct!");
                    } else {
                        println!("Incorrect! The correct answer is {}", result);
                    }
                }
            }
            _ => {
                println!("Invalid operation");
            }
        }
    }
}
