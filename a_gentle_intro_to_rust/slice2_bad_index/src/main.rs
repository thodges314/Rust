fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);
    let maybe_last = slice.get(5);
    let success_last = if maybe_last.is_some() {
        *maybe_last.unwrap() // the type inside Sone() is a reference, so we need to defererence it
    } else {
        -1
    };
    // vvvv get returns a reference, so our unwrap_or alt value must be a reference (&-1) and the return must be dereferenced (*)
    let success_last_2 = *slice.get(5).unwrap_or(&-1);

    println!("first: {:?}", first);
    println!("last: {:?}\n", last);

    println!("first: {} {}", first.is_some(), first.is_none());
    println!("last: {} {}", last.is_some(), last.is_none());
    println!("first value: {}\n", first.unwrap()); // if no value this would return a panic - so don't unwrap last

    println!("success_last:{}", success_last);
    println!("success_last_2:{}", success_last_2);
}

// first: Some(1) <-- 'Option' type is either Some or None.
// last: None
//
// first: true false
// last: false true
// first value: 1
//
// success_last:-1
// success_last_2:-1
