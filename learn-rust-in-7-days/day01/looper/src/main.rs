fn main() {
    println!("Hello, world!");
    loop_to_10();
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