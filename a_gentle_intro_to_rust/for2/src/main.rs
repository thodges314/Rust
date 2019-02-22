fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even: {}", i);
        } else {
            println!("odd: {}", i);
        }
    }
}

// even: 0
// odd: 1
// even: 2
// odd: 3
// even: 4
