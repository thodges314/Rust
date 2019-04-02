#[derive(PartialEq, Debug)]
pub struct USD(f32);
#[derive(PartialEq, Debug)]
pub struct GBP(f32);
#[derive(PartialEq, Debug)]
pub struct CAD(f32);

fn round_to_two_decimals(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

pub trait ToUSDv<F> {
    // F is a *type* of currency (we don't know what type)
    fn to_uv(&self, f: F) -> f32; // here we take F as a parameter
}

pub trait FromUSDv<F> {
    // F is the type of currency (we don't know what type)
    fn from_uv(&self, f: f32) -> F; // this returns a mysterious type F
}

pub trait Exchange<F, T> {
    // takes a [F]rom type and a [T]o type.
    fn convert(&self, f: F) -> T;
}

pub struct Ex {
    // this *exchange* has two conversion rates
    cad: f32,
    gbp: f32,
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        // same signature as trait type, except F is filled in as GBP
        g.0 * self.gbp
    }
}

impl ToUSDv<CAD> for Ex {
    fn to_uv(&self, c: CAD) -> f32 {
        c.0 * self.cad
    }
}

impl FromUSDv<CAD> for Ex {
    // same signature as trait type, except F is filled in as CAD
    fn from_uv(&self, f: f32) -> CAD {
        CAD(round_to_two_decimals(f / self.cad))
    }
}

impl FromUSDv<GBP> for Ex {
    fn from_uv(&self, f: f32) -> GBP {
        GBP(round_to_two_decimals(f / self.gbp))
    }
}

impl<E, F, T> Exchange<F, T> for E
// implements the trait for E, F and T - we have to delcare these in impl because they are unknown types
where
    E: ToUSDv<F> + FromUSDv<T>, // implements where E implements ToUSDv for the from type and FromUSDv for the to type
                                // this will be applied to the Ex type because the Ex type implements ToUSD on GBP and FromUSD on CAD
{
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200.0);
        let c = CAD(352.0);
        let ex = Ex {
            cad: 0.75,
            gbp: 1.32,
        };
        // let cc: CAD = ex.from_uv(ex.to_uv(g));
        // let gg: GBP = ex.from_uv(ex.to_uv(c));
        let cc: CAD = ex.convert(g); // specify the type because there are several FromUSDv available to Ex
        let gg: GBP = ex.convert(c);
        println!("{:?}", gg);
        assert_eq!(cc, CAD(352.0));
        assert_eq!(gg, GBP(200.0));
    }
}
