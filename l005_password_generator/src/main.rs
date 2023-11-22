use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

fn read_param(msg: &str) -> Option<u8> {
    eprintln!("{msg}");
    let mut buff = String::new();
    if io::stdin().read_line(&mut buff).is_err() {
        return None;
    }

    buff.trim().parse::<u8>().ok()
}

fn generate_password(letters_count: u8, numbers_count: u8, symbols_count: u8) -> String {
    let mut pool = Vec::<char>::new();
    let mut rng = rand::thread_rng();

    let chars: Vec<char> = ('A'..='z').collect();
    let numbers: Vec<char> = ('0'..='9').collect();
    let symbols: Vec<char> = ('!'..='/').collect();

    for _ in 0..letters_count {
        pool.push(chars[rng.gen_range(0..chars.len())]);
    }

    for _ in 0..numbers_count {
        pool.push(numbers[rng.gen_range(0..numbers.len())]);
    }

    for _ in 0..symbols_count {
        pool.push(symbols[rng.gen_range(0..symbols.len())]);
    }

    pool.shuffle(&mut rng);
    pool.into_iter().collect()
}

fn main() {
    let letters_count = read_param("How many letters would you like in your password?").unwrap();
    let numbers_count = read_param("How many numbers would you like in your password?").unwrap();
    let symbols_count = read_param("How many symbols would you like in your password?").unwrap();

    eprintln!("Your password:");
    println!(
        "{}",
        generate_password(letters_count, numbers_count, symbols_count)
    )
}
