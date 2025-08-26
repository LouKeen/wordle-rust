use std::io::stdin;
use wordle_rust::validate_answer;

fn main() {
    let word: &str = "abcde";

    let mut turn: u8 = 0;

    loop {
        println!("Guess the secret word:");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim();

        if validate_answer(guess) {
            continue;
        }

        if word.eq(guess) {
            println!("You win!");
            break;
        } else {
            for (i, c) in guess.chars().enumerate() {
                if c == word.chars().nth(i).unwrap() {
                    let x = "\x1b[92mx\x1b[0m";
                    print!("{}", x);
                } else if word.contains(c) {
                    print!("\x1b[93m{}\x1b[0m", c)
                } else {
                    print!("\x1b[91m{}\x1b[0m", c)
                }
            }
            println!();
        }
        turn += 1;
        if turn == 6 {
            println!("You lose!");
            break;
        }
    }
}
