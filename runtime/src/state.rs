//! VM state management and execution results

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{VmError, ExecutionStats, TraceEntry};

/// Complete VM state containing all registers, memory, and execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmState {
    /// General-purpose registers (32-bit each)
    pub registers: Vec<u32>,
    
    /// Execution stack with automatic management
    pub stack: Vec<u32>,
    
    /// Main memory (64KB addressable space)
    pub memory: Vec<u32>,
    
    /// Program counter
    pub program_counter: u32,
    
    /// Execution state
    pub halted: bool,
    pub cycle_count: u64,
    
    /// Cryptographic state
    pub last_hash: Option<u32>,
    pub signatures: HashMap<u32, bool>,
    
    /// I/O buffers
    pub input_buffer: Vec<u32>,
    pub output_buffer: Vec<u32>,
    
    /// Call stack for function calls
    pub call_stack: Vec<u32>,
}

impl VmState {
    /// Create new VM state with default configuration
    pub fn new(memory_size: usize, register_count: usize) -> Self {
        Self {
            registers: vec![0; register_count],
            stack: Vec::new(),
            memory: vec![0; memory_size],
            program_counter: 0,
            halted: false,
            cycle_count: 0,
            last_hash: None,
            signatures: HashMap::new(),
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
            call_stack: Vec::new(),
        }
    }
    
    /// Reset VM state to initial conditions
    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.stack.clear();
        self.memory.fill(0);
        self.program_counter = 0;
        self.halted = false;
        self.cycle_count = 0;
        self.last_hash = None;
        self.signatures.clear();
        self.input_buffer.clear();
        self.output_buffer.clear();
        self.call_stack.clear();
    }
    
    /// Check if memory address is valid
    pub fn is_valid_memory_address(&self, address: u32) -> bool {
        (address as usize) < self.memory.len()
    }
    
    /// Get memory value at address (safe)
    pub fn get_memory(&self, address: u32) -> Result<u32, VmError> {
        if !self.is_valid_memory_address(address) {
            return Err(VmError::InvalidMemoryAccess { address });
        }
        Ok(self.memory[address as usize])
    }
    
    /// Set memory value at address (safe)
    pub fn set_memory(&mut self, address: u32, value: u32) -> Result<(), VmError> {
        if !self.is_valid_memory_address(address) {
            return Err(VmError::InvalidMemoryAccess { address });
        }
        self.memory[address as usize] = value;
        Ok(())
    }
    
    /// Push value to stack
    pub fn push_stack(&mut self, value: u32) {
        self.stack.push(value);
    }
    
    /// Pop value from stack
    pub fn pop_stack(&mut self) -> Result<u32, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow {
            operation: "pop".to_string(),
            required: 1,
        })
    }
    
    /// Peek at top of stack without removing
    pub fn peek_stack(&self) -> Result<u32, VmError> {
        self.stack.last().copied().ok_or(VmError::StackUnderflow {
            operation: "peek".to_string(),
            required: 1,
        })
    }
    
    /// Check if stack has at least n elements
    pub fn has_stack_elements(&self, n: usize) -> bool {
        self.stack.len() >= n
    }
    
    /// Get register value (safe)
    pub fn get_register(&self, index: usize) -> Result<u32, VmError> {
        self.registers.get(index).copied().ok_or(VmError::ProgramError {
            message: format!("Invalid register index: {}", index),
        })
    }
    
    /// Set register value (safe)
    pub fn set_register(&mut self, index: usize, value: u32) -> Result<(), VmError> {
        if index >= self.registers.len() {
            return Err(VmError::ProgramError {
                message: format!("Invalid register index: {}", index),
            });
        }
        self.registers[index] = value;
        Ok(())
    }
    
    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        self.memory.len() * 4 + // Memory
        self.stack.len() * 4 + // Stack
        self.registers.len() * 4 + // Registers
        self.input_buffer.len() * 4 + // Input buffer
        self.output_buffer.len() * 4 + // Output buffer
        self.call_stack.len() * 4 + // Call stack
        self.signatures.len() * 8 // Signatures (approximate)
    }
    
    /// Create a snapshot of current state for debugging
    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            cycle: self.cycle_count,
            pc: self.program_counter,
            stack: self.stack.clone(),
            registers: self.registers.clone(),
            stack_size: self.stack.len(),
            memory_usage: self.memory_usage(),
            halted: self.halted,
        }
    }
}

impl Default for VmState {
    fn default() -> Self {
        Self::new(65536, 16) // 64KB memory, 16 registers
    }
}

/// Lightweight snapshot of VM state for debugging and tracing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub cycle: u64,
    pub pc: u32,
    pub stack: Vec<u32>,
    pub registers: Vec<u32>,
    pub stack_size: usize,
    pub memory_usage: usize,
    pub halted: bool,
}

