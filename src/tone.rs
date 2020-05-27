use crate::num::Real;
use std::ops::Sub;

pub fn note_ratio() -> Real {
    Real::powf(1.0 / 2.0, 1.0 / 12.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Semitone {
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

impl Semitone {
    pub const TOTAL: usize = 12;

    #[allow(non_upper_case_globals)]
    pub const Db: Self = Semitone::Cs;

    #[allow(non_upper_case_globals)]
    pub const Eb: Self = Semitone::Ds;

    #[allow(non_upper_case_globals)]
    pub const Fb: Self = Semitone::E;

    #[allow(non_upper_case_globals)]
    pub const Es: Self = Semitone::F;

    #[allow(non_upper_case_globals)]
    pub const Gb: Self = Semitone::Fs;

    #[allow(non_upper_case_globals)]
    pub const Ab: Self = Semitone::Gs;

    #[allow(non_upper_case_globals)]
    pub const Bb: Self = Semitone::As;

    #[allow(non_upper_case_globals)]
    pub const Cb: Self = Semitone::B;

    #[allow(non_upper_case_globals)]
    pub const Bs: Self = Semitone::C;
}

impl Sub for Semitone {
    type Output = i8;

    fn sub(self, other: Self) -> Self::Output {
        self as u8 as i8 - other as u8 as i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardNote {
    pub note: Semitone,
    pub octave: u32,
}

impl HardNote {
    pub fn freq(self, a5: Real) -> Real {
        let a5_note = HardNote { note: Semitone::A, octave: 5 };
        let octaves = self.octave as i32 - a5_note.octave as i32;
        let notes = (self.note - a5_note.note) as i32;

        a5 * Real::powi(2.0, octaves) * note_ratio().powi(notes)
    }
}

impl Sub for HardNote {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        let total = Semitone::TOTAL as i32;
        let octave = self.octave as i32 - other.octave as i32;
        let note = (self.note - other.note) as i32;
        octave * total + note
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Note {
    Hard(HardNote),
    Ligature,
}
