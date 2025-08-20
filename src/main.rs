use std::io::stdin;

fn main() {
    let word: &str = "abcde";

    loop {
        println!("Guess the secret word:");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != 5 {
            println!("word must be exactly 5 letters");
            continue;
        }

        if word.eq(guess) {
            println!("You win!");
            break;
        } else {
            for n in 0..5 {
                let current_char = guess.chars().nth(n).unwrap();

                if current_char == word.chars().nth(n).unwrap() {
                    print!("\x1b[92m{}\x1b[0m", current_char)
                } else if word.contains(current_char) {
                    print!("\x1b[93m{}\x1b[0m", current_char)
                } else {
                    print!("\x1b[91m{}\x1b[0m", current_char)
                }
            }
            println!();
        }
    }
}
