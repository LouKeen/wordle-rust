use std::io::stdin;
use wordle_rust::validate_answer;

fn main() {
    let word: &str = "abcde";

    let mut guesses: [String; 6] = std::array::from_fn(|_| String::new());

    let mut turn: usize = 0;

    loop {
        println!("Guess the secret word:");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let trimmed_len = guess.trim_end().len();
        guess.truncate(trimmed_len);

        if validate_answer(guess.as_str()) {
            continue;
        }

        if word == guess {
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
            guesses[turn] = guess;
            println!("{:?}", guesses);
        }
        turn += 1;
        if turn == 6 {
            println!("You lose!");
            break;
        }
    }
}
