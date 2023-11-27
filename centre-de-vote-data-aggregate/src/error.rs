#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;