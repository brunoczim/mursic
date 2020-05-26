mod rodio;

use crate::source::Source;

#[derive(Debug)]
pub struct Player {
    #[cfg(target_arch = "wasm32")]
    backend: (),
    #[cfg(not(target_arch = "wasm32"))]
    backend: rodio::Rodio,
}

impl Player {
    pub fn new() -> Option<Self> {
        Backend::new().map(|backend| Player { backend })
    }

    pub fn play<S>(&self, stream: S)
    where
        S: Source + 'static,
    {
        self.backend.play(stream)
    }

    pub fn wait(&self) {
        self.backend.wait()
    }
}

trait Backend: Sized {
    fn new() -> Option<Self>;

    fn play<S>(&self, stream: S)
    where
        S: Source + 'static;

    fn wait(&self);
}