/// Result of program execution with comprehensive information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether execution completed successfully
    pub success: bool,
    
    /// Final VM state after execution
    pub final_state: VmState,
    
    /// Execution statistics and performance metrics
    pub stats: ExecutionStats,
    
    /// Error information if execution failed
    pub error: Option<String>,
    
    /// Execution trace (if enabled)
    pub trace: Vec<TraceEntry>,
    
    /// Constraint violations (if any)
    pub constraint_violations: Vec<ConstraintViolation>,
}

/// Information about a constraint violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintViolation {
    pub cycle: u64,
    pub instruction: String,
    pub inputs: Vec<u32>,
    pub outputs: Vec<u32>,
    pub details: String,
}

impl ExecutionResult {
    /// Create a successful execution result
    pub fn success(final_state: VmState, stats: ExecutionStats) -> Self {
        Self {
            success: true,
            final_state,
            stats,
            error: None,
            trace: Vec::new(),
            constraint_violations: Vec::new(),
        }
    }
    
    /// Create a failed execution result
    pub fn failure(
        final_state: VmState,
        stats: ExecutionStats,
        error: String,
    ) -> Self {
        Self {
            success: false,
            final_state,
            stats,
            error: Some(error),
            trace: Vec::new(),
            constraint_violations: Vec::new(),
        }
    }
    
    /// Add execution trace
    pub fn with_trace(mut self, trace: Vec<TraceEntry>) -> Self {
        self.trace = trace;
        self
    }
    
    /// Add constraint violations
    pub fn with_violations(mut self, violations: Vec<ConstraintViolation>) -> Self {
        self.constraint_violations = violations;
        self
    }
    
    /// Check if execution had any constraint violations
    pub fn has_violations(&self) -> bool {
        !self.constraint_violations.is_empty()
    }
    
    /// Get the final value on the stack (if any)
    pub fn final_stack_value(&self) -> Option<u32> {
        self.final_state.stack.last().copied()
    }
    
    /// Get execution summary as string
    pub fn summary(&self) -> String {
        let status = if self.success { "SUCCESS" } else { "FAILED" };
        let cycles = self.stats.cycles_executed;
        let instructions = self.stats.instructions_executed;
        let violations = self.constraint_violations.len();
        
        format!(
            "{} - {} cycles, {} instructions, {} violations",
            status, cycles, instructions, violations
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_state_creation() {
        let state = VmState::new(1024, 8);
        assert_eq!(state.memory.len(), 1024);
        assert_eq!(state.registers.len(), 8);
        assert_eq!(state.stack.len(), 0);
        assert_eq!(state.program_counter, 0);
        assert!(!state.halted);
    }
    
    #[test]
    fn test_stack_operations() {
        let mut state = VmState::default();
        
        // Test push and pop
        state.push_stack(42);
        state.push_stack(100);
        
        assert_eq!(state.stack.len(), 2);
        assert_eq!(state.peek_stack().unwrap(), 100);
        assert_eq!(state.pop_stack().unwrap(), 100);
        assert_eq!(state.pop_stack().unwrap(), 42);
        
        // Test underflow
        assert!(state.pop_stack().is_err());
    }
    
    #[test]
    fn test_memory_operations() {
        let mut state = VmState::new(100, 4);
        
        // Test valid memory access
        assert!(state.set_memory(50, 12345).is_ok());
        assert_eq!(state.get_memory(50).unwrap(), 12345);
        
        // Test invalid memory access
        assert!(state.get_memory(200).is_err());
        assert!(state.set_memory(200, 0).is_err());
    }
    
    #[test]
    fn test_register_operations() {
        let mut state = VmState::new(100, 4);
        
        // Test valid register access
        assert!(state.set_register(2, 98765).is_ok());
        assert_eq!(state.get_register(2).unwrap(), 98765);
        
        // Test invalid register access
        assert!(state.get_register(10).is_err());
        assert!(state.set_register(10, 0).is_err());
    }
    
    #[test]
    fn test_state_reset() {
        let mut state = VmState::default();
        
        // Modify state
        state.push_stack(100);
        state.set_memory(10, 200).unwrap();
        state.set_register(1, 300).unwrap();
        state.program_counter = 50;
        state.cycle_count = 1000;
        state.halted = true;
        
        // Reset state
        state.reset();
        
        // Verify reset
        assert_eq!(state.stack.len(), 0);
        assert_eq!(state.get_memory(10).unwrap(), 0);
        assert_eq!(state.get_register(1).unwrap(), 0);
        assert_eq!(state.program_counter, 0);
        assert_eq!(state.cycle_count, 0);
        assert!(!state.halted);
    }
    
    #[test]
    fn test_execution_result() {
        let state = VmState::default();
        let stats = ExecutionStats::default();
        
        let success_result = ExecutionResult::success(state.clone(), stats.clone());
        assert!(success_result.success);
        assert!(success_result.error.is_none());
        
        let failure_result = ExecutionResult::failure(
            state, 
            stats, 
            "Test error".to_string()
        );
        assert!(!failure_result.success);
        assert_eq!(failure_result.error.as_ref().unwrap(), "Test error");
    }
}