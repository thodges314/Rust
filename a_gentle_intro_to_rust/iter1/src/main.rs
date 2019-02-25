fn main() {
    let mut iter = 0..3; // defined directly
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

// iter is an object with a 'nect' method that returns an 'Option'.  as long at the Option is not 'None' we can call next.
