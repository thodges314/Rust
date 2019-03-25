#[derive(PartialEq, Debug)] // checks for equality where there is not a full equivalance relation.
pub struct USD(f32); // this is a tuple
#[derive(PartialEq, Debug)]
pub struct GBP(f32);
#[derive(PartialEq, Debug)]
pub struct CAD(f32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T: FromUSD>(&self) -> T {
        // worts when T implements FromUSD
        // takes a type parameter of T and self, when called in the code this function will be created for each type
        T::from_usd(&self.to_usd())
    }
}

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self;
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 132.0) / 100.0) // floats are bad for money
    }
}

impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        CAD((u.0 * 133.0) / 100.0)
    }
}

// Use cargo test to test these.

#[cfg(test)]
mod tests {
    use super::*;
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
