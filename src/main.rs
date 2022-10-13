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
            "1" => {
                loop {
                    println!("GAME");
                    println!("  r   rock");
                    println!("  p   paper");
                    println!("  s   scissors");
                    println!("Please enter selection:");

                    let mut selection = String::new();
                    stdin().read_line(&mut selection).expect("Invalid input.");

                    match selection.trim() {
                        "r" => println!("You selected rock!"),
                        "p" => println!("You selected paper!"),
                        "s" => println!("You selected scissors!"),
                        _ => break,
                    }
                }
            },
            "2" => break,
            x => println!("Invalid input: {x}"),
        }
    }
}
