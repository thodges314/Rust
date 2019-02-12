fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        // if n % 15 == 0 {
        //     println!("fizzbuzz");
        // } else if n % 3 == 0 {
        //     println!("fizz");
        // } else if n % 5 == 0 {
        //     println!("buzz");
        // } else {
        //     println!("{}", n);
        // }
        if n % 3 == 0 {
        	print!("fizz");
        }
        if n % 5 == 0 {
        	print!("bizz");
        }
        if (n % 3 != 0) && (n % 5 != 0) {
        	print!("{}", n);
        }
        println!("");

        // Increment counter
        n += 1;
    }
}
