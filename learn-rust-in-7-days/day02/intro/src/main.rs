use std::ops::Add; // lets us override the Add (+) function for the Point struct

#[derive(Debug, Copy, Clone)]
// by derriving Copy and Clone, Add will *copy* self and other rather than consume them
// by derriving Debug we can print with {:?}
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // tells what 'type' we expect the output to be
    fn add(self, other: Point) -> Self::Output {
        // Self means type that self is. -> needs type Output
        // we don't need 'pub' because that's implied by implementing an interface
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let a = Point { x: 3, y: 5 };
    let b = Point { x: 30, y: 50 };

    let c = a + b;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
