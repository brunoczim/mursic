use crate::{
    compass::{Compass, InvalidCompass, Note},
    effects::TakeDuration,
    manner::{Manner, MannerKind},
    num::{DurationExt, Natural, NaturalRatio, Real},
    pitch::{Key, Pitch},
    source::Source,
    tempo::{Dot, NoteTime, NoteValue, TimeSignature},
    wave::WaveBuilder,
};
use num::Zero;
use std::{mem, time::Duration};

#[derive(Debug, Clone)]
pub struct SongBuilder {
    signature: TimeSignature,
    bpm: NaturalRatio,
    tuplet: u32,
    note_value: NoteValue,
    dot: Dot,
    pitch: Pitch,
    manner_kind: MannerKind,
    notes: Vec<Note>,
    compasses: Vec<Compass>,
}

impl Default for SongBuilder {
    fn default() -> Self {
        Self {
            signature: TimeSignature { numer: 4, denom: NoteValue::Quarter },
            bpm: NaturalRatio::from(120),
            tuplet: 2,
            note_value: NoteValue::Quarter,
            dot: Dot::None,
            pitch: Pitch { key: Key::A, octave: 5 },
            manner_kind: MannerKind::Plain,
            notes: Vec::new(),
            compasses: Vec::new(),
        }
    }
}

impl SongBuilder {
    pub fn signature(&mut self, signature: TimeSignature) -> &mut Self {
        self.signature = signature;
        self
    }

    pub fn get_signature(&self) -> TimeSignature {
        self.signature
    }

    pub fn bpm(
        &mut self,
        note_value: NoteValue,
        bpm: NaturalRatio,
    ) -> &mut Self {
        self.bpm = bpm / Natural::from(note_value as u32 as Natural);
        self
    }

    pub fn get_bpm(&self) -> NaturalRatio {
        self.bpm
    }

    pub fn tuplet(&mut self, tuplet: u32) -> &mut Self {
        self.tuplet = tuplet;
        self
    }

    pub fn get_tuplet(&self) -> u32 {
        self.tuplet
    }

    pub fn note_value(&mut self, note_value: NoteValue) -> &mut Self {
        self.note_value = note_value;
        self
    }

    pub fn get_note_value(&self) -> NoteValue {
        self.note_value
    }

    pub fn dot(&mut self, dot: Dot) -> &mut Self {
        self.dot = dot;
        self
    }

    pub fn get_dot(&self) -> Dot {
        self.dot
    }

    pub fn pitch(&mut self, pitch: Pitch) -> &mut Self {
        self.pitch = pitch;
        self
    }

    pub fn get_pitch(&self) -> Pitch {
        self.pitch
    }

    pub fn manner_kind(&mut self, manner_kind: MannerKind) -> &mut Self {
        self.manner_kind = manner_kind;
        self
    }

    pub fn get_manner_kind(&self) -> MannerKind {
        self.manner_kind
    }

    pub fn note(&mut self) -> &mut Self {
        self.try_note().expect("Error creating note")
    }

    pub fn try_note(&mut self) -> Result<&mut Self, InvalidCompass> {
        let note = Note {
            pitch: self.manner_kind.make_note(self.pitch),
            tempo: NoteTime {
                whole_bpm: self.bpm,
                tuplet: self.tuplet,
                dot: self.dot,
                note_value: self.note_value,
            },
        };
        let sum = self
            .notes
            .iter()
            .map(|note| note.tempo.measure())
            .sum::<NaturalRatio>()
            + note.tempo.measure();

        if sum <= self.signature.ratio() {
            self.notes.push(note);
            Ok(self)
        } else {
            Err(InvalidCompass { expected: self.signature.ratio(), found: sum })
        }
    }

    pub fn compass(&mut self) -> &mut Self {
        self.try_compass().expect("Error creating compass")
    }
    pub fn try_compass(&mut self) -> Result<&mut Self, InvalidCompass> {
        let compass = Compass::new(
            self.signature,
            mem::replace(&mut self.notes, Vec::new()),
        )?;
        self.compasses.push(compass);
        Ok(self)
    }

    pub fn finish(&self) -> Song {
        Song { compasses: self.compasses.clone() }
    }

    pub fn clear_finish(&mut self) -> Song {
        Song { compasses: mem::replace(&mut self.compasses, Vec::new()) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Song {
    pub compasses: Vec<Compass>,
}

impl Song {
    pub fn nanos(&self) -> NaturalRatio {
        self.compasses.iter().map(|comp| comp.nanos()).sum()
    }

    pub fn duration(&self) -> Duration {
        Duration::from_raw_nanos(self.nanos().to_integer())
    }
}

#[derive(Debug, Clone)]
pub struct PlayableSongBuilder {
    a5: Real,
}

impl Default for PlayableSongBuilder {
    fn default() -> Self {
        Self { a5: 440.0 }
    }
}

impl PlayableSongBuilder {
    pub fn a5_freq(&mut self, a5: Real) -> &mut Self {
        self.a5 = a5;
        self
    }

    pub fn get_a5_freq(&self) -> Real {
        self.a5
    }

    pub fn finish<W>(&self, song: Song, instrument: W) -> PlayableSong<W>
    where
        W: WaveBuilder,
    {
        PlayableSong {
            curr_wave: instrument
                .finish()
                .take_duration(Duration::from_secs(0)),
            a5: self.a5,
            instrument,
            remaining: song.nanos(),
            correction: NaturalRatio::zero(),
            song,
            curr_compass: 0,
            curr_note: 0,
            curr_pitch: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayableSong<W>
where
    W: WaveBuilder,
{
    a5: Real,
    instrument: W,
    remaining: NaturalRatio,
    correction: NaturalRatio,
    song: Song,
    curr_compass: usize,
    curr_note: usize,
    curr_pitch: Option<Pitch>,
    curr_wave: TakeDuration<W::Wave>,
}

impl<W> Iterator for PlayableSong<W>
where
    W: WaveBuilder,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ret) = self.curr_wave.next() {
                break Some(ret);
            }

            let note = loop {
                match self
                    .song
                    .compasses
                    .get(self.curr_compass)?
                    .notes
                    .get(self.curr_note)
                {
                    Some(note) => break note,
                    None => {
                        self.curr_note = 0;
                        self.curr_compass += 1;
                    },
                }
            };

            let pitch = match note.pitch {
                Manner::Plain(note) => note,
                Manner::Ligature => {
                    self.curr_pitch.expect("Cannot start song with ligature")
                },
            };
            self.curr_pitch = Some(pitch);

            let mut time = note.tempo.nanos().to_integer();
            self.correction += note.tempo.nanos().fract();
            if self.correction.to_integer() > 0 {
                time += self.correction.to_integer();
                self.correction = self.correction.fract();
            }

            self.curr_wave = self
                .instrument
                .freq(pitch.freq(self.a5))
                .finish()
                .take_duration(Duration::from_raw_nanos(time));

            self.curr_note += 1;
        }
    }
}

impl<W> Source for PlayableSong<W>
where
    W: WaveBuilder + Send + Sync,
{
    fn len(&self) -> Option<usize> {
        let nanos = self.song.nanos().to_integer();
        let rate = self.sample_rate() as Natural;
        Some((nanos / rate) as usize)
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.song.duration())
    }

    fn sample_rate(&self) -> u32 {
        self.instrument.get_sample_rate()
    }

    fn channels(&self) -> u16 {
        self.curr_wave.channels()
    }
}
