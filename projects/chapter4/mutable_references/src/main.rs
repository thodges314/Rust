// compare to references_and_borrowing
fn main() {
    let mut s = String::from("Hello");

    {
    	let r2 = &mut s;
    	change(r2);
    } // r2 goes out of scope here

    let r1 = &mut s;

    change(r1);


    // println!("s: {}, r1: {}", s, r1); // this has been changed to 'Hello, world!!' here.
    println!("r1: {}", r1);
}

fn change(some_string: &mut String){
	some_string.push_str(", world!!");
}


// we can only have one mutable borrow or loads of immutable borrows.