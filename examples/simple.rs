use mursic::{
    effects::LinearFadeOutBuilder,
    rodio,
    wave::{SawWaveBuilder, WaveBuilder},
};
use rodio::{Sink, Source};
use std::time::Duration;

fn main() {
    let dev = rodio::default_output_device().unwrap();
    let sink = Sink::new(&dev);
    let duration = Duration::from_millis(2000);
    sink.append(LinearFadeOutBuilder::default().finish(
        SawWaveBuilder::default().freq(440.0).finish().take_duration(duration),
    ));
    sink.play();
    sink.sleep_until_end();
}
