// see return_values_with_tuple for comparison
fn main() {
    let s1 = String::from("hello");

    // change(&s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
	
	s.len()

} 	// since s is a reference, the value that it points to will not be dropped when s goes out of scope.
	// however, s is dropped and it's value which is the location of s1.

// s points to s1, not *directly* to the String.

//////////////////////////////////////
// fn change(some_string: &String) {
// 	some_string.push_str(", world!")	// should be an error, we can't mutate via a reference
// }