fn main() {
    let ints = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &ints;

    for s in slice.chunks(2) {
        println!("slice: {:?}", s);
    }
}
