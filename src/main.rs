use rand::{self, Rng};
use std::io::stdin;

fn main() {
    println!("Rock Paper Scissors");
    let mut rng = rand::thread_rng();
    loop {
        println!("MENU");
        println!("  1   play game");
        println!("  2   quit game");
        println!("Please enter selection:");

        let mut selection = String::new();
        stdin().read_line(&mut selection).expect("Invalid input.");

        match selection.trim() {
            "1" => loop {
                println!("GAME");
                println!("  r   rock");
                println!("  p   paper");
                println!("  s   scissors");
                println!("Please enter selection:");

                let mut input = String::new();
                stdin().read_line(&mut input).expect("Invalid input.");

                let player = match input.trim() {
                    "r" => Item::Rock,
                    "p" => Item::Paper,
                    "s" => Item::Scissors,
                    _ => continue,
                };

                let opponent = match rng.gen_range(0..3) {
                    0 => Item::Rock,
                    1 => Item::Paper,
                    _ => Item::Scissors,
                };

                println!("Player: {:?} vs. Opponent: {:?}", player, opponent);
                match check_result(player, opponent) {
                    Outcome::Draw => println!("It is a draw! Let us have an another round!"),
                    Outcome::Win => {
                        println!("You win! Congratulations!");
                        break;
                    }
                    Outcome::Lose => {
                        println!("You lose! Better luck next time!");
                        break;
                    }
                };
            },
            "2" => break,
            x => println!("Invalid input: {x}"),
        }
    }
}

#[derive(Debug)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn check_result(player_selection: Item, opponent_selection: Item) -> Outcome {
    match player_selection {
        Item::Rock => match opponent_selection {
            Item::Rock => Outcome::Draw,
            Item::Paper => Outcome::Lose,
            Item::Scissors => Outcome::Win,
        },
        Item::Paper => match opponent_selection {
            Item::Rock => Outcome::Win,
            Item::Paper => Outcome::Draw,
            Item::Scissors => Outcome::Lose,
        },
        Item::Scissors => match opponent_selection {
            Item::Rock => Outcome::Lose,
            Item::Paper => Outcome::Win,
            Item::Scissors => Outcome::Draw,
        },
    }
}
