#[derive(Debug)]
enum Room {
	Kitchen,
	Bedroom,
	Lounge,
}

fn main() {
	let t = Room::Kitchen;
    println!("Hello from the {:?}.", t);
}
