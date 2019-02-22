fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("The sum is: {}", sum);
}

// error[E0277]: cannot add-assign `{integer}` to `{float}`
//  --> src/main.rs:4:13
//   |
// 4 |         sum += i;
//   |             ^^ no implementation for `{float} += {integer}`
//   |
//   = help: the trait `std::ops::AddAssign<{integer}>` is not implemented for `{float}`

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0277`.
// error: Could not compile `add3`.

// To learn more, run the command again with --verbose.
