use std::io;
mod apps;

fn main() {
    loop {
        println!();
        println!("Select what you want to perform:");
        println!("1. Fibonacci Number,");
        println!("2. Convert Celcius to Fahrenheit,");
        println!("3. Exit.");

        let mut val = String::new();

        io::stdin()
            .read_line(&mut val)
            .expect("Failed to read line!");

        let val_new: u8 = val.trim().parse().expect("Expecting a number!");

        match val_new {
            1 => apps::fibonacci::run(),
            2 => apps::ctof::run(),
            3 => {
                println!("Thanks! Visit again!");
                break;
            }
            _ => println!("Not an expected value!"),
        }
    }
}
