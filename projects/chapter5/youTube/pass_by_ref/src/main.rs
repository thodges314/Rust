struct Color {
	red: u8, // 0-255
	green: u8,
	blue: u8,
}

fn main() {
	let blue = Color { red: 0, green: 0, blue: 255, };

	print_color(&blue);
	print_color(&blue);
}

fn print_color(c: &Color) {
	println!("color: Red:{}, Green:{}, Blue:{}", c.red, c.green, c.blue);
}
