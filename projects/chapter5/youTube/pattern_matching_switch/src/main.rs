fn main() {
    let number = 7;
    let name = "Bill";

    match number {
    	// Some(expr) => expr,
    	1 => println!("It is one."),
    	2...8 => println!("It is between 2 and 8 inclusive."),
    	10 | 11 => println!("It is either 10 or 11."),
    	_ => println!("It doesn't match."),
    	// None => expr,
    };

    match name {
    	"Thomas" => println!("Hello, Thomas!!"),
    	"Jill" => println!("Sup, Jill!"),
    	_ => println!("Hello, new person!"),
    };
}
