fn main() {
    let mut counter = 0;

    let result = loop { // set result to loop
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        println!("counter inside: {}", counter);
    };

    println!("counter: {}, result:{}", counter, result);

    assert_eq!(result, 20);
}