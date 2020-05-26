use num::{cast::ToPrimitive, rational::Ratio, Num};

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
