use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("guessed number: {secret_number}");
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed");
    println!("{guess}");
}
