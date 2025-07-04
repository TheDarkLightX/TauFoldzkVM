
#[derive(Clone)]
pub struct Calculator {
    pub display: String,
    pub current_value: f64,
    pub stored_value: f64,
    pub pending_operation: Option<Operation>,
    pub memory: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add = 0,
    Subtract = 1,
    Multiply = 2,
    Divide = 3,
    Power = 4,
    Modulo = 5,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: "0".to_string(),
            current_value: 0.0,
            stored_value: 0.0,
            pending_operation: None,
            memory: 0.0,
        }
    }
    
    pub fn input_digit(&mut self, digit: char) {
        if self.display == "0" {
            self.display = digit.to_string();
        } else if self.display.len() < 10 {
            self.display.push(digit);
        }
        self.current_value = self.display.parse().unwrap_or(0.0);
    }
    
    pub fn input_decimal(&mut self) {
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }
    
    pub fn set_operation(&mut self, op: Operation) {
        self.stored_value = self.current_value;
        self.pending_operation = Some(op);
        self.display = "0".to_string();
        self.current_value = 0.0;
    }
    
    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.stored_value = 0.0;
        self.pending_operation = None;
    }
    
    pub fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
    }
    
    pub fn memory_store(&mut self) {
        self.memory = self.current_value;
    }
    
    pub fn memory_recall(&mut self) {
        self.current_value = self.memory;
        self.display = self.format_number(self.memory);
    }
    
    pub fn memory_add(&mut self) {
        self.memory += self.current_value;
    }
    
    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }
    
    pub fn calculate(&mut self) {
        if let Some(op) = self.pending_operation {
            let result = match op {
                Operation::Add => self.stored_value + self.current_value,
                Operation::Subtract => self.stored_value - self.current_value,
                Operation::Multiply => self.stored_value * self.current_value,
                Operation::Divide => {
                    if self.current_value != 0.0 {
                        self.stored_value / self.current_value
                    } else {
                        self.display = "Error".to_string();
                        return;
                    }
                }
                Operation::Power => self.stored_value.powf(self.current_value),
                Operation::Modulo => {
                    if self.current_value != 0.0 {
                        self.stored_value % self.current_value
                    } else {
                        self.display = "Error".to_string();
                        return;
                    }
                }
            };
            self.current_value = result;
            self.display = self.format_number(result);
            self.pending_operation = None;
            self.stored_value = 0.0;
        }
    }
    
    pub fn prepare_zkvm_input(&self) -> Vec<u32> {
        if let Some(op) = self.pending_operation {
            vec![
                op as u32,
                self.float_to_fixed_point(self.stored_value),
                self.float_to_fixed_point(self.current_value),
            ]
        } else {
            vec![]
        }
    }
    
    pub fn process_zkvm_result(&mut self, result: &[u32]) {
        if !result.is_empty() {
            self.current_value = self.fixed_point_to_float(result[0]);
            self.display = self.format_number(self.current_value);
            self.pending_operation = None;
        }
    }
    
    fn float_to_fixed_point(&self, value: f64) -> u32 {
        // Convert to fixed point with 4 decimal places
        ((value * 10000.0) as i32).max(0) as u32
    }
    
    fn fixed_point_to_float(&self, value: u32) -> f64 {
        (value as f64) / 10000.0
    }
    
    fn format_number(&self, value: f64) -> String {
        if value.fract() == 0.0 && value.abs() < 1e9 {
            format!("{:.0}", value)
        } else {
            format!("{:.4}", value).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}