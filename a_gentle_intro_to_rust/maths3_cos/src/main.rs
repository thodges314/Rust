use std::f64::consts;

fn main() {
    let x = 2.0 * consts::PI;
    let abs_dif = (x.cos() - 1.0).abs();
    assert!(abs_dif < 1e-10);
}
