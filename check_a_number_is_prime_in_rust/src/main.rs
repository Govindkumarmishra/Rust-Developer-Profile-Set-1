use std::io;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; 
    }

    
    for i in 2..=f64::sqrt(n as f64) as u64 {
        if n % i == 0 {
            return false; 
        }
    }

    true 
}

fn main() {
    println!("Enter a number to check if it's prime or not:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u64 = input.trim().parse().expect("Please enter a valid number");

    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}
