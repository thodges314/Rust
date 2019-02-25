fn mutates(x: &mut f64) {
    *x *= 2.0;
}

fn main() {
    let mut res = 2.0;
    mutates(&mut res);
    println!("Res is: {}", res);
}
