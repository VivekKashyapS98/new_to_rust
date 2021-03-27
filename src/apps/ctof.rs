use std::io;

fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn run() {
    println!("Enter a number to convert to Fahrenheit");

    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Failed reading input!");

    let val_new: f64 = val.trim().parse().expect("Expecting a number!");

    println!(
        "Your result is {} Fahrenheit",
        celcius_to_fahrenheit(val_new)
    );
}
