const GLOBAL_CONSTANT: u8 = 2;

fn main() {
	const CALLING_CONSTANT: u8 = 3;
    println!("Hello, world!");
    println!("GLOBAL_CONSTANT: {}", GLOBAL_CONSTANT);
    println!("CALLING_CONSTANT: {}", CALLING_CONSTANT);
    inner_scope();
    // println!("SCOPE_CONSTANT: {}", SCOPE_CONSTANT);
}

fn inner_scope() {
	const SCOPE_CONSTANT: u8 = 1;
	println!("GLOBAL_CONSTANT: {}", GLOBAL_CONSTANT);
	println!("SCOPE_CONSTANT: {}", SCOPE_CONSTANT);
	// println!("CALLING_CONSTANT: {}", CALLING_CONSTANT);
}

// conclusion: constants follow same scope rules as other vars.