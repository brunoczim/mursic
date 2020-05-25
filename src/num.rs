use num::{cast::ToPrimitive, integer::Integer, rational::Ratio, Num};

pub type URational = Ratio<u128>;
pub type Rational = Ratio<i128>;
pub type Real = f64;

pub trait RatioExt {
    fn approx_to_real(&self) -> Real;
}

fn two<T>() -> T
where
    T: Num,
{
    T::one() + T::one()
}

impl<T> RatioExt for Ratio<T>
where
    T: Integer + Clone + ToPrimitive,
{
    fn approx_to_real(&self) -> Real {
        let mut ratio = self;
        loop {
            if let (Some(numer), Some(denom)) =
                (ratio.numer().to_f64(), ratio.denom().to_f64())
            {
                break numer / denom;
            }
            ratio = &Ratio::new(
                ratio.numer().clone() / two(),
                ratio.denom().clone() / two(),
            );
        }
    }
}
