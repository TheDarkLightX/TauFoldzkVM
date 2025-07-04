pub struct CryptoDemo {
    pub mode: CryptoMode,
}

#[derive(Debug, Clone)]
pub enum CryptoMode {
    Hash,
    Sign,
    Verify,
}

impl CryptoDemo {
    pub fn new() -> Self {
        Self {
            mode: CryptoMode::Hash,
        }
    }
}