fn main() {
    let ans = 42;
    assert_eq!(ans, 40); // panic!!!
}

// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `42`,
//  right: `40`', src/main.rs:3:5
// note: Run with `RUST_BACKTRACE=1` for a backtrace.
