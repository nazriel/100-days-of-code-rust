use rand::Rng;
use std::fmt;
use std::io;

enum Score {
    Win,
    Lose,
    Draw,
}

const ROCK_ASCII: &str = r"
Rock

    _______
---'   ____)
      (_____)
      (_____)
      (____)
---.__(___)
";

const PAPER_ASCII: &str = r"
Paper

    _______
---'   ____)____
          ______)
          _______)
         _______)
---.__________)
";

const SCISSORS_ASCII: &str = r"
Scissors

    _______
---'   ____)____
          ______)
       __________)
      (____)
---.__(___)
";

#[derive(PartialEq, Clone, Copy)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for Pick {
    fn from(value: u8) -> Self {
        match value {
            1 => Pick::Rock,
            2 => Pick::Paper,
            3 => Pick::Scissors,
            _ => panic!("unhandled"),
        }
    }
}

impl fmt::Display for Pick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Pick::Rock => ROCK_ASCII,
            Pick::Paper => PAPER_ASCII,
            Pick::Scissors => SCISSORS_ASCII,
        };
        write!(f, "{str}")
    }
}

fn main() {
    println!("What do you choose?");
    println!("1 - Rock");
    println!("2 - Paper");
    println!("3 - Scissors");

    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();
    let user_choice = Pick::from(buff.trim().parse::<u8>().unwrap_or(0));

    let mut rng = rand::thread_rng();
    let opponent_choice = Pick::from(rng.gen_range(1u8..=3));

    let result = match (user_choice, opponent_choice) {
        (Pick::Rock, Pick::Paper) => Score::Lose,
        (Pick::Rock, Pick::Scissors) => Score::Win,
        (Pick::Paper, Pick::Rock) => Score::Win,
        (Pick::Paper, Pick::Scissors) => Score::Lose,
        (Pick::Scissors, Pick::Rock) => Score::Lose,
        (Pick::Scissors, Pick::Paper) => Score::Win,
        (_, _) if user_choice == opponent_choice => Score::Draw,
        (_, _) => panic!("weird"),
    };

    println!("You picked: {user_choice}");
    println!("Your opponent picked: {opponent_choice}");
    match result {
        Score::Draw => println!("Draw."),
        Score::Lose => println!("You lose."),
        Score::Win => println!("You WIN!"),
    }
}
