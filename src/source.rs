use crate::{
    effects::{LinearFadeOut, LinearFadeOutBuilder, Take},
    num::{Natural, NaturalRatio, Real},
};
use std::{
    io::{Seek, Write},
    time::Duration,
};

pub trait Source: Iterator<Item = Real> + Send + Sync {
    fn len(&self) -> Option<usize>;

    fn duration(&self) -> Option<Duration>;

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn fade_out(self) -> LinearFadeOut<Self>
    where
        Self: Sized,
    {
        LinearFadeOutBuilder::default().finish(self)
    }

    fn take_samples(self, samples: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, samples)
    }

    fn take_duration(self, duration: Duration) -> Take<Self>
    where
        Self: Sized,
    {
        let sample_time = NaturalRatio::new(
            Duration::from_secs(1).as_nanos(),
            self.sample_rate() as Natural,
        );
        let nanos = NaturalRatio::from(duration.as_nanos());
        self.take_samples((nanos / sample_time).round().to_integer() as usize)
    }

    fn to_wav<W>(self, target: W) -> Result<(), hound::Error>
    where
        Self: Sized,
        W: Write + Seek,
    {
        let mut writer = hound::WavWriter::new(
            target,
            hound::WavSpec {
                channels: self.channels(),
                sample_rate: self.sample_rate(),
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            },
        )?;

        for sample in self {
            writer.write_sample(sample as f32)?;
        }

        writer.flush()?;
        Ok(())
    }
}

impl<'this, S> Source for &'this mut S
where
    S: Source,
{
    fn len(&self) -> Option<usize> {
        (**self).len()
    }

    fn duration(&self) -> Option<Duration> {
        (**self).duration()
    }

    fn channels(&self) -> u16 {
        (**self).channels()
    }

    fn sample_rate(&self) -> u32 {
        (**self).sample_rate()
    }
}

impl<S> Source for Box<S>
where
    S: Source + ?Sized,
{
    fn len(&self) -> Option<usize> {
        (**self).len()
    }

    fn duration(&self) -> Option<Duration> {
        (**self).duration()
    }

    fn channels(&self) -> u16 {
        (**self).channels()
    }

    fn sample_rate(&self) -> u32 {
        (**self).sample_rate()
    }
}

pub trait SourceBuilder {
    type Source: Source;

    fn get_channels(&self) -> u16;

    fn get_sample_rate(&self) -> u32;

    fn finish(&self) -> Self::Source;
}

impl<'builder, B> SourceBuilder for &'builder mut B
where
    B: SourceBuilder,
{
    type Source = B::Source;

    fn get_channels(&self) -> u16 {
        (**self).get_channels()
    }

    fn get_sample_rate(&self) -> u32 {
        (**self).get_sample_rate()
    }

    fn finish(&self) -> Self::Source {
        (**self).finish()
    }
}

impl<B> SourceBuilder for Box<B>
where
    B: SourceBuilder + ?Sized,
{
    type Source = B::Source;

    fn get_channels(&self) -> u16 {
        (**self).get_channels()
    }

    fn get_sample_rate(&self) -> u32 {
        (**self).get_sample_rate()
    }

    fn finish(&self) -> Self::Source {
        (**self).finish()
    }
}

#[derive(Debug, Clone)]
pub struct Silence {
    sample_rate: u32,
    channels: u16,
}

impl Iterator for Silence {
    type Item = Real;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0.0)
    }
}

impl Source for Silence {
    fn len(&self) -> Option<usize> {
        None
    }

    fn duration(&self) -> Option<Duration> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[derive(Debug, Clone)]
pub struct SilenceBuilder {
    sample_rate: u32,
    channels: u16,
}

impl Default for SilenceBuilder {
    fn default() -> Self {
        Self { sample_rate: 48000, channels: 1 }
    }
}

impl SilenceBuilder {
    pub fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn channels(&mut self, channels: u16) -> &mut Self {
        self.channels = channels;
        self
    }
}

impl SourceBuilder for SilenceBuilder {
    type Source = Silence;

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn get_channels(&self) -> u16 {
        self.channels
    }

    fn finish(&self) -> Self::Source {
        Silence { sample_rate: self.sample_rate, channels: self.channels }
    }
}
