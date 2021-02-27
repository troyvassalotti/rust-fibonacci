use std::io;

fn main() {
    println!("Let's find some Fibonacci numbers.");

    println!("Which Fibonacci number do you want to know?");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("Easy! That's {}", fibonacci(number));
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
