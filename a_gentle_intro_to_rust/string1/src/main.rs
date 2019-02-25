fn dump(s: &str) {
    println!("string: `{}`", s);
}

fn main() {
    let text = "Hello, world!";
    let s = text.to_string();

    dump(text);
    dump(&s); // borrow operator coerces String into &str - just as Vec<T> can be coerced into &[T]
}
