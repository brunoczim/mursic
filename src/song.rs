use crate::{
    compass::{Compass, CompassBuilder, InvalidCompass},
    effects::TakeDuration,
    num::{DurationExt, Natural, NaturalRatio, Real},
    source::Source,
    tempo,
    tone,
    wave::WaveBuilder,
};
use num::Zero;
use std::{mem, time::Duration};

#[derive(Debug, Clone)]
pub struct SongBuilder {
    song: Song,
    curr_bpm: NaturalRatio,
    curr_tuplet: u32,
    curr_sig: tempo::Signature,
}

impl Default for SongBuilder {
    fn default() -> Self {
        Self {
            song: Song { compasses: Vec::new() },
            curr_bpm: NaturalRatio::from(120),
            curr_tuplet: 2,
            curr_sig: tempo::Signature {
                numer: 4,
                denom: tempo::Base::Quarter,
            },
        }
    }
}

impl SongBuilder {
    pub fn with_compass<F>(
        &mut self,
        func: F,
    ) -> Result<&mut Self, InvalidCompass>
    where
        F: FnOnce(&mut CompassBuilder) -> Result<(), InvalidCompass>,
    {
        let mut builder = CompassBuilder::default();
        func(
            builder
                .curr_bpm(tempo::Base::Whole, self.curr_bpm)
                .curr_tuplet(self.curr_tuplet)
                .signature(self.curr_sig),
        )?;

        self.song.compasses.push(builder.clear_finish()?);

        self.curr_bpm = builder.get_curr_bpm();
        self.curr_tuplet = builder.get_curr_tuplet();
        self.curr_sig = builder.get_signature();

        Ok(self)
    }

    pub fn finish(&self) -> Song {
        self.song.clone()
    }

    pub fn clear_finish(&mut self) -> Song {
        mem::replace(&mut self.song, Song { compasses: Vec::new() })
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
            curr_hard_note: None,
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
    curr_hard_note: Option<tone::HardNote>,
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

            let tone_hard_note = match note.tone {
                tone::Note::Hard(note) => note,
                tone::Note::Ligature => self
                    .curr_hard_note
                    .expect("Cannot start song with ligature"),
            };
            self.curr_hard_note = Some(tone_hard_note);

            let mut time = note.tempo.nanos().to_integer();
            self.correction += note.tempo.nanos().fract();
            if self.correction.to_integer() > 0 {
                time += self.correction.to_integer();
                self.correction = self.correction.fract();
            }

            self.curr_wave = self
                .instrument
                .freq(tone_hard_note.freq(self.a5))
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
