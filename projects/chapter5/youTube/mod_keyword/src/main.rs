mod camdenbloke {
	fn chicken(){
		println!("look at me, I'm a chicken!!!");
	}

	pub fn print_message() {
		println!("This message comes from inside of a module!");
		chicken();
	}

	pub mod water {
		pub fn print_message() {
			println!("Fire and water");
		}
	}
}

fn main() {
    println!("Hello, world!");
    camdenbloke::print_message();
    camdenbloke::water::print_message();
}
