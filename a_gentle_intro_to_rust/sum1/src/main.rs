fn main() {
    // let sum1: u128 = (1..31).product();
    let sum1: i32 = (1..101).sum();
    println!("Sum is: {}", sum1);

    let sum2: i64 = [10, 20, 30].iter().sum();
    println!("sum is: {}", sum2);
}
