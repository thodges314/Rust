fn main() {
    let x = 5;
    let x = x + 1; // shadows here
    let x = x * 2; // shadows here
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len(); // instantly assigns new type.
    println!("{}", spaces);
}
