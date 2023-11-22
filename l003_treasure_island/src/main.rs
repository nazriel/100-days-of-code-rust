use std::collections::HashMap;
use std::io;

fn read_reply() -> String {
    let mut buff = String::new();
    if io::stdin().read_line(&mut buff).is_err() {
        return "".to_owned();
    }

    println!();
    buff.trim().to_lowercase()
}

fn print_intro() {
    println!(
        r"
                            _.--.
                        _.-'_:-'||
                    _.-'_.-::::'||
               _.-:'_.-::::::'  ||
             .'`-.-:::::::'     ||
            /.'`;|:::::::'      ||_
           ||   ||::::::'     _.;._'-._
           ||   ||:::::'  _.-!oo @.!-._'-.
           \'.  ||:::::.-!()oo @!()@.-'_.|
            '.'-;|:.-'.&$@.& ()$%-'o.'\U||
              `>'-.!@%()@'@_%-'_.-o _.|'||
               ||-._'-.@.-'_.-' _.-o  |'||
               ||=[ '-._.-\U/.-'    o |'||
               || '-.]=|| |'|      o  |'||
               ||      || |'|        _| ';
               ||      || |'|    _.-'_.-'
               |'-._   || |'|_.-'_.-'
            jgs '-._'-.|| |' `_.-'
                    '-.||_/.-'

    "
    );

    println!("Welcome to Tresure Island");
    println!("Your mission is to find the treasure.");
    println!();
}

fn step_start() -> &'static str {
    println!("You're at a crossroad, where do you want to go? Type \"left\" or \"right\".");

    match read_reply().as_str() {
        "right" => {
            println!("You fell into a hole. Game Over.");
            "game_over"
        }
        "left" => "lake",
        _ => "start",
    }
}

fn step_lake() -> &'static str {
    println!("You've come to a lake. There is an island in the middle of the lake.");
    println!("Type \"wait\" to wait for a boat. Type \"swim\" to swim across.");

    match read_reply().as_str() {
        "swim" => {
            println!("You got attacked by an angry trout. Game Over.");
            "game_over"
        }
        "wait" => "houses",
        _ => "lake",
    }
}

fn step_houses() -> &'static str {
    println!("You arrive at the island unharmed. There is a house with 3 doors. One red, one yellow and one blue.");
    println!("Which colour do you choose?");

    match read_reply().as_str() {
        "red" => {
            println!("It's a room full of fire. Game Over.");
            "game_over"
        }
        "blue" => {
            println!("You enter a room of beasts. Game Over.");
            "game_over"
        }
        "yellow" => {
            println!("You found the treasure! You Win!");
            "game_over"
        }
        _ => "houses",
    }
}

fn main() {
    print_intro();

    // TODO: would be nice to make HashMap::from() to work
    let mut steps: HashMap<&str, fn() -> &'static str> = HashMap::new();
    steps.insert("start", step_start);
    steps.insert("lake", step_lake);
    steps.insert("houses", step_houses);

    let mut current = "start";
    while current != "game_over" {
        current = (steps
            .get(current)
            .unwrap_or_else(|| panic!("Unhandled step {current}")))();
    }
}
