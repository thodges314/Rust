fn dump(arr: &[i32]) {
    println!("array is: {:?}", arr);
}

fn main() {
    let mut vec = Vec::new();

    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);

    dump(&vec); // the borrow operator (&) coerces vector into a slice

    let slice = &vec[1..];
    println!("slice is: {:?}", slice);
}

// array is: [10, 20, 30, 40]
// slice is: [20, 30, 40]
