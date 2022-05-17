/// The parameters for the spin operation
#[derive(Debug, Clone, Copy)]
pub struct SpinParams {
    /// The number of ticks to drop from the UTC timestamp
    pub spin_counter_interval: SpinCounterInterval,
    /// The number of bits to use from the UTC timestamp
    pub spin_counter_periodicity: SpinCounterPeriodicity,
    /// How many entropy bytes to use
    pub spin_entropy: SpinEntropy,
}

/// The number of ticks to drop from the UTC timestamp
#[derive(Debug, Clone, Copy)]
pub enum SpinCounterInterval {
    /// Drop 24 bits
    Coarse,
    /// Drop 16 bits
    Fine,
}

pub(crate) fn ticks_to_drop(interval: SpinCounterInterval) -> u64 {
    match interval {
        SpinCounterInterval::Coarse => 24,
        SpinCounterInterval::Fine => 16,
    }
}

/// The number of bits to use from the UTC timestamp
#[derive(Debug, Clone, Copy)]
pub enum SpinCounterPeriodicity {
    None,
    /// use 16 bits
    Short,
    /// use 24 bits
    Medium,
    /// use 32 bits
    Long,
}

pub(crate) fn tick_periodicity_bits(params: SpinParams) -> u64 {
    let counter_bits = match params.spin_counter_periodicity {
        SpinCounterPeriodicity::None => 0,
        SpinCounterPeriodicity::Short => 16,
        SpinCounterPeriodicity::Medium => 24,
        SpinCounterPeriodicity::Long => 32,
    };

    counter_bits + entropy_bytes(params.spin_entropy) * 8
}

/// How many entropy bytes to use
#[derive(Debug, Clone, Copy)]
pub enum SpinEntropy {
    None,
    One,
    Two,
    Three,
    Four,
}

pub(crate) fn entropy_bytes(entropy: SpinEntropy) -> u64 {
    match entropy {
        SpinEntropy::None => 0,
        SpinEntropy::One => 1,
        SpinEntropy::Two => 2,
        SpinEntropy::Three => 3,
        SpinEntropy::Four => 4,
    }
}

pub(crate) fn generate_entropy(entropy: SpinEntropy) -> Vec<u8> {
    let bytes_to_generate = entropy_bytes(entropy);

    let mut result = Vec::new();
    for _ in 0..bytes_to_generate {
        result.push(rand::random::<u8>());
    }
    result
}
