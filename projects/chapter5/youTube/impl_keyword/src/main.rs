struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn print_description(&self){
		println!("Rectangle: {}x{}", &self.height, &self.width,);
	}

	fn is_square(&self) -> bool {
		self.height == self.width
	}
}

fn main() {
    let my_rect = Rectangle {width: 10, height: 5,};

    let my_square = Rectangle {width: 10, height: 10,};

    my_rect.print_description();

    println!("Is my_rect a square? {}", my_rect.is_square());

    my_square.print_description();

    println!("Is my_square a square? {}", my_square.is_square());
}
