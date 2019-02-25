fn main() {
    // let ints = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ints = Vec::new();
    ints.extend(0..101);
    let slice = &ints;

    for s in slice.chunks(4) {
        println!("slice: {:?}", s);
    }
}
