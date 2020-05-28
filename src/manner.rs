use crate::pitch::Pitch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MannerKind {
    Plain,
    Ligature,
}

impl MannerKind {
    pub fn make_note(self, pitch: Pitch) -> Manner {
        match self {
            MannerKind::Plain => Manner::Plain(pitch),
            MannerKind::Ligature => Manner::Ligature,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Manner {
    Plain(Pitch),
    Ligature,
}

impl Manner {
    pub fn kind(self) -> MannerKind {
        match self {
            Manner::Plain(_) => MannerKind::Plain,
            Manner::Ligature => MannerKind::Ligature,
        }
    }
}
