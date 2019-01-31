use std::io;

fn main() {
    println!("Input a number in Fahrenheit!");

    let mut fahr = String::new();
    io::stdin().read_line(&mut fahr).expect("Failed to read input!!");
    let fahr:f64 = fahr.trim().parse().expect("Chars in input string!!");
    // let fahr:f64 = match fahr.trim().parse() {
    // 	Ok(expr) => expr,
    // 	Err(err) => println!("Error! {}", err),
    // };
    let celc = (fahr - 32.0) * 9.0 / 5.0;

    println!("{}F is {}C", fahr, celc);
}
