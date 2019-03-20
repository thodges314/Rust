fn main() {
    println!("Hello, world!");
    loop_to_10();
    loop_to_10_for();
    loop_to_10_while();
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

fn loop_to_10_for() {
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