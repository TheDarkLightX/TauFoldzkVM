pub struct Calculator {
    pub display: String,
    pub current_value: f64,
    pub pending_operation: Option<Operation>,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: "0".to_string(),
            current_value: 0.0,
            pending_operation: None,
        }
    }
}