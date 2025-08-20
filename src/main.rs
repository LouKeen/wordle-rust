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
            println!("You lose!");
        }
    }
}
