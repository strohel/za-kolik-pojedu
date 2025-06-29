use crate::provider::{bolt::Bolt, car4way::Car4way};
use dioxus::signals::{Readable, Signal};

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
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderKind {
    Bolt(Signal<Bolt>),
    Car4way(Signal<Car4way>),
}
