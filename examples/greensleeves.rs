use mursic::{
    num::NaturalRatio,
    pitch::{Key, Pitch},
    player::Player,
    song::{PlayableSongBuilder, SongBuilder},
    tempo::{Dot, NoteValue, TimeSignature},
    wave::SquareWaveBuilder,
};

fn main() {
    let octave = 4;
    let song = SongBuilder::default()
        .bpm(NoteValue::Quarter, NaturalRatio::from(120))
        .signature(TimeSignature { numer: 1, denom: NoteValue::Quarter })
        .pitch(Pitch { octave, key: Key::A })
        .note()
        .compass()
        .signature(TimeSignature { numer: 3, denom: NoteValue::Quarter })
        .pitch(Pitch { octave: octave + 1, key: Key::C })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: octave + 1, key: Key::D })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave: octave + 1, key: Key::E })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave: octave + 1, key: Key::F })
        .note_value(NoteValue::Eighth)
        .dot(Dot::None)
        .note()
        .pitch(Pitch { octave: octave + 1, key: Key::E })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave: octave + 1, key: Key::D })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .clear_finish();
    let playable = PlayableSongBuilder::default()
        .finish(song, SquareWaveBuilder::default());
    let player = Player::new().unwrap();
    player.play(playable);
    player.wait();
}
