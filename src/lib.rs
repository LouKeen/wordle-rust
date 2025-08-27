pub fn validate_answer(answer: &str) -> bool {
    if answer.len() != 5 {
        println!("word must be exactly 5 letters");
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::validate_answer;

    #[test]
    fn should_result_in_false_when_answer_is_exactly_5_letters() {
        assert_eq!(validate_answer("abcde"), false);
    }

    #[test]
    fn should_result_in_true_when_answer_is_not_5_letters() {
        assert_eq!(validate_answer("a"), true);
        assert_eq!(validate_answer("ab"), true);
        assert_eq!(validate_answer("abc"), true);
        assert_eq!(validate_answer("abcd"), true);
        assert_eq!(validate_answer("abcdef"), true);
    }
}
