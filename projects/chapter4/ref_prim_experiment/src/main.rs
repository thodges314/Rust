fn take_prim(c: &u32) {
    println!("The number is {}.", c);
}

fn main() {
    let f = 22;
    take_prim(&f);
    println!("The number again is {}", f);
}
