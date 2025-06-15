use crate::provider::ProviderCalculator;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Bolt {}

impl Bolt {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProviderCalculator for Bolt {}
