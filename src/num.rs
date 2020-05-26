use num::{cast::ToPrimitive, rational::Ratio, Num};
use std::{convert::TryFrom, time::Duration};

pub use std::{f64 as real, i128 as integer, u128 as natural};

pub type Natural = u128;
pub type Integer = i128;
pub type NaturalRatio = Ratio<Natural>;
pub type Rational = Ratio<Integer>;
pub type Real = f64;

pub trait RatioExt {
    fn approx_to_f64(&self) -> f64;

    fn approx_to_real(&self) -> Real {
        self.approx_to_f64() as Real
    }

    fn approx_to_f32(&self) -> f32 {
        self.approx_to_f64() as f32
    }
}

fn two<T>() -> T
where
    T: Num,
{
    T::one() + T::one()
}

impl<T> RatioExt for Ratio<T>
where
    T: num::Integer + Clone + ToPrimitive,
{
    fn approx_to_f64(&self) -> f64 {
        if let (Some(numer), Some(denom)) =
            (self.numer().to_f64(), self.denom().to_f64())
        {
            return numer / denom;
        }

        let mut ratio = self.clone();
        loop {
            if let (Some(numer), Some(denom)) =
                (ratio.numer().to_f64(), ratio.denom().to_f64())
            {
                break numer / denom;
            }
            ratio = Ratio::new(
                ratio.numer().clone() / two(),
                ratio.denom().clone() / two(),
            );
        }
    }
}

pub trait DurationExt {
    fn from_raw_nanos(nanos: u128) -> Self;
}

impl DurationExt for Duration {
    fn from_raw_nanos(nanos: u128) -> Self {
        let one_sec = Duration::from_secs(1).as_nanos();
        let secs = u64::try_from(nanos / one_sec).expect("Unsupported nanos");
        let subsec_nanos =
            u32::try_from(nanos % one_sec).expect("Unsupported nanos");

        Self::new(secs, subsec_nanos)
    }
}
