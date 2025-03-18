// Generates a random integer between 1 and 100 inclusive
fn main() {
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(1..=100);
    println!("The random number is {}", number);
}
