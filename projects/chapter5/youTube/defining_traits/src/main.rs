struct Person {
	name: String,
	age: u8,
}

trait HasVoiceBox {
	// Speak
	// Check if can speak
	fn speak(&self); // if have a voicebox then can speak
	fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
	fn speak(&self) {
		println!("Hello, my name is {}.", self.name);
	}

	fn can_speak(&self) -> bool {
		self.age > 1
	}
}

fn main() {
    let thom = Person {name: String::from("Thomas"), age: 38,};
    let baby = Person {name: String::from("baby"), age: 1,};
    if thom.can_speak() {thom.speak();};
    if baby.can_speak() {baby.speak();};
}
