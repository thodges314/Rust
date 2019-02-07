fn plus_one(x: Option<i32>)-> Option<i32> {
	match x {
		Some(i) => Some(i + 1),
		None => None,
	}
}

fn print_limited_values(x: Option<i32>) {
	match x {
		Some(1) => println!("one"),
		Some(5) => println!("five"),
		Some(6) => println!("six"),
		None => println!("none"),
		_ => println!("other"),
	}
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(six);
    let none = plus_one(None);

    print_limited_values(five);
    print_limited_values(six);
    print_limited_values(seven);
    print_limited_values(none);
}
