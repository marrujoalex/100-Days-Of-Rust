fn find_nemo(test_str: &str, word_to_find: &str) -> Option<usize> {
    for (index, word) in test_str.split_whitespace().enumerate() {
        let word = word.trim_matches(|c: char| c.is_ascii_punctuation());

        if word == word_to_find {
            return Some(index + 1)
        }
    }

    None
}

fn main() {
    let test_string = "Hi, I am Nemo!";
    let word_to_find = "Nemo";

    match find_nemo(test_string, word_to_find) {
        Some(result) => println!("I found {word_to_find} at {}!", result),
        None => println!("I can't find Nemo :(")
    }
}
