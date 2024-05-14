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
    let number = 17;
    
    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}
