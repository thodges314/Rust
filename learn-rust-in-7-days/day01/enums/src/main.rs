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
	use self::Room::*; // can 'use' locally
	let mut t = Kitchen(4);
    println!("Hello from the {:?}.", t);
    // t = Bedroom(Bed{size:50, count:2,});

    // match t {
    // 	Kitchen(n) => println!("The room is a kitchen with {} rooms.", n),
    // 	d=>println!("{:?}", d), 
    // }
    // 
    let v = match t {
    	Kitchen(n) => n,
    	_=>0, 
    };

    println!("The number is {}.", v);
}
