use mursic::{
    num::NaturalRatio,
    pitch::{Key, Pitch},
    player::Player,
    song::{PlayableSongBuilder, SongBuilder},
    tempo::{NoteValue, TimeSignature},
    wave::SquareWaveBuilder,
};

fn main() {
    let song = SongBuilder::default()
        .bpm(NoteValue::Quarter, NaturalRatio::from(150))
        .signature(TimeSignature { numer: 1, denom: NoteValue::Quarter })
        .pitch(Pitch { octave: 3, key: Key::A })
        .note()
        .compass()
        .signature(TimeSignature { numer: 3, denom: NoteValue::Quarter })
        .pitch(Pitch { octave: 4, key: Key::C })
        .tempo_note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: 4, key: Key::D })
        .tempo_note_value(NoteValue::Quarter)
        .note()
        .compass()
        .clear_finish();
    let playable = PlayableSongBuilder::default()
        .finish(song, SquareWaveBuilder::default());
    let player = Player::new().unwrap();
    player.play(playable);
    player.wait();
}
