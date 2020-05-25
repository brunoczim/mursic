mod rodio;

#[derive(Debug)]
pub struct Player {
    #[cfg(target = "wasm")]
    backend: (),
    #[cfg(not(target = "wasm"))]
    backend: rodio::Rodio,
}

trait Backend {
    fn new() -> Self;

    fn append<S>(&self, stream: &S)
    where
        S: Stream;

    fn play(&self);

    fn wait(&self);
}
