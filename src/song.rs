use crate::{
    compass::{Compass, InvalidCompass},
    note::{Note, NoteGroup, NoteKind},
    num::{DurationExt, Natural, NaturalRatio, Real},
    pitch::{Key, Pitch},
    source::{SilenceBuilder, Source, SourceBuilder},
    tempo::{Dot, NoteTime, NoteValue, TimeSignature},
    wave::{Wave, WaveBuilder},
};
use num::Zero;
use std::{collections::BTreeSet, fmt, mem, time::Duration};

#[derive(Debug, Clone)]
pub struct SongBuilder {
    signature: TimeSignature,
    bpm: NaturalRatio,
    tuplet: u32,
    note_value: NoteValue,
    dot: Dot,
    pitch: Pitch,
    note_kind: NoteKind,
    notes: BTreeSet<Note>,
    note_groups: Vec<NoteGroup>,
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
            note_kind: NoteKind::Plain,
            notes: BTreeSet::new(),
            note_groups: Vec::new(),
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

    pub fn note_kind(&mut self, note_kind: NoteKind) -> &mut Self {
        self.note_kind = note_kind;
        self
    }

    pub fn get_note_kind(&self) -> NoteKind {
        self.note_kind
    }

    pub fn note(&mut self) -> &mut Self {
        self.notes.insert(Note { pitch: self.pitch, kind: self.note_kind });
        self
    }

    pub fn note_group(&mut self) -> &mut Self {
        self.try_note_group().expect("Error creating note")
    }

    pub fn try_note_group(&mut self) -> Result<&mut Self, InvalidCompass> {
        let group = NoteGroup {
            notes: mem::replace(&mut self.notes, BTreeSet::new()),
            tempo: NoteTime {
                whole_bpm: self.bpm,
                tuplet: self.tuplet,
                dot: self.dot,
                note_value: self.note_value,
            },
        };
        let sum = self
            .note_groups
            .iter()
            .map(|group| group.tempo.measure())
            .sum::<NaturalRatio>()
            + group.tempo.measure();

        if sum <= self.signature.ratio() {
            self.note_groups.push(group);
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
            mem::replace(&mut self.note_groups, Vec::new()),
        )?;
        self.compasses.push(compass);
        Ok(self)
    }

    pub fn finish(&self) -> Song {
        if self.notes.len() > 0 {
            panic!("Notes left unincluded in any compass");
        }
        Song { compasses: self.compasses.clone() }
    }

    pub fn clear_finish(&mut self) -> Song {
        if self.notes.len() > 0 {
            panic!("Notes left unincluded in any compass");
        }
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
    start_compass: usize,
}

impl Default for PlayableSongBuilder {
    fn default() -> Self {
        Self { a5: 440.0, start_compass: 0 }
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

    pub fn start_compass(&mut self, start_compass: usize) -> &mut Self {
        self.start_compass = start_compass;
        self
    }

    pub fn get_start_compass(&self) -> usize {
        self.start_compass
    }

    pub fn finish<W>(&self, song: Song, instrument: W) -> PlayableSong<W>
    where
        W: WaveBuilder + Send + Sync,
        W::Source: Wave + 'static,
    {
        PlayableSong {
            curr_wave: Box::new(
                instrument.finish().take_duration(Duration::from_secs(0)),
            ),
            a5: self.a5,
            instrument,
            remaining: song.nanos(),
            correction: NaturalRatio::zero(),
            song,
            curr_compass: self.start_compass,
            curr_note: 0,
            curr_pitch: None,
        }
    }
}

pub struct PlayableSong<W>
where
    W: WaveBuilder + Send + Sync,
    W::Source: Wave + 'static,
{
    a5: Real,
    instrument: W,
    remaining: NaturalRatio,
    correction: NaturalRatio,
    song: Song,
    curr_compass: usize,
    curr_note: usize,
    curr_pitch: Option<Pitch>,
    curr_wave: Box<dyn Source>,
}

impl<W> PlayableSong<W>
where
    W: WaveBuilder + Send + Sync,
    W::Source: Wave + 'static,
{
}

impl<W> fmt::Debug for PlayableSong<W>
where
    W: WaveBuilder + Send + Sync + fmt::Debug,
    W::Source: Wave + 'static,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PlayableSong")
            .field("a5", &self.a5)
            .field("instrument", &self.instrument)
            .field("remaining", &self.remaining)
            .field("correction", &self.correction)
            .field("song", &self.song)
            .field("curr_compass", &self.curr_compass)
            .field("curr_note", &self.curr_note)
            .field("curr_pitch", &self.curr_pitch)
            .finish()
    }
}

impl<W> Iterator for PlayableSong<W>
where
    W: WaveBuilder + Send + Sync,
    W::Source: Wave + 'static,
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
                    .note_groups
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
                Manner::Plain(note) => Some(note),
                Manner::Ligature => Some(
                    self.curr_pitch.expect("Cannot start song with ligature"),
                ),
                Manner::Rest => None,
            };
            self.curr_pitch = pitch;

            let mut time = note.tempo.nanos().to_integer();
            self.correction += note.tempo.nanos().fract();
            if self.correction.to_integer() > 0 {
                time += self.correction.to_integer();
                self.correction = self.correction.fract();
            }

            let duration = Duration::from_raw_nanos(time);

            self.curr_wave = match pitch {
                Some(pitch) => Box::new(
                    self.instrument
                        .freq(pitch.freq(self.a5))
                        .finish()
                        .take_duration(duration),
                ),

                None => Box::new(
                    SilenceBuilder::default()
                        .sample_rate(self.sample_rate())
                        .channels(self.channels())
                        .finish()
                        .take_duration(duration),
                ),
            };

            self.curr_note += 1;
        }
    }
}

impl<W> Source for PlayableSong<W>
where
    W: WaveBuilder + Send + Sync,
    W::Source: Wave + 'static,
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
        self.instrument.get_channels()
    }
}
