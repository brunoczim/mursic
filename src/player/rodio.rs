use super::Stream;
use std::time::Duration;

#[derive(Debug)]
pub struct Rodio {
    sink: rodio::Sink,
}

#[derive(Debug)]
struct Bridge<S>
where
    S: Stream,
{
    inner: S,
}

impl<S> Iterator for Bridge<S>
where
    S: Stream,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|elem| elem as f32)
    }
}

impl<S> rodio::Source for Bridge<S>
where
    S: Stream,
{
    fn total_duration(&self) -> Option<Duration> {
        self.inner.duration()
    }

    fn current_frame_len(&self) -> Option<usize> {
        self.inner.frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.rate()
    }
}
struct Bridge<S>
where
    S: Stream,
{
    inner: S,
}

impl<S> Iterator for Bridge<S>
where
    S: Stream,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|elem| elem as f32)
    }
}

impl<S> rodio::Source for Bridge<S>
where
    S: Stream,
{
    fn total_duration(&self) -> Option<Duration> {
        self.inner.duration()
    }

    fn current_frame_len(&self) -> Option<usize> {
        self.inner.frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.rate()
    }
}
