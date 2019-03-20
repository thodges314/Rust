fn main() {
	let b = higher(4,2,8);
	// format!("{} is highest!", b); <-- sets a string value to that result
    println!("{} is highest.", b);
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