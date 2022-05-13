use thiserror::Error;

#[derive(Debug, Error)]
pub enum CorrelationVectorParseError {
    #[error("Empty input")]
    Empty,
    #[error("Missing vector portion of correlation vector")]
    MissingVector,
    #[error("Invalid vector portion of correlation vector")]
    ParseError {
        #[from]
        source: std::num::ParseIntError,
    },
    #[error("String is too long to be a valid correlation vector")]
    StringTooLongError,
}
