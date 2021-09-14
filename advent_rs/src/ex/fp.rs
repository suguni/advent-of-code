
fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = (bits >> 31) & 1;
    let expo = (bits >> 23) & 0xff;
    let frac = bits & 0x7fffff;

    (sign, expo, frac)
}

fn decode(sign: u32, expo: u32, frac: u32) -> (f32, f32, f32) {
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        if (frac & mask) != 0 {
            mantissa += 2_f32.powi(i - 23);
        }
    }

    ((-1_f32).powi(sign as i32),
     2_f32.powi(expo as i32 - 127),
     mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powi(-7)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        let v: f32 = 42.42;
        let (s, e, f) = to_parts(v);
        let (s, e, f) = decode(s, e, f);
        assert!(v.eq(&from_parts(s, e, f)));
    }
}