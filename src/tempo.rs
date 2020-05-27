use crate::num::{DurationExt, Natural, NaturalRatio};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Base {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
    SixtyForth = 64,
}

impl Base {
    pub fn numeric(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Dot {
    None = 1,
    Single = 2,
    Double = 4,
}

impl Dot {
    pub fn numeric(&self) -> NaturalRatio {
        let base = Natural::from(*self as u32);
        NaturalRatio::new(base * 2 - 1, base)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature {
    pub numer: u32,
    pub denom: Base,
}

impl Signature {
    pub fn ratio(&self) -> NaturalRatio {
        NaturalRatio::new(
            Natural::from(self.numer),
            Natural::from(self.denom.numeric()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note {
    pub base: Base,
    pub dot: Dot,
    pub tuplet: u32,
    pub whole_bpm: NaturalRatio,
}

impl Note {
    pub fn measure(&self) -> NaturalRatio {
        let dividend = self.dot.numeric();
        let divisor = NaturalRatio::new(
            Natural::from(self.base.numeric()) * 2,
            Natural::from(self.tuplet),
        );
        dividend / divisor
    }

    pub fn nanos(&self) -> NaturalRatio {
        let dividend =
            NaturalRatio::from(60) / self.whole_bpm * self.dot.numeric();
        let divisor = NaturalRatio::new(
            Natural::from(self.base.numeric()) * 2,
            Natural::from(self.tuplet),
        );
        let secs = dividend / divisor;
        let nanos =
            secs * NaturalRatio::from(Duration::from_secs(1).as_nanos());

        nanos
    }

    pub fn duration(&self) -> Duration {
        Duration::from_raw_nanos(self.nanos().to_integer())
    }
}
