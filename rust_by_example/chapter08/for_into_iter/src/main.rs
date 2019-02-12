fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() { // into_iter consumes the collection.  Notice no & on the identifier "Ferris".
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("{:?}", names); // this is gone.
}