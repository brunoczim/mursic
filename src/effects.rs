use crate::{num::Real, source::Source};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LinearFadeOut<S>
where
    S: Source,
{
    inner: S,
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
        self.channel -= 1;
        if self.channel == 0 {
            self.channel = self.channels();
            if self.curr_vol > self.final_vol {
                self.curr_vol -= self.step;
            }
        }
        Some(value)
    }
}

impl<S> Source for LinearFadeOut<S>
where
    S: Source,
{
    fn total_len(&self) -> Option<usize> {
        self.inner.total_len()
    }

    fn current_frame_len(&self) -> Option<usize> {
        self.inner.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
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
        let len = source.total_len().unwrap_or(self.iterations);
        LinearFadeOut {
            channel: source.channels(),
            inner: source,
            curr_vol: 1.0,
            final_vol: self.final_vol,
            step: (1.0 - self.final_vol) / len as Real,
        }
    }
}
