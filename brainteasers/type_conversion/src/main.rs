#[warn(clippy::pedantic)]

fn main() {
    let x: u64 = 4_294_967_296;
    let y = x as u32;
    if x == y as u64 {
        println!("{} equals {}.", x, y);
    } else {
        println!("{} does not equal {}.", x, y);
    }
}
