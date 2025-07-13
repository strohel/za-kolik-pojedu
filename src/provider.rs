use crate::{
    provider::{bolt::Bolt, car4way::Car4way},
    TripInputData,
};
use dioxus::signals::{Readable, Signal};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

pub mod bolt;
pub mod car4way;

#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
    pub enabled: Signal<bool>,
    pub kind: ProviderKind,
}

impl Provider {
    pub fn new(enabled: Signal<bool>, kind: ProviderKind) -> Self {
        Self { enabled, kind }
    }

    pub fn name(&self) -> &'static str {
        match &self.kind {
            ProviderKind::Bolt(bolt) => bolt.read().name(),
            ProviderKind::Car4way(car4way) => car4way.read().name(),
        }
    }

    pub fn calculate(&self, input_data: Signal<TripInputData>) -> CalculationResult {
        match &self.kind {
            ProviderKind::Bolt(_bolt) => {
                CalculationResult { price_czk: 0.0, details: String::from("TODO Bolt calculate") }
            },
            ProviderKind::Car4way(car4way) => car4way.read().calculate(*input_data.read()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderKind {
    Bolt(Signal<Bolt>),
    Car4way(Signal<Car4way>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CalculationResult {
    pub price_czk: f64,
    pub details: String,
}

impl Display for CalculationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let CalculationResult { price_czk, details } = &self;
        write!(f, "{price_czk} Kč {details}")
    }
}

impl Eq for CalculationResult {}

#[allow(clippy::derive_ord_xor_partial_ord)] // Ord and PartialOrd impls match
impl Ord for CalculationResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("we use only comparable float values")
    }
}
