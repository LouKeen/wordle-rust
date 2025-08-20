use std::io::stdin;

fn main() {
    let word: &str = "abcde";
    loop {
        println!("Guess the secret word:");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        if word.eq(guess.trim()) {
            println!("You win!");
            break;
        } else {
            println!("You lose!");
        }
    }
}
