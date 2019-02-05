fn main() {
    let mut my_string = String::from("How's it going?  My name is Thomas.");

    println!("Length: {}", my_string.len());

    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
     	print!("{} ", token);
     } 
     print!("\n");

     println!("Does the String contain 'Thomas?' {}", my_string.contains("Thomas"));
     println!("Does the String contain 'Dom?' {}", my_string.contains("Dom"));

     my_string.push_str("  Welcome to a tutorial on strings.");

     println!("{}", my_string);
}
