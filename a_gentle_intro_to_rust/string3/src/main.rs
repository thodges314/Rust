fn main() {
    // let multilingual = "Hi! ¡Hola! привет!";
    let multilingual = "Hi! Hola!"; // removing ¡ and привет! the chars match the number of bytes
    for ch in multilingual.chars() {
        print!("{}, ", ch);
    }
    println!("");

    println!("len: {}", multilingual.len());
    println!("chars: {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian 'hi': {}", hi);
    }
}
