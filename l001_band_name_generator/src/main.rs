use std::error;
use std::io;

use band_name_generator::generate_band;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Welcome to the Band Name Generator.");

    println!("What's name of the city you grew up in?");
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;

    println!("What's your pet's name?");
    let mut pet = String::new();
    io::stdin().read_line(&mut pet)?;

    match generate_band(city.trim_end(), pet.trim_end()) {
        Ok(result) => {
            println!("Your band name could be {result}");
            Ok(())
        }
        Err(fail) => panic!("Error occured: {fail:?}"),
    }
}
