use crate::{
    manner::Manner,
    num::{DurationExt, NaturalRatio},
    tempo::{NoteTime, TimeSignature},
};
use std::{error::Error, fmt, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note {
    pub tempo: NoteTime,
    pub pitch: Manner,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InvalidCompass {
    pub found: NaturalRatio,
    pub expected: NaturalRatio,
}

impl fmt::Display for InvalidCompass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Invalid compass length: found {}, expected {}",
            self.found, self.expected
        )
    }
}

impl Error for InvalidCompass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Compass {
    pub signature: TimeSignature,
    pub notes: Vec<Note>,
    _private: (),
}

impl Compass {
    pub fn new(
        signature: TimeSignature,
        notes: Vec<Note>,
    ) -> Result<Self, InvalidCompass> {
        let sum = notes.iter().map(|note| note.tempo.measure()).sum();

        if sum == signature.ratio() {
            Ok(Self { signature, notes, _private: () })
        } else {
            Err(InvalidCompass { expected: signature.ratio(), found: sum })
        }
    }

    pub fn nanos(&self) -> NaturalRatio {
        self.notes.iter().map(|note| note.tempo.nanos()).sum()
    }

    pub fn duration(&self) -> Duration {
        Duration::from_raw_nanos(self.nanos().to_integer())
    }
}
