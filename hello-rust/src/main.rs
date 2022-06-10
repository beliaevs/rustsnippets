use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut guess = String::new();
    // generate random number
    let a = 1;
    let b = 101;
    let secret_number = rand::thread_rng().gen_range(a .. b);
    println!("Guessed number is {}", secret_number);
    println!("Enter the number");
    loop
    {
        guess.clear();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess_num: u32 = guess.trim().parse().expect("Please type a number!");
        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            }
        }
    }

}
