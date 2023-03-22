use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Shri Ganeshay");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your secret number is  {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You typed lesser number"),
            Ordering::Greater => println!("You typed greater number"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
