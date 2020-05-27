use super::Backend;
use crate::source::Source;
use std::{fmt, time::Duration};

pub struct Rodio {
    sink: rodio::Sink,
}

impl fmt::Debug for Rodio {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad("Rodio Backend")
    }
}

impl Backend for Rodio {
    fn new() -> Option<Self> {
        rodio::default_output_device()
            .map(|device| Self { sink: rodio::Sink::new(&device) })
    }

    fn play<S>(&self, source: S)
    where
        S: Source + 'static,
    {
        self.sink.append(Bridge { inner: source });
        self.sink.play();
    }

    fn wait(&self) {
        self.sink.sleep_until_end();
    }
}

#[derive(Debug)]
struct Bridge<S>
where
    S: Source,
{
    inner: S,
}

impl<S> Iterator for Bridge<S>
where
    S: Source,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|elem| elem as f32)
    }
}

impl<S> rodio::Source for Bridge<S>
where
    S: Source,
{
    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
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
}
