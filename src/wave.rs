use crate::{
    num::{real::consts::PI, Real},
    source::{Source, SourceBuilder},
};
use std::time::Duration;

pub trait Wave: Source {
    fn freq(&self) -> Real;
}

pub trait WaveBuilder
where
    Self: SourceBuilder,
    Self::Source: Wave,
{
    fn freq(&mut self, freq: Real) -> &mut Self;

    fn get_freq(&self) -> Real;

    fn sample_rate(&mut self, rate: u32) -> &mut Self;
}

impl<'builder, B> WaveBuilder for &'builder mut B
where
    B: WaveBuilder,
    B::Source: Wave,
{
    fn freq(&mut self, freq: Real) -> &mut Self {
        (**self).freq(freq);
        self
    }

    fn get_freq(&self) -> Real {
        (**self).get_freq()
    }

    fn sample_rate(&mut self, rate: u32) -> &mut Self {
        (**self).sample_rate(rate);
        self
    }
}

impl<B> WaveBuilder for Box<B>
where
    B: WaveBuilder + ?Sized,
    B::Source: Wave,
{
    fn freq(&mut self, freq: Real) -> &mut Self {
        (**self).freq(freq);
        self
    }

    fn get_freq(&self) -> Real {
        (**self).get_freq()
    }

    fn sample_rate(&mut self, rate: u32) -> &mut Self {
        (**self).sample_rate(rate);
        self
    }
}

#[derive(Debug, Clone)]
pub struct SineWave {
    freq: Real,
    index: usize,
}

impl Iterator for SineWave {
    type Item = Real;

    fn next(&mut self) -> Option<Real> {
        self.index = self.index.wrapping_add(1);

        let index = self.index as Real;
        let coeficient = PI * 2.0 * self.freq;
        Some((coeficient * index / 48000.0).sin())
    }
}

impl Source for SineWave {
    fn duration(&self) -> Option<Duration> {
        None
    }

    fn len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }
}

impl Wave for SineWave {
    fn freq(&self) -> Real {
        self.freq
    }
}

#[derive(Debug, Clone)]
pub struct SineWaveBuilder {
    freq: Real,
    sample_rate: u32,
}

impl Default for SineWaveBuilder {
    fn default() -> Self {
        Self { freq: 440.0, sample_rate: 48000 }
    }
}

impl SourceBuilder for SineWaveBuilder {
    type Source = SineWave;

    fn get_channels(&self) -> u16 {
        1
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn finish(&self) -> Self::Source {
        SineWave { freq: self.freq, index: 0 }
    }
}

impl WaveBuilder for SineWaveBuilder {
    fn freq(&mut self, freq: Real) -> &mut Self {
        self.freq = freq;
        self
    }

    fn get_freq(&self) -> Real {
        self.freq
    }

    fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SawWave {
    index: Real,
    freq: Real,
}

impl Iterator for SawWave {
    type Item = Real;

    fn next(&mut self) -> Option<Real> {
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
    fn len(&self) -> Option<usize> {
        None
    }

    fn duration(&self) -> Option<Duration> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }
}

impl Wave for SawWave {
    fn freq(&self) -> Real {
        self.freq
    }
}

#[derive(Debug, Clone)]
pub struct SawWaveBuilder {
    freq: Real,
    sample_rate: u32,
}

impl Default for SawWaveBuilder {
    fn default() -> Self {
        Self { freq: 440.0, sample_rate: 48000 }
    }
}

impl SourceBuilder for SawWaveBuilder {
    type Source = SawWave;

    fn get_channels(&self) -> u16 {
        1
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn finish(&self) -> Self::Source {
        SawWave { freq: self.freq, index: 0.0 }
    }
}

impl WaveBuilder for SawWaveBuilder {
    fn freq(&mut self, freq: Real) -> &mut Self {
        self.freq = freq;
        self
    }

    fn get_freq(&self) -> Real {
        self.freq
    }

    fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SquareWave {
    index: Real,
    freq: Real,
}

impl Iterator for SquareWave {
    type Item = Real;

    fn next(&mut self) -> Option<Real> {
        let period = 48000.0 / self.freq;
        if self.index > period {
            self.index -= period;
        } else {
            self.index += 1.0;
        }

        let ratio = self.index / period;
        let value = if ratio < 0.5 { 1.0 } else { -1.0 };
        Some(value)
    }
}

impl Source for SquareWave {
    fn len(&self) -> Option<usize> {
        None
    }

    fn duration(&self) -> Option<Duration> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }
}

impl Wave for SquareWave {
    fn freq(&self) -> Real {
        self.freq
    }
}

#[derive(Debug, Clone)]
pub struct SquareWaveBuilder {
    freq: Real,
    sample_rate: u32,
}

impl Default for SquareWaveBuilder {
    fn default() -> Self {
        Self { freq: 440.0, sample_rate: 48000 }
    }
}

impl SourceBuilder for SquareWaveBuilder {
    type Source = SquareWave;

    fn get_channels(&self) -> u16 {
        1
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn finish(&self) -> Self::Source {
        SquareWave { freq: self.freq, index: 0.0 }
    }
}

impl WaveBuilder for SquareWaveBuilder {
    fn freq(&mut self, freq: Real) -> &mut Self {
        self.freq = freq;
        self
    }

    fn get_freq(&self) -> Real {
        self.freq
    }

    fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TriangleWave {
    index: Real,
    freq: Real,
}

impl Iterator for TriangleWave {
    type Item = Real;

