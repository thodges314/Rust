fn main() {
    // let s = "¡";
    // println!("{}", &s[0..1]); <-- char is two bytes long
    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", words);

    let mut other_words = Vec::new();
    other_words.extend(text.split_whitespace());
    println!("{:?}", other_words);

    let stripped: String = text.chars().filter(|ch| !ch.is_whitespace()).collect();
    println!("{}", stripped);
}
