use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

// Basic guess game
fn level_one() {
    println!("\n");
    println!("===========================================================================");
    println!("\t\t\tBASIC GUESSING GAME");
    println!("===========================================================================");
    let mut randy = rand::thread_rng();
    let mut selection = randy.gen_range(0x0..0x100);
    let mut player_score = 0;

    // main game loop
    loop {
        // println!("DEBUG: answer is {}", selection);
        println!("Current score: {}\n", player_score);
        println!("What is 0x{:x} in decimal? ", selection);

        // inner loop, grab guesses until the correct one is submitted
        loop {
            // reset the guess on each loop iteration
            let mut guess = String::new();

            // read from stdin
            print!("Answer: ");
            io::stdout().flush().expect("couldn't flush stdout");
            io::stdin()
                .read_line(&mut guess)
                .expect("failed to read from stdin");

            // convert input to a number
            let guess: u32 = guess.trim().parse().expect("That wasn't even a number...?");

            // check the submitted number
            match guess.cmp(&selection) {
                Ordering::Less => println!("Try a little higher"),
                Ordering::Greater => println!("Try a little lower"),
                Ordering::Equal => {
                    println!("That's right!");
                    // increment score and grab a new random number for the next round
                    player_score += 10;
                    selection = randy.gen_range(0x0..0x100);
                    break;
                }
            }
        }
    }
}


// Sums Game
fn level_two() {
    println!("\n");
    println!("===========================================================================");
    println!("\t\t\tSUMS GAME");
    println!("===========================================================================");
    let mut randy = rand::thread_rng();
    let mut num1 = randy.gen_range(0x0..0xa0);
    let mut num2 = randy.gen_range(0x0..0xa0);
    let mut answer = num1 + num2;
    let mut player_score = 0;

    // main game loop
    loop {
        // println!("DEBUG: answer is {}", answer);
        println!("Current score: {}\n", player_score);
        println!("What is 0x{:x} + 0x{:x} in decimal? ", num1, num2);

        // inner loop, grab guesses until the correct one is submitted
        loop {
            // reset the guess on each loop iteration
            let mut guess = String::new();

            // read from stdin
            print!("Answer: ");
            io::stdout().flush().expect("couldn't flush stdout");
            io::stdin()
                .read_line(&mut guess)
                .expect("failed to read from stdin");

            // convert input to a number
            let guess: u32 = guess.trim().parse().expect("That wasn't even a number...?");

            // check the submitted answer
            match guess.cmp(&answer) {
                Ordering::Less => println!("Try a little higher"),
                Ordering::Greater => println!("Try a little lower"),
                Ordering::Equal => {
                    println!("That's right!");
                    // increment score and grab a new random number for the next round
                    player_score += 10;
                    num1 = randy.gen_range(0x0..0xa0);
                    num2 = randy.gen_range(0x0..0xa0);
                    answer = num1 + num2;
                    break;
                }
            }
        }
    }
}

// Main game
fn main() {
    println!("Select the game mode you'd like to play:");
    println!("[1] Basic - provide the decimal version of a hex number");
    println!("[2] Sums - provide the sum of two hex numbers\n");

    loop {
        let mut mode_selection = String::new();
        print!(">> ");
        io::stdout().flush().expect("couldn't flush stdout");
        io::stdin()
            .read_line(&mut mode_selection)
            .expect("failed to read from stdin");

        let mode_selection: u32 = mode_selection.trim().parse().expect("that wasn't a number");
        match mode_selection {
            1 => level_one(),
            2 => level_two(),
            _ => {
                println!("error: invalid selection");
            }
        }
    }
}
