//! A rust library for working with Correlation Vectors.
//! The library provides a struct for CorrelationVector and methods for working with them.
//! Learn more about the methods in the [Readme](https://github.com/TransientError/CorrelationVector-rust/blob/master/cvlib/Readme.md)
//! or in the [specification](https://github.com/microsoft/CorrelationVector).
//! ```rust
//! use cvlib::CorrelationVector;
//!
//! let mut cv = CorrelationVector::new(); // e.g. wC71fJEqSPuHrPQ9ZoXrKg.0
//! cv.extend(); // e.g. wC71fJEqSPuHrPQ9ZoXrKg.0.0
//! cv.increment(); // e.g. wC71fJEqSPuHrPQ9ZoXrKg.0.1
//! let cv_string = cv.to_string(); // create string representation of CV
//!
//! let cv_parsed = CorrelationVector::parse(&cv_string); // parse the string representation of the correlation vector
//! ```

mod correlationvector;
mod correlationvectorparsererror;
mod spinparams;

pub use correlationvector::CorrelationVector;
pub use correlationvectorparsererror::CorrelationVectorParseError;
pub use spinparams::{SpinCounterInterval, SpinCounterPeriodicity, SpinEntropy, SpinParams};
