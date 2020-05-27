use crate::{
    num::{DurationExt, Natural, NaturalRatio},
    tempo,
    tone,
};
use std::{error::Error, fmt, mem, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note {
    pub tempo: tempo::Note,
    pub tone: tone::Note,
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
    pub signature: tempo::Signature,
    pub notes: Vec<Note>,
    _private: (),
}

impl Compass {
    pub fn new(
        signature: tempo::Signature,
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

#[derive(Debug, Clone)]
pub struct CompassBuilder {
    signature: tempo::Signature,
    curr_bpm: NaturalRatio,
    curr_tuplet: u32,
    notes: Vec<Note>,
}

impl Default for CompassBuilder {
    fn default() -> Self {
        Self {
            signature: tempo::Signature {
                numer: 4,
                denom: tempo::Base::Quarter,
            },
            curr_bpm: NaturalRatio::from(120),
            curr_tuplet: 2,
            notes: Vec::new(),
        }
    }
}

impl CompassBuilder {
    pub fn signature(&mut self, signature: tempo::Signature) -> &mut Self {
        self.signature = signature;
        self
    }

    pub fn get_signature(&self) -> tempo::Signature {
        self.signature
    }

    pub fn curr_bpm(
        &mut self,
        base: tempo::Base,
        bpm: NaturalRatio,
    ) -> &mut Self {
        self.curr_bpm = bpm / Natural::from(base as u32 as Natural);
        self
    }

    pub fn get_curr_bpm(&self) -> NaturalRatio {
        self.curr_bpm
    }

    pub fn curr_tuplet(&mut self, tuplet: u32) -> &mut Self {
        self.curr_tuplet = tuplet;
        self
    }

    pub fn get_curr_tuplet(&self) -> u32 {
        self.curr_tuplet
    }

    pub fn push_note(
        &mut self,
        tone: tone::Note,
        tempo_base: tempo::Base,
        tempo_dot: tempo::Dot,
    ) -> Result<&mut Self, InvalidCompass> {
        let note = Note {
            tone,
            tempo: tempo::Note {
                whole_bpm: self.curr_bpm,
                tuplet: self.curr_tuplet,
                dot: tempo_dot,
                base: tempo_base,
            },
        };
        let sum = self
            .notes
            .iter()
            .map(|note| note.tempo.measure())
            .sum::<NaturalRatio>()
            + note.tempo.measure();

        if sum == self.signature.ratio() {
            self.notes.push(note);
            Ok(self)
        } else {
            Err(InvalidCompass { expected: self.signature.ratio(), found: sum })
        }
    }

    pub fn finish(&self) -> Result<Compass, InvalidCompass> {
        Compass::new(self.signature, self.notes.clone())
    }

    pub fn clear_finish(&mut self) -> Result<Compass, InvalidCompass> {
        Compass::new(self.signature, mem::replace(&mut self.notes, Vec::new()))
    }
}
