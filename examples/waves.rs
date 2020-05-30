use mursic::{
    effects::LinearFadeOutBuilder,
    player::Player,
    source::{Source, SourceBuilder},
    wave::{
        SawWaveBuilder,
        SineWaveBuilder,
        SquareWaveBuilder,
        TriangleWaveBuilder,
        WaveBuilder,
    },
};
use std::time::Duration;

fn main() {
    let player = Player::new().unwrap();
    let duration = Duration::from_millis(1000);
    let wave = LinearFadeOutBuilder::default().finish(
        SineWaveBuilder::default().freq(440.0).finish().take_duration(duration),
    );
    player.play(wave);
    player.wait();
    let wave = LinearFadeOutBuilder::default().finish(
        SawWaveBuilder::default().freq(440.0).finish().take_duration(duration),
    );
    player.play(wave);
    player.wait();
    let wave = LinearFadeOutBuilder::default().finish(
        SquareWaveBuilder::default()
            .freq(440.0)
            .finish()
            .take_duration(duration),
    );
    player.play(wave);
    player.wait();
    let wave = LinearFadeOutBuilder::default().finish(
        TriangleWaveBuilder::default()
            .freq(440.0)
            .finish()
            .take_duration(duration),
    );
    player.play(wave);
    player.wait();
}
