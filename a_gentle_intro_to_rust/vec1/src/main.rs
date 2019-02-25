fn main() {
    let mut v = Vec::new(); // must be declared mut always
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0]; // will panic if out of range
    let maybe_first = v.get(0);

    println!("v is: {:?}", v);
    println!("v[0]: {:?}", first);
    println!("v.get(0): {:?}", maybe_first);
}
