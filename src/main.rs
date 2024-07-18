use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        let mut user_input = String::new();
    
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line.");
    
        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        let guess: Guess = Guess::new(user_input);

        println!("You guessed: {}", guess.value());
    
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
