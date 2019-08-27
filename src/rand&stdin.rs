extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // 乱数出力
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        // 標準入出力
        println!("Please input your number.");
        let mut input_string_2number = String::new();

        // 入力を得る
        io::stdin().read_line(&mut input_string_2number)
            .expect("Failed to read line");

        // stringの両端空白/タブをtrimしparse
        let input_string_2number: u32 = input_string_2number.trim().parse()
            .expect("Failed to read string to number.");

        // input_string_2numberを受取り比較対象型にしプリント
        match input_string_2number.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break
            }
        }
    }

    println!("game is done!!")
}