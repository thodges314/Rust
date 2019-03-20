fn main() {
	let b = higher(4,2,8);
	let o = other(5,9);
	// format!("{} is highest!", b); <-- sets a string value to that result
    println!("{} is highest.", b);
    println!("{} is other", o);
}

fn higher(a:i32, b:u32, c:i8) -> i32 {
	let mut res = a;
	if b as i32 > res {
		res = b as i32;
	}
	if c as i32 > res {
		res = c as i32;
	}
	res
}

fn other(a: i32, b:i32)->i32 {
	let mut c = a + b;
	c = c % 4;
	c = c / 2;
	c += 1;
	c
}