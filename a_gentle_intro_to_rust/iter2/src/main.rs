fn main() {
    let arr = [10, 20, 30];
    for i in arr.iter() {
        // i is the value, not the index
        println!("{}", i);
    }

    // slices will be converted inplicitly to iterators
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}
