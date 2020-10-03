use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //to introduce the game on the console
    println!("Guess the number!");

    // imutable variable definition to hold random number from 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // outputs the generated random number to screen
    println!("The secret number is: {}", secret_number);

    // begining of looping (infinite)
    loop{

        // instruct for an input number
        println!("Please input your guess.");

        // defining and assigning a mutable variable to an empty string
        let mut guess = String::new();

        // reading user input
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // changing the type of guess from string to integer for comparison
        // continues(skips the current loop) once the input type cannot be changed instead of trowing an error
        // using the match keyword (switching from an except to match expression
        // match is used with enum results mostly
        // : -> represents annotation(changing) of variable type, u32 -> represents unsigned 32 bit integer
        // _ -> represents all forms of errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // prints the guessed value
        println!("You guessed: {}", guess);

        // matches the coparison result and prints the approipraite output
        // then quits if the guess is equal to the random number
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
