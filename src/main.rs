use rand::Rng;
use std::io::stdin;

enum MenuSelection {
    Play,
    Quit,
    Invalid,
}

#[derive(Debug)]
enum IngameSelection {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

enum Outcome {
    Win,
    Draw,
    Lose,
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
            MenuSelection::Invalid => continue,
            MenuSelection::Play => loop {
                println!("===========================");
                println!("ROCK PAPER SCISSORS - ROUND");
                println!("r : rock");
                println!("p : paper");
                println!("s : scissors");
                println!("Please enter selection:");

                let player = wait_ingame_selection();

                let opponent = match rng.gen_range(0..3) {
                    0 => IngameSelection::Rock,
                    1 => IngameSelection::Paper,
                    _ => IngameSelection::Scissors,
                };

                println!("Player   -> {:?}", player);
                println!("Opponent -> {:?}", opponent);
                match check_result(player, opponent) {
                    Outcome::Draw => {
                        println!("It's a DRAW! Let's have an another round!");
                    }
                    Outcome::Win => {
                        println!("You WIN! Congratulations!");
                        break;
                    }
                    Outcome::Lose => {
                        println!("You LOSE! Better luck next time!");
                        break;
                    }
                }
            },
            MenuSelection::Quit => break,
        }
    }
}

fn wait_menu_selection() -> MenuSelection {
    let mut selection = String::new();
    stdin().read_line(&mut selection).expect("Invalid input.");
    match selection.trim() {
        "1" => MenuSelection::Play,
        "2" => MenuSelection::Quit,
        _ => MenuSelection::Invalid,
    }
}

fn wait_ingame_selection() -> IngameSelection {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid input.");
    match input.trim() {
        "r" => IngameSelection::Rock,
        "p" => IngameSelection::Paper,
        "s" => IngameSelection::Scissors,
        _ => IngameSelection::Invalid,
    }
}

fn check_result(player_selection: IngameSelection, opponent_selection: IngameSelection) -> Outcome {
    match player_selection {
        IngameSelection::Rock => match opponent_selection {
            IngameSelection::Rock => Outcome::Draw,
            IngameSelection::Paper => Outcome::Lose,
            IngameSelection::Scissors => Outcome::Win,
            IngameSelection::Invalid => panic!("Invalid opponent selection!"),
        },
        IngameSelection::Paper => match opponent_selection {
            IngameSelection::Rock => Outcome::Win,
            IngameSelection::Paper => Outcome::Draw,
            IngameSelection::Scissors => Outcome::Lose,
            IngameSelection::Invalid => panic!("Invalid opponent selection!"),
        },
        IngameSelection::Scissors => match opponent_selection {
            IngameSelection::Rock => Outcome::Lose,
            IngameSelection::Paper => Outcome::Win,
            IngameSelection::Scissors => Outcome::Draw,
            IngameSelection::Invalid => panic!("Invalid opponent selection!"),
        },
        IngameSelection::Invalid => panic!("Invalid player selection!"),
    }
}
