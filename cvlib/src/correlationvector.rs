use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
    num::ParseIntError,
    time::SystemTime,
};

use uuid::Uuid;

use crate::{
    correlationvectorparsererror::CorrelationVectorParseError,
    spinparams::{generate_entropy, tick_periodicity_bits, ticks_to_drop, SpinParams},
};

const TERMINATION_SYMBOL: &str = "!";

/// The Correlation Vector struct
#[derive(Eq, PartialEq, Debug)]
pub struct CorrelationVector {
    base: String,
    vector: Vec<u32>,
    immutable: bool,
    serialized_length: usize,
}

impl CorrelationVector {
    /// Creates a new CorrelationVector with a randomly generated UUID.
    pub fn new() -> CorrelationVector {
        Self::new_from_uuid(Uuid::new_v4())
    }

    /// Create a new CorrelationVector from a given UUID.
    pub fn new_from_uuid(base: Uuid) -> CorrelationVector {
        let mut base_string = base64::encode(base.as_bytes());
        while let Some(c) = base_string.pop() {
            if c != '=' {
                base_string.push(c);
                break;
            }
        }
        base_string.shrink_to_fit();
        let base_str_len = base_string.len();
        CorrelationVector {
            base: base_string,
            vector: vec![0],
            immutable: false,
            serialized_length: base_str_len + 2,
        }
    }

    /// Create a new CorrelationVector struct from a string representation of a CorrelationVector.
    pub fn parse(input: &str) -> Result<CorrelationVector, CorrelationVectorParseError> {
        if input.len() > 128 || (input.len() == 128 && !input.ends_with(TERMINATION_SYMBOL)) {
            return Err(CorrelationVectorParseError::StringTooLongError);
        }

        let mut input = input;
        if input.ends_with(TERMINATION_SYMBOL) {
            input = input.trim_end_matches(TERMINATION_SYMBOL);
        }

        let parts = input
            .split('.')
            .collect::<Vec<&str>>();
        match *parts.as_slice() {
            [base, _first, ..] => Ok(CorrelationVector {
                base: base.to_string(),
                vector: parts[1..]
                    .iter()
                    .map(|s| s.parse::<u32>())
                    .collect::<Result<Vec<u32>, ParseIntError>>()?,
                immutable: input.ends_with(TERMINATION_SYMBOL),
                serialized_length: input.len(),
            }),
            [_] => Err(CorrelationVectorParseError::MissingVector),
            [] => Err(CorrelationVectorParseError::Empty),
        }
    }

    /// Append a new clock to the end of the vector clock
    pub fn extend(&mut self) {
        if self.immutable {
            return;
        }
        let proposed_len = self.serialized_length + 2;
        if proposed_len > 127 {
            self.immutable = true;
            return;
        }
        self.vector.push(0);
        self.serialized_length = proposed_len; // .0
    }

    /// Increment the latest clock in the vector clock
    pub fn increment(&mut self) {
        if self.immutable {
            return;
        }
        let last_index = self.vector.len() - 1;
        let prev = self.vector[last_index];

        // if the last digit is 9, the serialized length will increase
        if prev % 10 == 9 {
            if self.serialized_length < 127 {
                self.serialized_length += 1;
            } else {
                self.immutable = true;
            }
        }

        if !self.immutable {
            self.vector[last_index] = prev + 1;
        }
    }

    /// Transform the vector clock in a unique, monotonically increasing way. 
    /// This is mostly used in situations where increment can not guaranatee uniqueness
    pub fn spin(&mut self, params: SpinParams) {
        if self.immutable {
            return;
        }
        let entropy = generate_entropy(params.spin_entropy);
        let ticks = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time is before the 0 epoch")
            .as_nanos()
            / 100;

        let mut value = u64::try_from(ticks >> ticks_to_drop(params.spin_counter_interval))
            .expect("Number of ticks did not fit in u64");

        for byte in entropy {
            value = (value << 8) | u64::from(byte);
        }

        let tick_bitmask_bits = tick_periodicity_bits(params);
        let mask = if tick_bitmask_bits == 64 {
            0
        } else {
            (1 << tick_bitmask_bits) - 1
        };

        value &= mask;

        let first_32_bits = value as u32;
        let proposed_extension_len = serialized_length_of(first_32_bits) + 1;
        if self.serialized_length + proposed_extension_len > 127 {
            self.immutable = true;
            return;
        }
        self.serialized_length += proposed_extension_len;
        self.vector.push(first_32_bits);
        if tick_bitmask_bits > 32 {
            let end_32_bits = (value >> 32) as u32;
            let proposed_extension_len = serialized_length_of(end_32_bits) + 1;
            if self.serialized_length + proposed_extension_len > 127 {
                self.immutable = true;
                return;
            }
            self.vector.push(end_32_bits);
            self.serialized_length += proposed_extension_len;
        }

        if self.serialized_length + 2 > 127 {
            self.immutable = true;
            return;
        }

        self.vector.push(0);
        self.serialized_length += 2;
    }
}

fn serialized_length_of(input: u32) -> usize {
    let mut length = 1;
    let mut input = input;
    while input > 10 {
        length += 1;
        input /= 10;
    }
    length
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
        write!(
            f,
            "{}.{}{}",
            self.base,
            vector_string,
            if self.immutable { "!" } else { "" }
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::spinparams::{SpinCounterInterval, SpinCounterPeriodicity, SpinEntropy};

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

    #[test]
    fn spin_cv() {
        let mut cv = CorrelationVector::new();
        cv.spin(SpinParams {
            spin_entropy: SpinEntropy::Two,
            spin_counter_interval: SpinCounterInterval::Fine,
            spin_counter_periodicity: SpinCounterPeriodicity::Short,
        });

        let cv_string = cv.to_string();
        assert!(cv_string.ends_with('0'));
    }

    #[test]
    fn extend_stops_when_oversize() {
        let mut cv = CorrelationVector::new();
        for _ in 0..128 {
            cv.extend();
        }
        let cv_string = cv.to_string();
        assert!(cv_string.len() <= 128);
        assert!(cv_string.ends_with(TERMINATION_SYMBOL));
    }

    #[test]
    fn spin_stops_when_oversize() {
        let mut cv = CorrelationVector::new();
        for _ in 0..128 {
            cv.spin(SpinParams {
                spin_entropy: SpinEntropy::Two,
                spin_counter_interval: SpinCounterInterval::Fine,
                spin_counter_periodicity: SpinCounterPeriodicity::Short,
            });
        }
        let cv_string = cv.to_string();
        assert!(cv_string.len() <= 128, "{}", cv_string.len());
        assert!(cv_string.ends_with(TERMINATION_SYMBOL));
        println!("{}", cv_string);
    }

    #[test]
    fn increment_stops_when_oversize() {
        let mut cv = CorrelationVector::parse(
            "P9v1ltK2S7qTS77z0lWtKg.0.386394219.0.386383989.0.386344389.0.386372594.0.386391233.0.386360320.0\
            .386386342.0.386341105.12344459"
        ).unwrap();

        cv.increment();

        let cv_string = cv.to_string();
        assert_eq!(cv_string.len(), 128);
        assert!(cv_string.ends_with(TERMINATION_SYMBOL));
    }

    #[test]
    fn parse_terminated() {
        let res = CorrelationVector::parse("base.0!");
        assert!(res.is_ok(), "{:?}", res);
    }
}
