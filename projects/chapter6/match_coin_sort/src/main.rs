#[derive(Debug)]

enum UsState {
	Alabama, Alaska, Arizona, Arkansas, California, Colorado, Connecticut,
	Delaware, DistrictofColumbia, Florida, Georgia, Hawaii, Idaho, Illinois,
	Indiana, Iowa, Kansas, Kentucky, Louisiana, Maine, Maryland, Massachusetts,
	Michigan, Minnesota, Mississippi, Missouri, Montana, Nebraska, Nevada,
	NewHampshire, NewJersey, NewMexico, NewYork, NorthCarolina, NorthDakota,
	Ohio, Oklahoma, Oregon, Pennsylvania, RhodeIsland, SouthCarolina, SouthDakota,
	Tennessee, Texas, Utah, Vermont, Virginia, Washington, WestVirginia, Wisconsin, Wyoming,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn valueincents(coin: Coin)-> u32 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		},
	}
}

fn main() {
	let pvalue = valueincents(Coin::Penny);
    println!("a penny: {}", pvalue);
	let nvalue = valueincents(Coin::Nickel);
    println!("a nickel: {}", nvalue);
    let dvalue = valueincents(Coin::Dime);
    println!("a dime: {}", dvalue);
    let qvalue = valueincents(Coin::Quarter(UsState::California));
    println!("a quarter: {}", qvalue);
}
