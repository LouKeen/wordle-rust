pub fn validate_answer(answer: &str) -> bool {
    if answer.len() != 5 {
        println!("word must be exactly 5 letters");
        return true
    }
    false
}