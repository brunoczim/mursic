use crate::{
    num::{DurationExt, Natural, NaturalRatio, Real},
    source::Source,
};
use num::{traits::CheckedSub, Zero};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LinearFadeOut<S>
where
    S: Source,
{
    inner: S,
    channels: u16,
    channel: u16,
    step: Real,
    curr_vol: Real,
    final_vol: Real,
}

impl<S> Iterator for LinearFadeOut<S>
where
    S: Source,
{
    type Item = Real;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next()? * self.curr_vol;
        self.channel = self.channel.saturating_sub(1);
        if self.channel == 0 {
            self.channel = self.channels();
            if self.curr_vol > self.final_vol {
                self.curr_vol = (self.curr_vol - self.step).max(0.0);
            }
        }
        Some(value)
    }
}

impl<S> Source for LinearFadeOut<S>
where
    S: Source,
{
    fn len(&self) -> Option<usize> {
        self.inner.len()
    }

    fn duration(&self) -> Option<Duration> {
        self.inner.duration()
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }
}

pub struct LinearFadeOutBuilder {
    iterations: usize,
    final_vol: Real,
}

impl Default for LinearFadeOutBuilder {
    fn default() -> Self {
        Self { iterations: 0, final_vol: 0.0 }
    }
}

impl LinearFadeOutBuilder {
    pub fn iterations(&mut self, iterations: usize) -> &mut Self {
        self.iterations = iterations;
        self
    }

    pub fn final_vol(&mut self, final_vol: Real) -> &mut Self {
        self.final_vol = final_vol;
        self
    }

    pub fn get_iterations(&self) -> usize {
        self.iterations
    }

    pub fn get_final_vol(&self) -> Real {
        self.final_vol
    }

    pub fn finish<S>(&self, source: S) -> LinearFadeOut<S>
    where
        S: Source,
    {
        let len = source.len().unwrap_or(self.iterations);
        let channels = source.channels();
        LinearFadeOut {
            channels,
            channel: channels,
            inner: source,
            curr_vol: 1.0,
            final_vol: self.final_vol,
            step: (1.0 - self.final_vol) / len as Real,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TakeDuration<S>
where
    S: Source,
{
    inner: S,
    channels: u16,
    channel: u16,
    remaining: NaturalRatio,
}

impl<S> TakeDuration<S>
where
    S: Source,
{
    pub(crate) fn new(inner: S, duration: Duration) -> Self {
        let channels = inner.channels();
        Self {
            channels,
            channel: channels,
            inner,
            remaining: NaturalRatio::from(duration.as_nanos()),
        }
    }

    fn max_duration(&self) -> Duration {
        Duration::from_raw_nanos(self.remaining.to_integer())
    }

    fn max_len(&self) -> usize {
        let one_sec = Duration::from_secs(1).as_nanos();
        let rate = self.sample_rate() as Natural;
        let nanos_per_sample = NaturalRatio::new(rate, one_sec);
        let nanos = NaturalRatio::from(self.max_duration().as_nanos());
        let total_ratio = nanos_per_sample * nanos;

        total_ratio.to_integer() as usize
    }
}

impl<S> Iterator for TakeDuration<S>
where
    S: Source,
{
    type Item = Real;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_zero() {
            return None;
        }

        self.channel = self.channel.saturating_sub(1);
        if self.channel == 0 {
            self.channel = self.channels();
            let one_sec = Duration::from_secs(1).as_nanos();
            let rate = self.sample_rate() as Natural;
            self.remaining = self
                .remaining
                .checked_sub(&NaturalRatio::new(one_sec, rate))
                .unwrap_or(Zero::zero());
            if self.remaining.is_zero() {
                return None;
            }
        }

        self.inner.next()
    }
}

impl<S> Source for TakeDuration<S>
where
    S: Source,
{
    fn len(&self) -> Option<usize> {
        let ret = match self.inner.len() {
            Some(len) => len.min(self.max_len()),
            None => self.max_len(),
        };
        Some(ret)
    }

    fn duration(&self) -> Option<Duration> {
        let ret = match self.inner.duration() {
            Some(duration) => duration.min(self.max_duration()),
            None => self.max_duration(),
        };
        Some(ret)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }
}
