use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The number is: {}", secret_number);

    loop {

        println!("please input the number");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
