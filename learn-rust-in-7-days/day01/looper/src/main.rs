fn main() {
    println!("Hello, world!");
    loop_to_10();
    loop_to_10_for();
    loop_to_10_while();
    array_loop();
    array_loop_continue();
    array_loop_break();
}

fn loop_to_10() {
	let mut n = 0;
	loop {
		n += 1;
		println!("Hello!");
		if n >= 10 {
			return;
		}
	}
}

fn loop_to_10_for() { // rust can loop on any *iterator*
	for n in 0..10 { // 10 is not inclusive
		println!("Hello {}!", n);
	}
}

fn loop_to_10_while() {
	let mut n = 0;
	while n < 10 {
		println!("Hello while {}", n);
		n += 1;
	}
}

fn array_loop() {
	// let mut v = Vec::new();
	// v.push(4);
	// v.push(7);
	// v.push(9);
	let v = vec![4,7,9,10];
	for n in v {
		println!("{}", n);
	}
}

fn array_loop_continue() {
	let v = vec![4,7,8,9,11,10];
	for n in v {
		if n%2 ==0 {
			continue; //skips remainder of loop code and goes to next member
		}
		println!("{} - cont", n);
	}
}

fn array_loop_break() {
	let v = vec![4,7,8,9,11,10];
	for n in v {
		if n ==11 {
			break; //breaks out of look
		}
		println!("{} - break", n);
	}
}