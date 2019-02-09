#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// #[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
	fn area(&self) -> f32 {
		let height = (self.p1.y - self.p2.y).abs();
		let width = (self.p1.x - self.p2.x).abs();
		height * width
	}

	fn square(p1: Point, len: f32) -> Rectangle {
		let x_pt = p1.x;
		let y_pt = p1.y;

		Rectangle {
			p1: p1,
			p2: Point {x: x_pt + len, y: y_pt + len}
		}
	}
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let rectangle = Rectangle {
    	p1: Point {x: 0.0, y: 0.0},
    	p2: Point {x: 3.0, y: 4.0},
    };

    let square = Rectangle::square(Point {x: 1.0, y: 1.0}, 4.0);

    println!("Rectange: {:?}", rectangle);
    println!("Area rect: {}", rectangle.area());

    println!("Square: {:?}", square);
    println!("Area square: {}", square.area());

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
