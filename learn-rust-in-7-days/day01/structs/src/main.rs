
#[derive(Debug)] // derives ability to print string automatically
struct User {
	name:String,
	age:i32,
	height:i32,
	shoesize:i32,

}

impl User {
	fn simple_string(&self) -> String{ // will override simple_string - &self points to self as read-only
		format!("{} - {} years - {} cm - shoe: {}", self.name, self.age, self.height, self.shoesize)
	}

	fn grow(&mut self, h: i32){ // will mutate self and returns nothing.
		self.height += h;
	}

	fn die(self){	// consumes user
		println!("Dead: {}", self.simple_string());
	}
}

fn main() {
	let mut u = User{ // must make u mutable for grow
		name: "matt".to_string(), // converts &str to String
		age: 33,
		height: 250,
		shoesize: 10,
	};
    println!("User is {}", u.simple_string());
    u.grow(100);
    println!("User is {}", u.simple_string());
    u.die();
    // u.grow(10); // will make an error because dead
}
