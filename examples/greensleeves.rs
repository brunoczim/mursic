use mursic::{
    note::NoteKind,
    num::NaturalRatio,
    pitch::{Key, Pitch},
    player::Player,
    song::{PlayableSongBuilder, SongBuilder},
    tempo::{Dot, NoteValue, TimeSignature},
    wave::SquareWaveBuilder,
};

fn main() {
    let octave = 5;
    let mut builder = SongBuilder::default();
    builder
        .bpm(NoteValue::Quarter, NaturalRatio::from(160))
        .signature(TimeSignature { numer: 1, denom: NoteValue::Quarter });

    make_verse_intro(octave, &mut builder);
    make_mid(octave, &mut builder);
    make_first_outro(octave, &mut builder);

    make_verse_intro(octave, &mut builder);
    make_mid(octave, &mut builder);
    make_second_outro(octave, &mut builder);

    make_chorus_intro(octave, &mut builder);
    make_mid(octave, &mut builder);
    make_first_outro(octave, &mut builder);

    builder
        .note_value(NoteValue::Quarter)
        .dot(Dot::None)
        .note_group()
        .compass();

    make_chorus_intro(octave, &mut builder);
    make_mid(octave, &mut builder);
    make_second_outro(octave, &mut builder);

    let song = builder.clear_finish();

    let playable = PlayableSongBuilder::default()
        .finish(song, SquareWaveBuilder::default());
    let player = Player::new().unwrap();
    player.play(playable);
    player.wait();
}

fn make_verse_intro(octave: u32, builder: &mut SongBuilder) {
    builder
        .pitch(Pitch { octave, key: Key::A })
        .note_kind(NoteKind::Plain)
        .note_value(NoteValue::Quarter)
        .dot(Dot::None)
        .note()
        .note_group()
        .compass()
        .signature(TimeSignature { numer: 3, denom: NoteValue::Quarter })
        .pitch(Pitch { octave: octave + 1, key: Key::C })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note()
        .note_group()
        .pitch(Pitch { octave: octave + 1, key: Key::D })
        .note_value(NoteValue::Quarter)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note_kind(NoteKind::Ligature)
        .note()
        .note_group()
        .compass()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave: octave + 1, key: Key::E })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::C })
        .note()
        .note_group();
}

fn make_chorus_intro(octave: u32, builder: &mut SongBuilder) {
    builder
        .pitch(Pitch { octave: octave + 1, key: Key::G })
        .note_value(NoteValue::Half)
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::C })
        .note()
        .note_group()
        .compass()
        .pitch(Pitch { octave: octave + 1, key: Key::G })
        .note_value(NoteValue::Quarter)
        .note()
        .pitch(Pitch { octave, key: Key::C })
        .note()
        .note_group();
}

fn make_mid(octave: u32, builder: &mut SongBuilder) {
    builder
        .pitch(Pitch { octave: octave + 1, key: Key::F })
        .note_kind(NoteKind::Plain)
        .note_value(NoteValue::Eighth)
        .dot(Dot::None)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave, key: Key::C })
        .note()
        .note_group()
        .note()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave: octave + 1, key: Key::E })
        .note_value(NoteValue::Quarter)
        .note()
        .note_group()
        .compass()
        .pitch(Pitch { octave: octave + 1, key: Key::D })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::G })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Quarter)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave: octave - 1, key: Key::G })
        .note()
        .note_group()
        .compass()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::G })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::E })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::A })
        .dot(Dot::None)
        .note_value(NoteValue::Eighth)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::E })
        .note_kind(NoteKind::Ligature)
        .note()
        .note_group()
        .note()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Quarter)
        .note()
        .note_group()
        .compass();
}

fn make_first_outro(octave: u32, builder: &mut SongBuilder) {
    builder
        .pitch(Pitch { octave: octave + 1, key: Key::C })
        .note_value(NoteValue::Half)
        .note_kind(NoteKind::Plain)
        .dot(Dot::None)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note()
        .note_group()
        .compass()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::A })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave, key: Key::C })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::Ab })
        .dot(Dot::None)
        .note_value(NoteValue::Eighth)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave, key: Key::C })
        .note()
        .note_group()
        .note()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .note()
        .note_group()
        .compass()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::B })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::Ab })
        .note_value(NoteValue::Quarter)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave: octave - 1, key: Key::B })
        .note()
        .note_group()
        .compass()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::E })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::E })
        .note()
        .note_group();
}

fn make_second_outro(octave: u32, builder: &mut SongBuilder) {
    builder
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave: octave + 1, key: Key::C })
        .note_value(NoteValue::Quarter)
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::B })
        .note_value(NoteValue::Eighth)
        .dot(Dot::None)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note()
        .note_group()
        .note()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Quarter)
        .note()
        .note_group()
        .compass()
        .pitch(Pitch { octave, key: Key::Ab })
        .dot(Dot::Single)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::E })
        .note()
        .note_group()
        .pitch(Pitch { octave, key: Key::Gb })
        .dot(Dot::None)
        .note_value(NoteValue::Eighth)
        .note()
        .note_kind(NoteKind::Ligature)
        .pitch(Pitch { octave: octave - 1, key: Key::E })
        .note()
        .note_group()
        .note()
        .note_kind(NoteKind::Plain)
        .pitch(Pitch { octave, key: Key::Ab })
        .note_value(NoteValue::Quarter)
        .note()
        .note_group()
        .compass()
        .pitch(Pitch { octave, key: Key::A })
        .note_value(NoteValue::Half)
        .note()
        .pitch(Pitch { octave: octave - 1, key: Key::A })
        .note()
        .note_group()
        .note_value(NoteValue::Quarter)
        .note_group()
        .compass();
}
