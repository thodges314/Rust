fn main() {
    let ints = [1, 2, 3, 4];
    // let ints = [1..101];
    let slice = &ints;
    // let slice = 1..101;

    println!("{:?}", ints);
    for s in slice.windows(2) {
        println!("window: {:?}", s);
    }
}
