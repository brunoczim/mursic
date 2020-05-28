use crate::num::Real;
use std::ops::Sub;

pub fn note_ratio() -> Real {
    Real::powf(1.0 / 2.0, 1.0 / 12.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Key {
    C = 0,
    Cs = 1,
    D = 2,
    Ds = 3,
    E = 4,
    F = 5,
    Fs = 6,
    G = 7,
    Gs = 8,
    A = 9,
    As = 10,
    B = 11,
}

impl Key {
    pub const TOTAL: usize = 12;

    #[allow(non_upper_case_globals)]
    pub const Db: Self = Key::Cs;

    #[allow(non_upper_case_globals)]
    pub const Eb: Self = Key::Ds;

    #[allow(non_upper_case_globals)]
    pub const Fb: Self = Key::E;

    #[allow(non_upper_case_globals)]
    pub const Es: Self = Key::F;

    #[allow(non_upper_case_globals)]
    pub const Gb: Self = Key::Fs;

    #[allow(non_upper_case_globals)]
    pub const Ab: Self = Key::Gs;

    #[allow(non_upper_case_globals)]
    pub const Bb: Self = Key::As;

    #[allow(non_upper_case_globals)]
    pub const Cb: Self = Key::B;

    #[allow(non_upper_case_globals)]
    pub const Bs: Self = Key::C;
}

impl Sub for Key {
    type Output = i8;

    fn sub(self, other: Self) -> Self::Output {
        self as u8 as i8 - other as u8 as i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch {
    pub key: Key,
    pub octave: u32,
}

impl Pitch {
    pub fn freq(self, a5: Real) -> Real {
        let a5_note = Pitch { key: Key::A, octave: 5 };
        let octaves = self.octave as i32 - a5_note.octave as i32;
        let notes = (self.key - a5_note.key) as i32;

        a5 * Real::powi(2.0, octaves) * note_ratio().powi(notes)
    }
}

impl Sub for Pitch {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        let total = Key::TOTAL as i32;
        let octave = self.octave as i32 - other.octave as i32;
        let note = (self.key - other.key) as i32;
        octave * total + note
    }
}
