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
