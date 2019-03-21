use rand::Rng; // prelude is convention for publically exposing things in libraries
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn random() -> Self{
		let mut tr = rand::thread_rng();
		Point{
			x: tr.gen(),
			y: tr.gen(),
		}
	}
}

impl Add for Point{
	type Output=Point; // tells what 'type' we expect the output to be
	fn add(self, other: Point)->Self::Output{ // Self means type that self is.
		Point{
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

fn main() {
    let a = Point{x:3, y:5};
    let b = Point{x:30, y:50};

    let c = a + b;

    let d = Point::random();

    // println!("a = {:?}", a); // I cant work out how to not have this consume 'self'.
    println!("c = {:?}", c);
    println!("d = {:?}", d);


}