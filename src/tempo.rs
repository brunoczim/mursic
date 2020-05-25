use crate::num::RatioExt;
use num::{rational::Ratio, Num};
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
    pub fn numeric(&self) -> Ratio<u128> {
        let base = *self as u32 as u128;
        Ratio::new(base * 2 - 1, base)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Note {
    pub base: Base,
    pub dot: Dot,
    pub tuplet: u32,
    pub whole_bpm: f32,
}

impl Note {
    pub fn measure(&self) -> Ratio<u128> {
        let divisor =
            Ratio::new(self.base.numeric() as u128 * 2, self.tuplet as u128);
        let dividend = Ratio::from(self.base.numeric() as u128);
        dividend / divisor
    }

    pub fn duration(&self) -> Duration {
        let numer = 60.0 / self.whole_bpm * self.dot.numeric().approx_to_f32();
        let denom = self.base.numeric() as f32 * (2.0 / self.tuplet as f32);
        let secs = numer / denom;
        let nanos = secs * Duration::from_secs(1).as_nanos() as f32;
        Duration::from_nanos(nanos as u64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature {
    pub numer: u32,
    pub denom: Base,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InvalidCompass;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Compass {
    pub signature: Signature,
    pub notes: Vec<Note>,
    _private: (),
}

impl Compass {
    pub fn new(
        signature: Signature,
        notes: Vec<Note>,
    ) -> Result<Self, InvalidCompass> {
        let mut sum = Ratio::zero();
        for note in &notes {
            let base = note.base.numeric();
        }

        Ok(Self { signature, notes, _private: () })
    }
}
