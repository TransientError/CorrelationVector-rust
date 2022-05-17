use thiserror::Error;

/// The error type for the correlation vector parse function
#[derive(Debug, Error)]
pub enum CorrelationVectorParseError {
    /// The input is empty
    #[error("Empty input")]
    Empty,
    /// There was no vector clock in the input
    #[error("Missing vector portion of correlation vector")]
    MissingVector,
    /// The numbers in the vector clock could not be parsed as u32
    #[error("Invalid vector portion of correlation vector")]
    ParseError {
        #[from]
        source: std::num::ParseIntError,
    },
    /// The input is too long to form a valid correlation vector according to the specification
    #[error("String is too long to be a valid correlation vector")]
    StringTooLongError,
}
