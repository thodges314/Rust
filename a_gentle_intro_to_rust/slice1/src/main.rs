fn main() {
    let ints = [1, 2, 3, 4];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..]; // open range

    println!("ints: {:?}", ints);
    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
}

// ints: [1, 2, 3, 4]
// slice1: [1, 2] <- index 0, ends before index 2
// slice2: [2, 3, 4] <- index 1, doesn't end
