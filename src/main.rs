use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "GUESSING GAME".dimmed().bold().blue());
    let mut chances: u32 = 0;
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    println!("secret number: {}", secret_number);
    loop {
        println!("input the number:");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("you guessed : {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too less".red()),
            Ordering::Equal => {
                println!("{}", "guessed the right answer".green());
                break;
            }
            Ordering::Greater => println!("{}", "too big".red()),
        }
        chances += 1;
        if chances >= 5 {
            println!("{}", "no more chances left".red());
            print!("{}", "the secret number was:".yellow());
            print!("{}", secret_number.to_string().yellow());
            break;
        };
    }
}

// fn rand() -> i32 {
//     static a: AtomicI32 = 3251;
//     a = ((a * a) / 100) % 1000;
//     return a;
// }

// fn rand_int((min, max): (i32, i32)) -> i32 {
//     return rand() % (max + 1 - min) + min;
// }
