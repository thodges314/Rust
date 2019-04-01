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
    fn to_uv(&self, f: F) -> f32;
}

pub trait FromUSDv<F> {
    fn from_uv(&self, f: f32) -> F;
}

pub struct Ex {
    cad: f32,
    gbp: f32,
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        g.0 * self.gbp
    }
}

impl ToUSDv<CAD> for Ex {
    fn to_uv(&self, c: CAD) -> f32 {
        c.0 * self.cad
    }
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, f: f32) -> CAD {
        CAD(round_to_two_decimals(f / self.cad))
    }
}

impl FromUSDv<GBP> for Ex {
    fn from_uv(&self, f: f32) -> GBP {
        GBP(round_to_two_decimals(f / self.gbp))
    }
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

impl<E, F, T> Exchange<F, T> for E
where
    E: ToUSDv<F> + FromUSDv<T>,
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
        let cc: CAD = ex.convert(g);
        let gg: GBP = ex.convert(c);
        println!("{:?}", gg);
        assert_eq!(cc, CAD(352.0));
        assert_eq!(gg, GBP(200.0));
    }
}
