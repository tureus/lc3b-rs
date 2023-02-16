#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not parse assembly")]
    ParseAssembly(String),
}
