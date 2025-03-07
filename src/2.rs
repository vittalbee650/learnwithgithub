// Generate a random number between 1 and 6
let mut rng = rand::thread_rng();
let random_number = rng.gen_range(1..=6);

// Print the random number
println!("The random number is: {}", random_number);
