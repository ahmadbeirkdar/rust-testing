use std::io;
use rand::Rng;

fn main() {
    loop{
    println!("Guess the number!");
    println!("Enter your guess");
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1,100);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(n) => n,
        Err(_) => {println!("PLEASE ENTER A NUMBER!");continue;},
    };

    println!("The Secret num is {}", secret_num);
    println!("Your number {}", guess);

    match guess.cmp(&secret_num){
        std::cmp::Ordering::Less => println!("Too small"),
        std::cmp::Ordering::Greater => println!("Too Big"),
        std::cmp::Ordering::Equal => {println!("YOU GOT IT!");break;},
    }
}

}
