fn sum_values(values: &[i32]) -> i32 {
    // pass an array slice as a slice with a &.  & is ponounced 'borrow'.  Anything brrowed remains property of the original owner.
    let mut sum = 0;
    for i in 0..values.len() {
        // notice that we don't have to dereference the slice to use it
        sum += values[i];
    }
    sum
}

fn main() {
    let arr = [10, 20, 30, 40];
    let total = sum_values(&arr); // slice is passed with &
    println!("The sum is: {}", total);
}

// this passes *slices*, not arrays.  Slices are *views* of the underlying array structure.
