use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
// cargo doc --open 
// opens the documentation for your crates !!!
fn main() {

    println!("Guess the number! ");

    // Sometimes just using another function is getting the job done
    let secret_number = rand::thread_rng().gen_range(0..=50);
   
    loop {
        let mut guess = String::new();

        println!("Enter a guess");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        // &mut guess is immutable
        // &guess would be mutable 
        
        println!("You guessed {guess}");

        // This line is precious, shadowing, converting types
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {println!("You win"); break;},
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small")
        }
    }
    
}
