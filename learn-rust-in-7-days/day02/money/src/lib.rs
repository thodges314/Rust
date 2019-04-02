#[derive(PartialEq, Debug)] // PartialEq checks for equality where there is not a full equivalance relation.
pub struct USD(f32); // this is a tuple
#[derive(PartialEq, Debug)] // both PartialEq and Debug needed for assert_eq
pub struct GBP(f32);
#[derive(PartialEq, Debug)]
pub struct CAD(f32);

pub trait ToUSD {
    fn to_usd(&self) -> USD; // required methods for implementstion have a semicolon instead of swiglly braces
    fn convert<T: FromUSD>(&self) -> T {
        // T can be *any* type (this is generics)
        // <T: FromUSD> meant that this worts when T implements FromUSD
        // takes a type parameter of T and self, when called in the code this function will be created for each type
        T::from_usd(&self.to_usd()) // we are implementing this method so it's not required.
    }
}

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self; // is a constructod so it doesn't require self - returns type Self (whatever type is implementing fromUSD)
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 132.0) / 100.0) // floats are bad for money - .0 is first member of tuple of one element
    }
}

impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        CAD((u.0 * 133.0) / 100.0)
    }
}

// Use `cargo test` to test these.

#[cfg(test)]
mod tests {
    use super::*; // this module uses everything from it's containing module
    #[test]
    fn it_works() {
        let g = GBP(200.0);
        let u = g.to_usd();
        assert_eq!(u, USD(264.0));
        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(351.12));
        let c2: CAD = g.convert();
        assert_eq!(c2, c);
    }
}
