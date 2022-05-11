use std::{
    fmt::{Display, Formatter},
    num::ParseIntError,
};

use thiserror::Error;
use uuid::Uuid;

#[derive(Eq, PartialEq, Debug)]
pub struct CorrelationVector {
    base: String,
    vector: Vec<u8>,
}

impl CorrelationVector {
    pub fn new() -> CorrelationVector {
        Self::new_from_uuid(Uuid::new_v4())
    }

    pub fn new_from_uuid(base: Uuid) -> CorrelationVector {
        let mut base_string = base64::encode(base.as_bytes());
        while let Some(c) = base_string.pop() {
            if c != '=' {
                base_string.push(c);
                break;
            }
        }
        base_string.shrink_to_fit();
        CorrelationVector {
            base: base_string, 
            vector: vec![0],
        }
    }

    pub fn parse(input: &str) -> Result<CorrelationVector, CorrelationVectorParseError> {
        let parts = input.split('.').collect::<Vec<&str>>();
        match *parts.as_slice() {
            [base, _first, ..] => Ok(CorrelationVector {
                base: base.to_string(),
                vector: parts[1..]
                    .iter()
                    .map(|s| s.parse::<u8>())
                    .collect::<Result<Vec<u8>, ParseIntError>>()?,
            }),
            [_] => Err(CorrelationVectorParseError::MissingVector),
            [] => Err(CorrelationVectorParseError::Empty),
        }
    }

    pub fn extend(&mut self) {
        self.vector.push(0);
    }

    pub fn increment(&mut self) {
        let last_index = self.vector.len() - 1;
        self.vector[last_index] += 1;
    }

    pub fn spin(&mut self) {
        unimplemented!()
    }
}

impl Default for CorrelationVector {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CorrelationVector {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let vector_string: String = self
            .vector
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(".");
        write!(f, "{}.{}", self.base, vector_string)
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_cv() {
        let cv = CorrelationVector::new();
        let cv_string = cv.to_string();
        assert_eq!(cv_string.split('.').count(), 2);
    }

    #[test]
    fn parse_cv_works() {
        let cv = CorrelationVector::new();
        let cv_string = cv.to_string();
        let cv_parsed = CorrelationVector::parse(&cv_string);
        assert_eq!(cv, cv_parsed.expect("Failed to parse cV"));
    }

    #[test]
    fn increment_cv() {
        let mut cv = CorrelationVector::new();
        cv.increment();
        let cv_string = cv.to_string();
        assert!(cv_string.ends_with('1'));
    }

    #[test]
    fn extend_cv() {
        let mut cv = CorrelationVector::new();
        cv.extend();
        let cv_string = cv.to_string();
        assert_eq!(cv_string.split('.').count(), 3);
    }

}
