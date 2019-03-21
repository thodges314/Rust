use std::ops::Add;

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
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

    // println!("a = {:?}", a); // I cant work out how to not have this consume 'self'.
    println!("c = {:?}", c);


}
