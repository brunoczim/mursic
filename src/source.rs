use crate::{
    effects::{LinearFadeOut, LinearFadeOutBuilder, TakeDuration},
    num::Real,
};
use std::time::Duration;

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

    fn take_duration(self, duration: Duration) -> TakeDuration<Self>
    where
        Self: Sized,
    {
        TakeDuration::new(self, duration)
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

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&mut self, channels: u16) -> &mut Self {
        self.channels = channels;
        self
    }

    pub fn get_channels(&self) -> u16 {
        self.channels
    }

    pub fn finish(&self) -> Silence {
        Silence { sample_rate: self.sample_rate, channels: self.channels }
    }
}
