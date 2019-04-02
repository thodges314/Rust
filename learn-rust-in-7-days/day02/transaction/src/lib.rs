#[derive(PartialEq, Debug, Clone)]
pub struct USD(f32);
#[derive(PartialEq, Debug, Clone)]
pub struct GBP(f32);
#[derive(PartialEq, Debug, Clone)]
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

pub trait Account {
    fn id(&self) -> i32;
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

pub trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>);
}

pub struct Ex {
    ac_id: i32,
    cad: f32,
    gbp: f32,
}

#[derive(PartialEq, Debug)]
pub struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    amount: A, // amount type is generic
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

impl Account for Ex {
    fn id(&self) -> i32 {
        self.ac_id
    }
}

impl<E, F, T> Exchange<F, T> for E
where
    E: ToUSDv<F> + FromUSDv<T>,
{
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}

impl<E, F, T> ExchangeAccount<F, T> for E
where
    E: Exchange<F, T> + Account,
    F: Clone, // will need to put info from this into From
{
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>) {
        let ft = Transaction {
            from_id: f_id,
            to_id: self.id(),
            amount: amount.clone(),
        }; // moving into our account - cloning amount because will be used elsewhere
        let tt = Transaction {
            from_id: self.id(),
            to_id: t_id,
            amount: self.convert(amount),
        };
        (ft, tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ex = Ex {
            ac_id: 30,
            cad: 0.75,
            gbp: 1.32,
        };
        let (ft, tt) = ex.exchange(20, 40, GBP(200.0));
        assert_eq!(
            ft,
            Transaction {
                from_id: 20,
                to_id: 30,
                amount: GBP(200.0)
            }
        );
        assert_eq!(
            tt,
            Transaction {
                from_id: 30,
                to_id: 40,
                amount: CAD(352.0)
            }
        );
    }
}
