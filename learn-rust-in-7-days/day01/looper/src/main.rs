fn main() {
    println!("Hello, world!");
    loop_to_10();
    loop_to_10_for();
    loop_to_10_while();
    array_loop();
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
	let mut v = Vec::new();
	v.push(4);
	v.push(7);
	v.push(9);
	for n in v {
		println!("{}", n);
	}
}