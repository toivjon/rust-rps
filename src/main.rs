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

                let mut player_selection = String::new();
                stdin().read_line(&mut player_selection).expect("Invalid input.");

                let opponent_selection = match rng.gen_range(0..3) {
                    0 => "r",
                    1 => "p",
                    _ => "s",
                };

                if check_result(player_selection.trim(), opponent_selection) {
                    break;
                }
            },
            "2" => break,
            x => println!("Invalid input: {x}"),
        }
    }
}

fn check_result(player_selection: &str, opponent_selection: &str) -> bool {
    let mut should_break = false;
    match player_selection.trim() {
        "r" if opponent_selection == "p" => {
            println!("You selected rock and opponent selected paper. Opponent wins!");
            should_break = true;
        }
        "r" if opponent_selection == "s" => {
            println!("You selected rock and opponent selected scissors. You win!");
            should_break = true;
        }
        "r" if opponent_selection == "r" => {
            println!("You selected rock and opponent selected rock. It's a draw!")
        }
        "p" if opponent_selection == "s" => {
            println!("You selected paper and opponent selected scissors. Opponent wins!");
            should_break = true;
        }
        "p" if opponent_selection == "r" => {
            println!("You selected paper and opponent selected rock. You win!");
            should_break = true;
        }
        "p" if opponent_selection == "p" => {
            println!("You selected paper and opponent selected paper. It's a draw!")
        }
        "s" if opponent_selection == "r" => {
            println!("You selected scissors and opponent selected rock. Opponent wins!");
            should_break = true;
        }
        "s" if opponent_selection == "p" => {
            println!("You selected scissors and opponent selected paper. You win!");
            should_break = true;
        }
        "s" if opponent_selection == "p" => {
            println!("You selected scissors and opponent selected scissors. It's a draw!")
        }
        _ => {
            should_break = true;
        }
    }
    should_break
}
