use crate::num::{DurationExt, Natural, NaturalRatio};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum NoteValue {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
    SixtyForth = 64,
}

impl NoteValue {
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
        let note_value = Natural::from(*self as u32);
        NaturalRatio::new(note_value * 2 - 1, note_value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSignature {
    pub numer: u32,
    pub denom: NoteValue,
}

impl TimeSignature {
    pub fn ratio(&self) -> NaturalRatio {
        NaturalRatio::new(
            Natural::from(self.numer),
            Natural::from(self.denom.numeric()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteTime {
    pub note_value: NoteValue,
    pub dot: Dot,
    pub tuplet: u32,
    pub whole_bpm: NaturalRatio,
}

impl NoteTime {
    pub fn measure(&self) -> NaturalRatio {
        let dividend = self.dot.numeric();
        let divisor = NaturalRatio::new(
            Natural::from(self.note_value.numeric()) * 2,
            Natural::from(self.tuplet),
        );
        dividend / divisor
    }

    pub fn nanos(&self) -> NaturalRatio {
        let dividend =
            NaturalRatio::from(60) / self.whole_bpm * self.dot.numeric();
        let divisor = NaturalRatio::new(
            Natural::from(self.note_value.numeric()) * 2,
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
