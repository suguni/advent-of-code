
#[derive(PartialEq, Debug)]
struct USD(i32);

#[derive(PartialEq, Debug)]
struct GBP(i32);

#[derive(PartialEq, Debug)]
struct CAD(i32);

trait ToUSD {
    fn to_usd(&self) -> USD;

    fn convert<T: FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

trait FromUSD {
    fn from_usd(u: &USD) -> Self;
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 130) / 100)
    }
}

impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        Self((u.0 * 130) / 100)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait ToUSDValue<F> {
    fn to_usd_value(&self, _: F) -> f32;
}

trait FromUSDValue<F> {
    fn from_usd_value(&self, usd_value: f32) -> F;
}

// exchange rate
struct ExchangeRate {
    cad: f32,
    gbp: f32,
}

trait Exchange<F, T> {
    fn convert(&self, from: F) -> T;
}

impl ToUSDValue<GBP> for ExchangeRate {
    fn to_usd_value(&self, g: GBP) -> f32 {
        g.0 as f32 * self.gbp
    }
}

impl FromUSDValue<GBP> for ExchangeRate {
    fn from_usd_value(&self, usd_value: f32) -> GBP {
        GBP((usd_value / self.gbp) as i32)
    }
}

impl FromUSDValue<CAD> for ExchangeRate {
    fn from_usd_value(&self, usd_value: f32) -> CAD {
        CAD((usd_value / self.cad) as i32)
    }
}

impl <E, F, T> Exchange<F, T> for E
where E: ToUSDValue<F> + FromUSDValue<T>
{
    fn convert(&self, from: F) -> T {
        self.from_usd_value(self.to_usd_value(from))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    amount: A,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let g = GBP(200);
        let c = g.convert::<CAD>();
        assert_eq!(c.0, 338);
    }

    #[test]
    fn test_exchange_table() {
        let ex = ExchangeRate { cad: 0.7, gbp: 1.3 };
        let c: CAD = ex.from_usd_value(ex.to_usd_value(GBP(200)));
        assert_eq!(c, CAD(371));
    }

    #[test]
    fn test_exchange() {
        let ex = ExchangeRate { cad: 0.7, gbp: 1.3 };
        let c: CAD = ex.convert(GBP(200));
        assert_eq!(c, CAD(371));
    }
}