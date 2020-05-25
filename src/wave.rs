use rodio::Source;
use std::{f32::consts::PI, time::Duration};

pub trait Wave: Source<Item = f32> {
    fn freq(&self) -> f32;
}

pub trait WaveBuilder {
    type Wave: Wave;

    fn freq(&mut self, freq: f32) -> &mut Self;

    fn get_freq(&self) -> f32;

    fn finish(&mut self) -> Self::Wave;
}

impl<'builder, B> WaveBuilder for &'builder mut B
where
    B: WaveBuilder,
{
    type Wave = B::Wave;

    fn freq(&mut self, freq: f32) -> &mut Self {
        (**self).freq(freq);
        self
    }

    fn get_freq(&self) -> f32 {
        (**self).get_freq()
    }

    fn finish(&mut self) -> Self::Wave {
        (**self).finish()
    }
}

#[derive(Debug, Clone)]
pub struct SineWave {
    freq: f32,
    index: usize,
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.index = self.index.wrapping_add(1);

        let index = self.index as f32;
        let coeficient = PI * 2.0 * self.freq;
        Some((coeficient * index / 48000.0).sin())
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Wave for SineWave {
    fn freq(&self) -> f32 {
        self.freq
    }
}

#[derive(Debug, Clone)]
pub struct SineWaveBuilder {
    freq: f32,
}

impl Default for SineWaveBuilder {
    fn default() -> Self {
        Self { freq: 440.0 }
    }
}

impl WaveBuilder for SineWaveBuilder {
    type Wave = SineWave;

    fn freq(&mut self, freq: f32) -> &mut Self {
        self.freq = freq;
        self
    }

    fn get_freq(&self) -> f32 {
        self.freq
    }

    fn finish(&mut self) -> Self::Wave {
        SineWave { freq: self.freq, index: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct SawWave {
    index: f32,
    freq: f32,
}

impl Iterator for SawWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let period = 48000.0 / self.freq;
        if self.index > period {
            self.index -= period;
        } else {
            self.index += 1.0;
        }

        let value = self.index / period * 2.0 - 1.0;
        Some(value)
    }
}

impl Source for SawWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Wave for SawWave {
    fn freq(&self) -> f32 {
        self.freq
    }
}

#[derive(Debug, Clone)]
pub struct SawWaveBuilder {
    freq: f32,
}

impl Default for SawWaveBuilder {
    fn default() -> Self {
        Self { freq: 440.0 }
    }
}

impl WaveBuilder for SawWaveBuilder {
    type Wave = SawWave;

    fn freq(&mut self, freq: f32) -> &mut Self {
        self.freq = freq;
        self
    }

    fn get_freq(&self) -> f32 {
        self.freq
    }

    fn finish(&mut self) -> Self::Wave {
        SawWave { freq: self.freq, index: 0.0 }
    }
}

pub struct RichWave<W>
where
    W: Wave,
{
    wave: W,
    helpers: Vec<W>,
    dry: f32,
    wet: f32,
}

impl<W> Iterator for RichWave<W>
where
    W: Wave,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut value = self.wave.next()? * self.dry;

        for helper in &mut self.helpers {
            value += helper.next()? * self.wet;
        }

        Some(value)
    }
}

impl<W> Source for RichWave<W>
where
    W: Wave,
{
    fn current_frame_len(&self) -> Option<usize> {
        let mut ret = self.wave.current_frame_len();

        for helper in &self.helpers {
            let frame = helper.current_frame_len();
            if let Some(curr) = ret {
                if let Some(val) = frame.filter(|&val| val < curr) {
                    ret = Some(val);
                }
            } else {
                ret = frame;
            }
        }

        ret
    }

    fn channels(&self) -> u16 {
        self.wave.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.wave.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        let mut ret = self.wave.total_duration();

        for helper in &self.helpers {
            let frame = helper.total_duration();
            if let Some(curr) = ret {
                if let Some(val) = frame.filter(|val| val < &curr) {
                    ret = Some(val);
                }
            } else {
                ret = frame;
            }
        }

        ret
    }
}

impl<W> Wave for RichWave<W>
where
    W: Wave,
{
    fn freq(&self) -> f32 {
        self.wave.freq()
    }
}

#[derive(Debug, Clone)]
pub struct RichWaveBuilder<B>
where
    B: WaveBuilder,
{
    depth: usize,
    dry: f32,
    wet: f32,
    min: f32,
    max: f32,
    inner: B,
}

impl<B> RichWaveBuilder<B>
where
    B: WaveBuilder,
{
    pub fn new(inner: B) -> Self {
        Self { inner, depth: 0, min: 0.0, max: 20000.0, dry: 0.9, wet: 0.01 }
    }

    pub fn depth(&mut self, depth: usize) -> &mut Self {
        self.depth = depth;
        self
    }

    pub fn min(&mut self, min: f32) -> &mut Self {
        self.min = min;
        self
    }

    pub fn max(&mut self, max: f32) -> &mut Self {
        self.max = max;
        self
    }

    pub fn dry(&mut self, dry: f32) -> &mut Self {
        self.dry = dry;
        self
    }

    pub fn wet(&mut self, wet: f32) -> &mut Self {
        self.wet = wet;
        self
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_min(&self) -> f32 {
        self.min
    }

    pub fn get_max(&self) -> f32 {
        self.max
    }

    pub fn get_dry(&self) -> f32 {
        self.dry
    }

    pub fn get_wet(&self) -> f32 {
        self.wet
    }
}

impl<B> WaveBuilder for RichWaveBuilder<B>
where
    B: WaveBuilder,
{
    type Wave = RichWave<B::Wave>;

    fn freq(&mut self, freq: f32) -> &mut Self {
        self.inner.freq(freq);
        self
    }

    fn get_freq(&self) -> f32 {
        self.inner.get_freq()
    }

    fn finish(&mut self) -> Self::Wave {
        let wave = self.inner.finish();
        let mut helpers = Vec::with_capacity(self.depth);

        let leap = (self.max - self.min) / self.depth as f32;
        for i in 0 .. self.depth {
            let freq = self.min + i as f32 * leap;
            helpers.push(self.inner.freq(freq).finish());
        }

        RichWave { wave, helpers, dry: self.dry, wet: self.wet }
    }
}
