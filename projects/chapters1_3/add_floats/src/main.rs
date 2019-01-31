fn main() {
	let x_1 = 0.1;
	let x_2 = 0.1;
	println!("{}", x_1+x_2);
	let x_1: f32 = 0.1;
	let x_2: f32 = 0.1;
    println!("{}", x_1+x_2);

    let c = 'z';
    let heart_eyed_cat='😻';
    println!("{}{}{}", c, heart_eyed_cat, c);

    let number = 3;
    // if !!number { // does not work, does not return bool
    if number != 0 {
    	println!("The number is defined and non-zero.");
    } else {
    	println!("The number is not defined or is zero!");
    }

    println!("loop");

    let mut x = 1;

    loop {
    	print!("{}, ", x);
    	if x == 100 { break };
    	x = x + 1;
    }

    println!("\nwhile loop");

    x = 1;

    while x <= 100 {
    	print!("{}, ", x);
    	x = x + 1;
    }

    println!("\nFor loop with array");

    for number in (1..6).rev() { // makes Range of number 1 to 5 (ends before 6) and reverses.
    	println!("T minus {}", number);
    }
    println!("liftoff!");

}
