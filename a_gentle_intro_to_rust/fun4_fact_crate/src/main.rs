extern crate num;

use num::BigUint;

fn fact_recursive(x: num::BigUint) -> num::BigUint {
    if x == BigUint::from(0 as u64) {
        BigUint::from(1 as u64)
    } else {
        x * fact_recursive(x - BigUint::from(1 as u64))
    }
}

fn fact_iterative(x: u64) -> u64 {
    let mut acc: u64 = 1;
    for num in 2..=x {
        acc *= num;
    }
    acc
}

fn fact_iterators(x: u64) -> u64 {
    (1..=x).product()
}

fn main() {
    println!(
        "fact_recursive(20) : {}",
        fact_recursive(BigUint::from(20 as u64))
    );
    println!("fact_iterative(20) : {}", fact_iterative(20));
    println!("fact_iterators(20) : {}", fact_iterators(20));
}
