#[derive(Debug)]
pub struct Bed {
	size:i32,
	count:u32,
}

#[derive(Debug)]
pub enum Room {
	Kitchen(i32),
	Bedroom(Bed),
	Lounge,
}

fn main() {
	let t = Room::Kitchen(4);
    println!("Hello from the {:?}.", t);
}
