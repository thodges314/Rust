fn take_prim(c: &u32) {
    println!("The number is {}.", c);
}

fn take_prim_no_ref(c: u32) {
    println!("Without a ref its: {}", c);
}

fn main() {
    let f = 22;
    take_prim(&f);
    println!("The number again is {}", f);
    take_prim_no_ref(f);
    println!("And once again here it's {}.", f);
}
