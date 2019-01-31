fn main() {
	// String type is mutable and therefore the pointer moves on a copy
    let s1 = String::from("Hello");
    let mut s2 = s1;
    s2.push_str(", world!");

    // println!("{}", s);
    println!("{}", s2);

    // use clone to make an actual copy
    let mut s3 = s2.clone();
    s3.push_str("!!!!");
    println!("cloned and modded: \"{}\"", s3);

    // string literal is actually copied because it's mutable with a fixed size.
    let sr1 = "String literal!";
    let sr2 = sr1;
    println!("sr1: \"{}\", sr2: \"{}\"", sr1, sr2);
}
