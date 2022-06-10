use std::io::stdin;

fn main() {
    println!("What is 3+2? Type your answer and press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read standard input");

    println!("Input string: {:#?}", input);
    // input string needs trimming
    if input.trim() == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}
