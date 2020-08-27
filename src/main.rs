use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        // creates a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Switching from an expect call to a match expression is how you generally move 
        // from crashing on an error to handling the error. 
        // If parse is able to successfully turn the string into a number, it will return an Ok value 
        // that contains the resulting number. If parse is not able to turn the string into a number, 
        // it will return an Err value that contains more information about the error. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  // _, is a catchall value; here we want to match all Err values, no matter what information they have inside them. 
        };
    
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    

    
}
