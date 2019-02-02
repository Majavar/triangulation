#[derive(Debug, serde_derive::Deserialize)]
pub struct Settings {
    #[serde(default = "default_number")]
    number: u64,
    #[serde(default = "default_seed")]
    seed: u64,
}

const fn default_number() -> u64 {
    100
}

fn default_seed() -> u64 {
    rand::random()
}

impl application::Settings for Settings {
    fn seed(&self) -> u64 {
        self.seed
    }
    fn number(&self) -> u64 {
        self.number
    }
}
