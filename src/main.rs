use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let guess: u32 = match get_guess() {
            Ok(i) => i,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // lambda functions for each "arm" of the match statment
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn get_guess() -> Result<u32, std::num::ParseIntError> {
    let mut s = String::new();
    println!("Input your guess.");
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line"); // message on fail

    // parse type casting is inferred by explicit type defs
    // this is a shorthand match statement in which the error is implied.
    let i: u32 = s.trim().parse()?;
    // Rust doesn't use "return" all that often, rather just an expression with no semicolon
    Ok(i)
}
