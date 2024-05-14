fn shortest_word(sentence: &str) -> Option<&str> {
    sentence
        .split_whitespace() 
        .min_by_key(|word| word.len()) 
}

fn main() {
    let input_string = "This is a sample sentence with some words";
    
    if let Some(shortest) = shortest_word(input_string) {
        println!("The shortest word in the string is: {}", shortest);
    } else {
        println!("No words found in the input string.");
    }
}
