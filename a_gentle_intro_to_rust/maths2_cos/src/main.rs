fn main() {
    let x = 2.0 * std::f64::consts::PI;
    let abs_dif = (x.cos() - 1.0).abs();
    assert!(abs_dif < 1e-10);
}
