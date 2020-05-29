use crate::{
    note::NoteGroup,
    num::{DurationExt, NaturalRatio},
    tempo::TimeSignature,
};
use std::{error::Error, fmt, time::Duration};

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
    pub note_groups: Vec<NoteGroup>,
    _private: (),
}

impl Compass {
    pub fn new(
        signature: TimeSignature,
        note_groups: Vec<NoteGroup>,
    ) -> Result<Self, InvalidCompass> {
        let sum = note_groups.iter().map(|group| group.tempo.measure()).sum();

        if sum == signature.ratio() {
            Ok(Self { signature, note_groups, _private: () })
        } else {
            Err(InvalidCompass { expected: signature.ratio(), found: sum })
        }
    }

    pub fn nanos(&self) -> NaturalRatio {
        self.note_groups.iter().map(|note| note.tempo.nanos()).sum()
    }

    pub fn duration(&self) -> Duration {
        Duration::from_raw_nanos(self.nanos().to_integer())
    }
}
