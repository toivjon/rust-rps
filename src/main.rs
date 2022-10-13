use std::io::{stdin};

fn main() {
    println!("Rock Paper Scissors");
    loop {
        println!("MENU");
        println!("  1   play game");
        println!("  2   quit game");
        println!("Please enter selection:");

        let mut selection = String::new();
        stdin().read_line(&mut selection).expect("Invalid input.");

        match selection.trim() {
            "1" => println!("TODO start game"),
            "2" => break,
            x => println!("Invalid input: {x}"),
        }
    }
}
