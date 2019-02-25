fn mutates(x: &mut f64) {
    *x *= 2.0;
}

fn main() {
    let mut res = 2.0;
    mutates(&mut res);
    println!("Res is: {}", res);
}

// This is more how C would do it than C++. You have to explicitly pass the reference (with &) and explicitly dereference with *. And then throw in mut because it's not the default. (I've always felt that C++ references are too easy to miss compared to C.)

// Basically, Rust is introducing some friction here, and not-so-subtly pushing you towards returning values from functions directly. Fortunately, Rust has powerful ways to express things like "operation succeeded and here's the result" so &mut isn't needed that often. Passing by reference is important when we have a large object and don't wish to copy it.
