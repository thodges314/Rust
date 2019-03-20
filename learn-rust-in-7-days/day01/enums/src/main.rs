#[derive(Debug)]
pub struct Bed {
	size:i32,
	count:u32,
}

#[derive(Debug)]
pub enum Room {
	Kitchen(i32),
	Bedroom(Bed),
	Lounge(i32, String),
}

fn main() {
	use self::Room::*; // can 'use' locally
	let mut t = Kitchen(4);
    println!("Hello from the {:?}.", t);
    // t = Bedroom(Bed{size:50, count:2,});
    t = Lounge(5, "big".to_string());

    // match t {
    // 	Kitchen(n) => println!("The room is a kitchen with {} rooms.", n),
    // 	d=>println!("{:?}", d), 
    // }
    
    // let v = match t {
    // 	Kitchen(n) => n,
    // 	_=>0, 
    // } + 10;

    if let Kitchen(n) = t {
    	println!("It's a kitchen with {} cupboards.", n);
    }
    if let Lounge(n, s) = t {
    	println!("It's a {} lounge with {} martinis.", s, n);
    }

    // println!("The number is {}.", v);
}
