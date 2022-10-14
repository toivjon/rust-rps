use rand::{self, Rng};
use std::io::stdin;

enum MenuSelection {
    Play,
    Quit,
    None,
}

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        println!("==========================");
        println!("ROCK PAPER SCISSORS - MENU");
        println!("1 : play game");
        println!("2 : quit game");
        println!("Please enter selection:");

        match wait_menu_selection() {
            MenuSelection::Play => {
                let mut outcome = Outcome::Draw;
                while outcome == Outcome::Draw {
                    println!("===========================");
                    println!("ROCK PAPER SCISSORS - ROUND");
                    println!("r : rock");
                    println!("p : paper");
                    println!("s : scissors");
                    println!("Please enter selection:");

                    let player = wait_ingame_selection();

                    let opponent = match rng.gen_range(0..3) {
                        0 => Item::Rock,
                        1 => Item::Paper,
                        _ => Item::Scissors,
                    };

                    println!("Player   -> {:?}", player);
                    println!("Opponent -> {:?}", opponent);
                    outcome = check_result(player, opponent);
                    match outcome {
                        Outcome::Draw => println!("It's a DRAW! Let's have an another round!"),
                        Outcome::Win => println!("You WIN! Congratulations!"),
                        Outcome::Lose => println!("You LOSE! Better luck next time!"),
                    }
                }
            }
            MenuSelection::Quit => break,
            MenuSelection::None => continue,
        }
    }
}

#[derive(Debug)]
enum Item {
    Rock,
    Paper,
    Scissors,
    None,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn wait_menu_selection() -> MenuSelection {
    let mut selection = String::new();
    stdin().read_line(&mut selection).expect("Invalid input.");
    match selection.trim() {
        "1" => MenuSelection::Play,
        "2" => MenuSelection::Quit,
        _ => MenuSelection::None,
    }
}

fn wait_ingame_selection() -> Item {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid input.");
    match input.trim() {
        "r" => Item::Rock,
        "p" => Item::Paper,
        "s" => Item::Scissors,
        _ => Item::None,
    }
}

fn check_result(player_selection: Item, opponent_selection: Item) -> Outcome {
    match player_selection {
        Item::Rock => match opponent_selection {
            Item::Rock => Outcome::Draw,
            Item::Paper => Outcome::Lose,
            Item::Scissors => Outcome::Win,
            Item::None => panic!("Invalid opponent selection!"),
        },
        Item::Paper => match opponent_selection {
            Item::Rock => Outcome::Win,
            Item::Paper => Outcome::Draw,
            Item::Scissors => Outcome::Lose,
            Item::None => panic!("Invalid opponent selection!"),
        },
        Item::Scissors => match opponent_selection {
            Item::Rock => Outcome::Lose,
            Item::Paper => Outcome::Win,
            Item::Scissors => Outcome::Draw,
            Item::None => panic!("Invalid opponent selection!"),
        },
        Item::None => panic!("Invalid player selection!"),
    }
}
