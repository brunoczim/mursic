use crate::{
    effects::{LinearFadeOut, LinearFadeOutBuilder},
    num::Real,
};
use std::time::Duration;

pub trait Source: Iterator<Item = Real> + Send + Sync {
    fn total_len(&self) -> Option<usize>;

    fn current_frame_len(&self) -> Option<usize>;

    fn total_duration(&self) -> Option<Duration>;

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
}

impl<'this, S> Source for &'this mut S
where
    S: Source,
{
    fn total_len(&self) -> Option<usize> {
        (**self).total_len()
    }

    fn current_frame_len(&self) -> Option<usize> {
        (**self).current_frame_len()
    }

    fn total_duration(&self) -> Option<Duration> {
        (**self).total_duration()
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
    fn total_len(&self) -> Option<usize> {
        (**self).total_len()
    }

    fn current_frame_len(&self) -> Option<usize> {
        (**self).current_frame_len()
    }

    fn total_duration(&self) -> Option<Duration> {
        (**self).total_duration()
    }

    fn channels(&self) -> u16 {
        (**self).channels()
    }

    fn sample_rate(&self) -> u32 {
        (**self).sample_rate()
    }
}
