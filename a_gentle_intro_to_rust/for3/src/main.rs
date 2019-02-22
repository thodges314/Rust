fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{}: {}", even_odd, i);
    }
}

// even: 0
// odd: 1
// even: 2
// odd: 3
// even: 4
