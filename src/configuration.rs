#[derive(Clone)]
pub struct Configuration {}

impl Configuration {
    pub fn load() -> anyhow::Result<Self> {
        // TODO do something meaningful
        Ok(Self {})
    }
}
