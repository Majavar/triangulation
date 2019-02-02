pub trait Settings {
    fn seed(&self) -> u64;
    fn number(&self) -> u64;
}