    fn next(&mut self) -> Option<Real> {
        let period = 48000.0 / self.freq;
        if self.index > period {
            self.index -= period;
        } else {
            self.index += 1.0;
        }

        let ratio = self.index / period * 4.0;
        let value = if ratio < 1.0 {
            ratio
        } else if ratio < 3.0 {
            2.0 - ratio
        } else {
            ratio - 4.0
        };
        Some(value)
    }
}

impl Source for TriangleWave {
    fn len(&self) -> Option<usize> {
        None
    }

    fn duration(&self) -> Option<Duration> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }
}

impl Wave for TriangleWave {
    fn freq(&self) -> Real {
        self.freq
    }
}

#[derive(Debug, Clone)]
pub struct TriangleWaveBuilder {
    freq: Real,
    sample_rate: u32,
}

impl Default for TriangleWaveBuilder {
    fn default() -> Self {
        Self { freq: 440.0, sample_rate: 48000 }
    }
}

impl SourceBuilder for TriangleWaveBuilder {
    type Source = TriangleWave;

    fn get_channels(&self) -> u16 {
        1
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn finish(&self) -> Self::Source {
        TriangleWave { freq: self.freq, index: 0.0 }
    }
}

impl WaveBuilder for TriangleWaveBuilder {
    fn freq(&mut self, freq: Real) -> &mut Self {
        self.freq = freq;
        self
    }

    fn get_freq(&self) -> Real {
        self.freq
    }

    fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }
}

pub struct RichWave<W>
where
    W: Wave,
{
    wave: W,
    helpers: Vec<W>,
    dry: Real,
    wet: Real,
}

impl<W> Iterator for RichWave<W>
where
    W: Wave,
{
    type Item = Real;

    fn next(&mut self) -> Option<Real> {
        let mut value = self.wave.next()? * self.dry;

        for helper in &mut self.helpers {
            value += helper.next()? * self.wet;
        }

        Some(value)
    }
}

fn option_min<I, T>(init: Option<T>, iterable: I) -> Option<T>
where
    I: IntoIterator<Item = Option<T>>,
    T: Ord,
{
    let mut ret = init;

    for elem in iterable {
        if let Some(curr) = ret {
            if let Some(val) = elem.filter(|val| val < &curr) {
                ret = Some(val);
            } else {
                ret = Some(curr);
            }
        } else {
            ret = elem;
        }
    }

    ret
}

impl<W> Source for RichWave<W>
where
    W: Wave,
{
    fn len(&self) -> Option<usize> {
        option_min(
            self.wave.len(),
            self.helpers.iter().map(|helper| helper.len()),
        )
    }

    fn duration(&self) -> Option<Duration> {
        option_min(
            self.wave.duration(),
            self.helpers.iter().map(|helper| helper.duration()),
        )
    }

    fn channels(&self) -> u16 {
        self.wave.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.wave.sample_rate()
    }
}

impl<W> Wave for RichWave<W>
where
    W: Wave,
{
    fn freq(&self) -> Real {
        self.wave.freq()
    }
}

#[derive(Debug, Clone)]
pub struct RichWaveBuilder<B>
where
    B: WaveBuilder + Clone,
    B::Source: Wave,
{
    depth: usize,
    dry: Real,
    wet: Real,
    min: Real,
    max: Real,
    inner: B,
}

impl<B> RichWaveBuilder<B>
where
    B: WaveBuilder + Clone,
    B::Source: Wave,
{
    pub fn new(inner: B) -> Self {
        Self { inner, depth: 0, min: 0.0, max: 20000.0, dry: 0.9, wet: 0.01 }
    }

    pub fn depth(&mut self, depth: usize) -> &mut Self {
        self.depth = depth;
        self
    }

    pub fn min(&mut self, min: Real) -> &mut Self {
        self.min = min;
        self
    }

    pub fn max(&mut self, max: Real) -> &mut Self {
        self.max = max;
        self
    }

    pub fn dry(&mut self, dry: Real) -> &mut Self {
        self.dry = dry;
        self
    }

    pub fn wet(&mut self, wet: Real) -> &mut Self {
        self.wet = wet;
        self
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_min(&self) -> Real {
        self.min
    }

    pub fn get_max(&self) -> Real {
        self.max
    }

    pub fn get_dry(&self) -> Real {
        self.dry
    }

    pub fn get_wet(&self) -> Real {
        self.wet
    }
}

impl<B> SourceBuilder for RichWaveBuilder<B>
where
    B: WaveBuilder + Clone,
    B::Source: Wave,
{
    type Source = RichWave<B::Source>;

    fn get_channels(&self) -> u16 {
        self.inner.get_channels()
    }

    fn get_sample_rate(&self) -> u32 {
        self.inner.get_sample_rate()
    }

    fn finish(&self) -> Self::Source {
        let wave = self.inner.finish();
        let mut helpers = Vec::with_capacity(self.depth);

        let leap = (self.max - self.min) / self.depth as Real;
        for i in 0 .. self.depth {
            let freq = self.min + i as Real * leap;
            helpers.push(self.inner.clone().freq(freq).finish());
        }

        RichWave { wave, helpers, dry: self.dry, wet: self.wet }
    }
}

impl<B> WaveBuilder for RichWaveBuilder<B>
where
    B: WaveBuilder + Clone,
    B::Source: Wave,
{
    fn freq(&mut self, freq: Real) -> &mut Self {
        self.inner.freq(freq);
        self
    }

    fn get_freq(&self) -> Real {
        self.inner.get_freq()
    }

    fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.inner.sample_rate(sample_rate);
        self
    }
}
