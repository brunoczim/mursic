use crate::pitch::Pitch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MannerKind {
    Plain,
    Ligature,
    Rest,
}

impl MannerKind {
    pub fn make_note(self, pitch: Pitch) -> Manner {
        match self {
            MannerKind::Plain => Manner::Plain(pitch),
            MannerKind::Ligature => Manner::Ligature,
            MannerKind::Rest => Manner::Rest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Manner {
    Plain(Pitch),
    Ligature,
    Rest,
}

impl Manner {
    pub fn kind(self) -> MannerKind {
        match self {
            Manner::Plain(_) => MannerKind::Plain,
            Manner::Ligature => MannerKind::Ligature,
            Manner::Rest => MannerKind::Rest,
        }
    }
}
