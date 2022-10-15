use rand::Rng;
use std::io::stdin;

enum MenuSelection {
    Play,
    Quit,
    Invalid,
}

enum IngameSelection {
    Rock,
    Paper,
    Scissors,
    Invalid,
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

                let opponent_selection = match rng.gen_range(0..3) {
                    0 => IngameSelection::Rock,
                    1 => IngameSelection::Paper,
                    2 => IngameSelection::Scissors,
                    _ => IngameSelection::Invalid,
                };

                match wait_ingame_selection() {
                    IngameSelection::Invalid => continue,
                    IngameSelection::Rock => match opponent_selection {
                        IngameSelection::Invalid => panic!("Invalid opponent selection!"),
                        IngameSelection::Rock => {
                            println!("Rock vs. Rock");
                            println!("We have a DRAW. Let's go for an another round!");
                            continue;
                        }
                        IngameSelection::Paper => {
                            println!("Rock vs. Paper");
                            println!("Paper beats rock. You LOSE. Better luck next time!");
                            break;
                        }
                        IngameSelection::Scissors => {
                            println!("Rock vs. Scissors");
                            println!("Rock beats scissors. You WIN. Congratulations!");
                            break;
                        }
                    },
                    IngameSelection::Paper => match opponent_selection {
                        IngameSelection::Invalid => panic!("Invalid opponent selection!"),
                        IngameSelection::Rock => {
                            println!("Paper vs. Rock");
                            println!("Paper beats rock. You WIN. Congratulations!");
                            break;
                        }
                        IngameSelection::Paper => {
                            println!("Paper vs. Paper");
                            println!("We have a DRAW. Let's go for an another round!");
                            continue;
                        }
                        IngameSelection::Scissors => {
                            println!("Paper vs. Scissors");
                            println!("Scissors beats paper. You LOSE. Better luck next time!");
                            break;
                        }
                    },
                    IngameSelection::Scissors => match opponent_selection {
                        IngameSelection::Invalid => panic!("Invalid opponent selection!"),
                        IngameSelection::Rock => {
                            println!("Scissors vs. Rock");
                            println!("Rock beats scissors. You LOSE. Congratulations!");
                            break;
                        }
                        IngameSelection::Paper => {
                            println!("Scissors vs. Paper");
                            println!("Scissors beats paper. You WIN. Congratulations!");
                            break;
                        }
                        IngameSelection::Scissors => {
                            println!("Scissors vs. Scissors");
                            println!("We have a DRAW. Let's go for an another round!");
                            continue;
                        }
                    },
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
