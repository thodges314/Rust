fn main() {
    let mut s = String::new();
    // initially empty
    s.push('H'); // char
    s.push_str("ello"); // push_str() != push()
    s.push(' ');
    s += "World!";
    s.pop();

    assert_eq!(s, "Hello World");
}
