use crate::num::Real;
use std::time::Duration;

pub trait Stream: Iterator<Item = Real> {
    fn frame_len(&self) -> Option<usize>;

    fn duration(&self) -> Option<Duration>;

    fn channels(&self) -> u16 {
        1
    }

    fn rate(&self) -> u32 {
        48000
    }
}

impl<'this, S> Stream for &'this mut S
where
    S: Stream,
{
    fn frame_len(&self) -> Option<usize> {
        (**self).frame_len()
    }

    fn duration(&self) -> Option<Duration> {
        (**self).duration()
    }

    fn channels(&self) -> u16 {
        (**self).channels()
    }

    fn rate(&self) -> u32 {
        (**self).rate()
    }
}
