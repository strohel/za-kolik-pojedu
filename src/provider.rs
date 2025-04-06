use crate::provider::{bolt::Bolt, car4way::Car4way};

pub mod bolt;
pub mod car4way;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Provider {
    Bolt(Bolt),
    Car4way(Car4way),
}

impl Provider {
    pub fn name(&self) -> &str {
        match self {
            Provider::Bolt(bolt) => bolt.name(),
            Provider::Car4way(car4way) => car4way.name(),
        }
    }
}
