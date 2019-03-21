#[derive(PartialEq,Debug)]
pub struct USD(f32); // this is a tuple
#[derive(PartialEq,Debug)]
pub struct GBP(f32);
#[derive(PartialEq,Debug)]
pub struct CAD(f32);

pub trait ToUSD {
	fn to_usd(&self)->USD;
}

pub trait FromUSD {
	fn from_usd(u:&USD)->Self;
}

impl ToUSD for GBP {
	fn to_usd(&self) -> USD {
		USD((self.0 * 132.0) / 100.0) // floats are bad for money
	}
}

impl FromUSD for CAD {
	fn from_usd(u:&USD)->Self {
		CAD((u.0 * 133.0) / 100.0)
	}
}

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
    }
}
