fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() { // iter borrows each name and returns the vector in it's original form.
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:?}", names); // this is still here.
}
