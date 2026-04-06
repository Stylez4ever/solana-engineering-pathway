use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count = *count + 1;
    }

    counts
}

fn count_characters(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        for character in word.chars() {
        let count = counts.entry(character).or_insert(0);
        *count = *count + 1;
        }
    }

    counts
}





fn main() {
    println!("{:?}", count_words("Sally sells sea sheels by the sea shore"));

    println!("{:?}", count_characters("Sally sells sea sheels by the sea shore"));
}
