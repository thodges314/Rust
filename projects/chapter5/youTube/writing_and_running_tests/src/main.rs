// the command 'cargo test' will run testing functions.

struct Rectangle {
	width: u8,
	height: u8,
}

impl Rectangle {
	fn is_square(&self) -> bool {
		self.width == self.height
	}
}

fn main() {
    
}

fn give_two() -> i32 {
	2
}

#[cfg(test)]
mod thodges314_tests {
	#[test]
	#[should_panic]
	fn test_basic() {
		assert!(1 == 1); // ok
		panic!("Oh no!");
	}

	#[test]
	fn test_equals() {
		assert_eq!(2, 1+1);
	}

	#[test]
	#[ignore]
	fn test_ne() {
		assert_ne!(2, 1+2);
	}

	#[test]
	fn gives_two_test() {
		assert_eq!(super::give_two(), 1+1);
		assert_ne!(super::give_two(), 1+2);
	}

	#[test]
	#[should_panic]
	fn test_structs() {
		let r = super::Rectangle {
			width: 50,
			height: 25,
		};
		assert!(r.is_square());
	}
}
