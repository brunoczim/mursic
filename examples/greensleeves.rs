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
    let mut builder = SongBuilder::default();
    builder.bpm(NoteValue::Quarter, NaturalRatio::from(120));
    make_verse_first_part(octave, &mut builder);
    builder
        .pitch(Pitch { octave: octave + 1, key: Key::C })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave, key: Key::A })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::Ab })
        .dot(Dot::None)
        .note_value(NoteValue::Eighth)
        .note()
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave, key: Key::Ab })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave, key: Key::E })
        .note_value(NoteValue::Half)
        .dot(Dot::Single)
        .note()
        .compass();

    make_verse_first_part(octave, &mut builder);
    builder
        .pitch(Pitch { octave: octave + 1, key: Key::C })
        .note_value(NoteValue::Quarter)
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Eighth)
        .dot(Dot::None)
        .note()
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave, key: Key::Ab })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::Gb })
        .dot(Dot::None)
        .note_value(NoteValue::Eighth)
        .note()
        .pitch(Pitch { octave, key: Key::Ab })
        .note_value(NoteValue::Quarter)
        .note()
        .compass()
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Half)
        .dot(Dot::Single)
        .note()
        .compass();

    let song = builder.clear_finish();

    let playable = PlayableSongBuilder::default()
        .finish(song, SquareWaveBuilder::default());
    let player = Player::new().unwrap();
    player.play(playable);
    player.wait();
}

fn make_verse_first_part(octave: u32, builder: &mut SongBuilder) {
    builder
        .signature(TimeSignature { numer: 1, denom: NoteValue::Quarter })
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .dot(Dot::None)
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
        .pitch(Pitch { octave, key: Key::G })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::A })
        .dot(Dot::None)
        .note_value(NoteValue::Eighth)
        .note()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Quarter)
        .note()
        .compass();
}
