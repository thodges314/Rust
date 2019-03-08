fn main() {
    let s1 = "Hello, ".to_string();
    let s2 = "world!".to_string();
    let s3 = s1 + &s2;

    println!("{}", s3);
    println!("{}", s2);
    // println!("{}", s1);
}
