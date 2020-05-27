use mursic::{
    player::Player,
    song::SongBuilder,
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
    let song = SongBuilder::default().clear_finish();
    let player = Player::new().unwrap();
    unimplemented!()
}
