extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // 乱数出力
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);


    // 標準入出力
    println!("Please input your guess.");
    let mut guess01 = String::new();
    let mut guess02 = String::new();

    io::stdin().read_line(&mut guess01)
        .expect("Failed to read line");

    io::stdin().read_line(&mut guess02)
        .expect("Failed to read line");

    println!("You guessed: {} - {}", guess01 , guess02);
}