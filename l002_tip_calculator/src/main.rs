use std::io;
use std::ops::{Add, Div};
use std::str::FromStr;
use tip_calculator::calculate_tip;

fn read_num<T>(default: T) -> T
where
    T: Add<Output = T> + Div<Output = T> + FromStr,
{
    let mut buff = String::new();
    if io::stdin().read_line(&mut buff).is_ok() {
        return buff.trim_end().parse::<T>().unwrap_or(default);
    }

    default
}

fn main() {
    println!("Welcome to the tip calculator.");

    println!("What was the total bill? ");
    let total = read_num(0.0f32);

    println!("What percentage tip would you like to give? ");
    let percentage = read_num(0u8);

    println!("How many people to split the bill? ");
    let people = read_num(0u32);

    if let Some(each) = calculate_tip(total, percentage, people) {
        println!("Each person should pay: {}", each);
    } else {
        eprintln!("One of provided values was invalid");
        std::process::exit(1)
    }
}
