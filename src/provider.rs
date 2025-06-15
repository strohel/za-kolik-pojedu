use crate::provider::{bolt::Bolt, car4way::Car4way};

pub mod bolt;
pub mod car4way;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Bolt,
    Car4way,
}

impl ProviderKind {
    pub fn name(&self) -> &str {
        match self {
            Self::Bolt => "Bolt",
            Self::Car4way => "car4way",
        }
    }

    pub fn new_calculator(&self) -> Box<dyn ProviderCalculator> {
        match self {
            Self::Bolt => Box::new(Bolt::new()),
            Self::Car4way => Box::new(Car4way::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
    pub kind: ProviderKind,
    pub enabled: bool,
}

impl Provider {
    pub fn new(kind: ProviderKind) -> Self {
        Self { kind, enabled: true }
    }
}

pub trait ProviderCalculator {}
