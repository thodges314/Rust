struct Person {
	name: String,
	age: u8,
}

impl ToString for Person {
	fn to_string(&self) -> String {
		return format!("My name is {} and I am age {}.", self.name, self.age);
	}
}

fn main() {
    let thom = Person {name: String::from("Thomas"), age: 38,};

    println!("{}", thom.to_string());
    // println!("{}", thom);
}
