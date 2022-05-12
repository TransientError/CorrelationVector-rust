use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
    num::ParseIntError,
    time::SystemTime,
};

use uuid::Uuid;

use crate::{
    correlationvectorparsererror::CorrelationVectorParseError,
    spinparams::{generate_entropy, tick_periodicity_bits, ticks_to_drop, SpinParams},
};

const TerminationSymbol: &str = "!";

#[derive(Eq, PartialEq, Debug)]
pub struct CorrelationVector {
    base: String,
    vector: Vec<u32>,
    immutable: bool,
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
            immutable: false,
        }
    }

    pub fn parse(input: &str) -> Result<CorrelationVector, CorrelationVectorParseError> {
        let parts = input
            .split('.')
            .filter(|&e| e != TerminationSymbol)
            .collect::<Vec<&str>>();
        match *parts.as_slice() {
            [base, _first, ..] => Ok(CorrelationVector {
                base: base.to_string(),
                vector: parts[1..]
                    .iter()
                    .map(|s| s.parse::<u32>())
                    .collect::<Result<Vec<u32>, ParseIntError>>()?,
                immutable: input.ends_with(TerminationSymbol),
            }),
            [_] => Err(CorrelationVectorParseError::MissingVector),
            [] => Err(CorrelationVectorParseError::Empty),
        }
    }

    pub fn extend(&mut self) {
        if self.immutable {
            return;
        }
        self.vector.push(0);
    }

    pub fn increment(&mut self) {
        if self.immutable {
            return;
        }
        let last_index = self.vector.len() - 1;
        self.vector[last_index] += 1;
    }

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

        self.vector.push(value as u32);
        if tick_bitmask_bits > 32 {
            self.vector.push((value >> 32).try_into().unwrap());
        }

        self.vector.push(0)
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
}
