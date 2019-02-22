fn fact_recursive(x: u64) -> u64 {
    if x == 0 {
        1
    } else {
        x * fact_recursive(x - 1)
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
    println!("fact_recursive(20) : {}", fact_recursive(20));
    println!("fact_iterative(20) : {}", fact_iterative(20));
    println!("fact_iterators(20) : {}", fact_iterators(20));
}
