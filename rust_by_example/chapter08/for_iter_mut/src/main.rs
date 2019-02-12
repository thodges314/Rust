fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
        // let mut test_name = name.to_uppercase();
        // *name = name.to_uppercase().as_str();
    }

    println!("{:?}", names);
}
