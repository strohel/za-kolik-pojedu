use crate::provider::{bolt::Bolt, car4way::Car4way};

pub mod bolt;
pub mod car4way;

#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
    enabled: bool,
    kind: ProviderKind,
}

impl Provider {
    pub fn new(kind: ProviderKind) -> Self {
        Self { enabled: true, kind }
    }

    pub fn name(&self) -> &str {
        match &self.kind {
            ProviderKind::Bolt(bolt) => bolt.name(),
            ProviderKind::Car4way(car4way) => car4way.name(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderKind {
    Bolt(Bolt),
    Car4way(Car4way),
}
