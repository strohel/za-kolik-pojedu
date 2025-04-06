#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Car4way {}

impl Car4way {
    pub fn name(&self) -> &str {
        "car4way"
    }
}
