use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut hm = HashMap::new();

    hm.insert(3, "hello");
    hm.insert(5, "world");

    // let r = hm.get(&3); // r returns an *option* enum with possible values Some(v) or None
    let r = match hm.get(&3) {
    	Some(v)=>v,
    	_=>"NOTHING",
    };

    println!("{}", r);
}
