fn main() {
    // let numbers = [1,2,3,4,5];		// both defintions are valid

    // let numbers: [i32; 5] = [1,2,3,4,5];

    let numbers = [2; 400]; // gives an array with 400 copies of 2.

    for n in numbers.iter(){
    	println!("{}", n);
    }

    for i in 0..numbers.len() {	// this is  length in such an index, not the last item.
    	println!("{}", numbers[i]);
    }


}
