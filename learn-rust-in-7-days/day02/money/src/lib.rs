#[derive(PartialEq,Debug)]
pub struct USD(i32); // this is a tuple
#[derive(PartialEq,Debug)]
pub struct GBP(i32);
#[derive(PartialEq,Debug)]
pub struct CAD(i32);

pub trait ToUSD {
	fn to_usd(&self)->USD;
}

impl ToUSD for GBP {
	fn to_usd(&self) -> USD {
		USD((self.0 * 132) / 100) // floats are bad for money
	}
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(264));
    }
}
