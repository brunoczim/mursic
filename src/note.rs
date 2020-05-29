use crate::{pitch::Pitch, tempo::NoteTime};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteKind {
    Plain,
    Ligature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note {
    pub kind: NoteKind,
    pub pitch: Pitch,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteGroup {
    pub tempo: NoteTime,
    pub notes: BTreeSet<Note>,
}
