use mursic::{
    effects::LinearFadeOutBuilder,
    player::Player,
    source::Source,
    wave::{SawWaveBuilder, WaveBuilder},
};
use std::time::Duration;

fn main() {
    let duration = Duration::from_millis(2000);
    let wave = LinearFadeOutBuilder::default().finish(
        SawWaveBuilder::default().freq(440.0).finish().take_duration(duration),
    );
    let player = Player::new().unwrap();
    player.play(wave);
    player.wait();
}
