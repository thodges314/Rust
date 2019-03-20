
#[derive(Debug)] // derives ability to print string
struct User {
	name:String,
	age:i32,
	height:i32,
	shoesize:i32,

}
fn main() {
	let u = User{
		name: "matt".to_string(), // converts &str to String
		age: 33,
		height: 250,
		shoesize: 10,
	};
    println!("User is {:?}", u);
}
