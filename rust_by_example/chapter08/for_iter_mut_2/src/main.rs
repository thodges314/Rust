fn main() {
    let nums = &mut [1, 2, 3];

    for number in nums.iter_mut(){
    	*number *= 2;
    }

    println!("{:?}", nums);
}
