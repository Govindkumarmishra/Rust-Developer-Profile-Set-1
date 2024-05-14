fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new(); 
    }

    for (i, char) in strings[0].chars().enumerate() {
        if strings.iter().any(|s| s.chars().nth(i) != Some(char)) {
            return strings[0][..i].to_string(); 
        }
    }

    
    strings[0].to_string()
}

fn main() {
    let strings1 = ["flower", "flow", "flight"];
    let strings2 = ["dog", "racecar", "car"];

    println!("Longest common prefix of {:?}: {}", strings1, longest_common_prefix(&strings1));
    println!("Longest common prefix of {:?}: {}", strings2, longest_common_prefix(&strings2));
}
