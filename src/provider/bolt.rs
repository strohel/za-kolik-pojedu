#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Bolt {}

impl Bolt {
    pub fn name(&self) -> &'static str {
        "Bolt"
    }
}
