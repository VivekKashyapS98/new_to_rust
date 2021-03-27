use std::io;

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn run() {
    println!("Enter a number to find out its Fibonacci number:");
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line!");

    let val_new: i32 = val.trim().parse().expect("Expecting a number!");

    println!("Your Fibonacci Number is {}", fibonacci(val_new));
}
