use crate::{
    TripInputData,
    provider::{bolt::Bolt, car4way::Car4way},
};
use dioxus::signals::{Readable, Signal};
use std::cmp::Ordering;

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
                CalculationResult { car_type: "TODO".into(), components: vec![] }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationResult {
    pub car_type: String,
    pub components: Vec<PriceComponent>,
}

impl CalculationResult {
    pub fn total_czk(&self) -> f64 {
        self.components.iter().map(|c| c.czk).sum()
    }
}

#[expect(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for CalculationResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_czk().partial_cmp(&other.total_czk())
    }
}

impl Ord for CalculationResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("our floats compare")
    }
}

#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct PriceComponent {
    pub czk: f64,
    pub name: String,
}

// We use floats that compare OK.
impl Eq for PriceComponent {}
