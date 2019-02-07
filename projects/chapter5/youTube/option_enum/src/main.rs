fn char_at_intex(my_name: &String, idx: usize) {
	println!("Character at index {}: {}", idx, match my_name.chars().nth(idx) {
    	Some(expr) => expr.to_string(),
    	None => "No character".to_string(),
    });
}

fn get_occupation(name: &str) -> Option<&str> {
	match name {
		"Thomas" => Some("Software Engineed"),
		"Rani" => Some("EMT"),
		_ => None,
	}
}

fn main() {
    let my_name = String::from("Thomas");

    for idx in 0..=10 {
    	char_at_intex(&my_name, idx);
    }

    println!("Occupation is: {}", match get_occupation("Rani") {
    	Some(expr) => expr,
    	None => "No occupation found.",
    });
}
