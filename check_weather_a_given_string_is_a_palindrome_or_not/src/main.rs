fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); 
    let reversed = input.chars().rev().collect::<String>(); 

    input == reversed 
}

fn main() {
    let test_string1 = "racecar";
    let test_string2 = "govind";
    
    println!("Is '{}' a palindrome? {}", test_string1, is_palindrome(test_string1));
    println!("Is '{}' a palindrome? {}", test_string2, is_palindrome(test_string2));
}
