fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count % 3 == 0 {
            println!("three");

            // Skip the rest of this iteration
            continue; // this is important.  counting to 15 caused an error because is skipped the break-out bit
        }

        println!("{}", count);

        if count == 25 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}