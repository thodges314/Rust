extern crate rand; // in Cargo.toml dependencies
use rand::Rng;


fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11); // 1-10 inclusive
    println!("Random number: {}", random_number);

    // flip a coin
    let random_boolean = rand::thread_rng().gen_weighted_bool(2); // 1/2 chance of true
    println!("Is true? {}", random_boolean);
}
