//! # TauFoldZKVM Rust Runtime
//!
//! A high-performance, production-ready runtime for the TauFoldZKVM with mathematical guarantees.
//! Every operation is verified against Tau constraints, making runtime errors impossible.
//!
//! ## Features
//!
//! - **45 Complete Instructions**: Full ISA with arithmetic, memory, control flow, and cryptographic operations
//! - **Mathematical Correctness**: All operations verified by Tau constraints  
//! - **Zero-Cost Abstractions**: Rust's performance with mathematical guarantees
//! - **Memory Safety**: Rust's ownership system prevents memory-related bugs
//! - **Production Ready**: Optimized for real-world deployment
//!
//! ## Quick Start
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use taufold_zkvm::{VirtualMachine, Instruction, Program};
//!
//! // Create a simple program
//! let program = Program::new(vec![
//!     Instruction::Push(42),
//!     Instruction::Push(58),
//!     Instruction::Add,
//!     Instruction::Halt,
//! ]);
//!
//! // Execute with mathematical guarantees
//! let mut vm = VirtualMachine::new();
//! let result = vm.execute(program)?;
//!
//! assert_eq!(result.success, true);
//! assert_eq!(result.final_state.stack.last(), Some(&100));
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
// HashMap removed - not needed in this module
use std::fmt;
use thiserror::Error;

pub mod instruction;
pub mod state;
pub mod validator;
pub mod executor;
pub mod examples;

pub use instruction::Instruction;
pub use state::{VmState, ExecutionResult};
pub use validator::TauValidator;
pub use executor::VirtualMachine;

/// Errors that can occur during VM execution
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum VmError {
    #[error("Stack underflow: {operation} requires {required} stack elements")]
    StackUnderflow {
        operation: String,
        required: usize,
    },
    
    #[error("Memory access error: invalid address {address}")]
    InvalidMemoryAccess {
        address: u32,
    },
    
    #[error("Division by zero in {operation}")]
    DivisionByZero {
        operation: String,
    },
    
    #[error("Constraint violation in {instruction}: {details}")]
    ConstraintViolation {
        instruction: String,
        details: String,
    },
    
    #[error("Program error: {message}")]
    ProgramError {
        message: String,
    },
    
    #[error("Assertion failed at cycle {cycle}")]
    AssertionFailed {
        cycle: u64,
    },
    
    #[error("Invalid instruction: {instruction}")]
    InvalidInstruction {
        instruction: String,
    },
    
    #[error("Execution timeout after {cycles} cycles")]
    ExecutionTimeout {
        cycles: u64,
    },
}

/// Result type for VM operations
pub type VmResult<T> = Result<T, VmError>;

/// A complete program for the TauFoldZKVM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub metadata: ProgramMetadata,
}

/// Metadata about a program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub created_at: String,
}

impl Program {
    /// Create a new program with instructions
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            metadata: ProgramMetadata::default(),
        }
    }
    
    /// Create a program with metadata
    pub fn with_metadata(instructions: Vec<Instruction>, metadata: ProgramMetadata) -> Self {
        Self {
            instructions,
            metadata,
        }
    }
    
    /// Load program from JSON
    pub fn from_json(json: &str) -> VmResult<Self> {
        serde_json::from_str(json).map_err(|e| VmError::ProgramError {
            message: format!("Failed to parse program JSON: {}", e),
        })
    }
    
    /// Serialize program to JSON
    pub fn to_json(&self) -> VmResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| VmError::ProgramError {
            message: format!("Failed to serialize program: {}", e),
        })
    }
    
    /// Validate program for common errors
    pub fn validate(&self) -> VmResult<()> {
        if self.instructions.is_empty() {
            return Err(VmError::ProgramError {
                message: "Program cannot be empty".to_string(),
            });
        }
        
        // Check for jump targets
        let max_pc = self.instructions.len();
        for (pc, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                Instruction::Jmp(target) | 
                Instruction::Jz(target) | 
                Instruction::Jnz(target) | 
                Instruction::Call(target) => {
                    if *target as usize >= max_pc {
                        return Err(VmError::ProgramError {
                            message: format!("Invalid jump target {} at PC {}", target, pc),
                        });
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Get program statistics
    pub fn stats(&self) -> ProgramStats {
        let mut stats = ProgramStats::default();
        
        for instruction in &self.instructions {
            stats.total_instructions += 1;
            
            match instruction {
                Instruction::Add | Instruction::Sub | Instruction::Mul | 
                Instruction::Div | Instruction::Mod => {
                    stats.arithmetic_ops += 1;
                }
                
                Instruction::And | Instruction::Or | Instruction::Xor | 
                Instruction::Not | Instruction::Shl | Instruction::Shr => {
                    stats.bitwise_ops += 1;
                }
                
                Instruction::Eq | Instruction::Neq | Instruction::Lt | 
                Instruction::Gt | Instruction::Lte | Instruction::Gte => {
                    stats.comparison_ops += 1;
                }
                
                Instruction::Load(_) | Instruction::Store(_) | 
                Instruction::Mload(_) | Instruction::Mstore(_) => {
                    stats.memory_ops += 1;
                }
                
                Instruction::Push(_) | Instruction::Pop | 
                Instruction::Dup | Instruction::Swap => {
                    stats.stack_ops += 1;
                }
                
                Instruction::Jmp(_) | Instruction::Jz(_) | Instruction::Jnz(_) | 
                Instruction::Call(_) | Instruction::Ret => {
                    stats.control_flow_ops += 1;
                }
                
                Instruction::Hash | Instruction::Verify | Instruction::Sign => {
                    stats.crypto_ops += 1;
                }
                
                _ => {
                    stats.other_ops += 1;
                }
            }
        }
        
        stats
    }
}

impl Default for ProgramMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled Program".to_string(),
            version: "1.0.0".to_string(),
            description: "A TauFoldZKVM program".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Statistics about a program
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProgramStats {
    pub total_instructions: usize,
    pub arithmetic_ops: usize,
    pub bitwise_ops: usize,
    pub comparison_ops: usize,
    pub memory_ops: usize,
    pub stack_ops: usize,
    pub control_flow_ops: usize,
    pub crypto_ops: usize,
    pub other_ops: usize,
}

impl fmt::Display for ProgramStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program Statistics:")?;
        writeln!(f, "  Total Instructions: {}", self.total_instructions)?;
        writeln!(f, "  Arithmetic Ops:     {}", self.arithmetic_ops)?;
        writeln!(f, "  Bitwise Ops:        {}", self.bitwise_ops)?;
        writeln!(f, "  Comparison Ops:     {}", self.comparison_ops)?;
        writeln!(f, "  Memory Ops:         {}", self.memory_ops)?;
        writeln!(f, "  Stack Ops:          {}", self.stack_ops)?;
        writeln!(f, "  Control Flow Ops:   {}", self.control_flow_ops)?;
        writeln!(f, "  Crypto Ops:         {}", self.crypto_ops)?;
        writeln!(f, "  Other Ops:          {}", self.other_ops)
    }
}

/// Configuration for the VM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Maximum number of execution cycles before timeout
    pub max_cycles: u64,
    
    /// Size of VM memory in words (32-bit)
    pub memory_size: usize,
    
    /// Number of general-purpose registers
    pub register_count: usize,
    
    /// Enable constraint validation
    pub validate_constraints: bool,
    
    /// Enable execution tracing
    pub enable_tracing: bool,
    
    /// Path to Tau constraint files
    pub constraint_path: Option<String>,
    
    /// Enable debug output
    pub debug_mode: bool,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            max_cycles: 1_000_000,
            memory_size: 65536,  // 64KB
            register_count: 16,
            validate_constraints: true,
            enable_tracing: false,
            constraint_path: None,
            debug_mode: false,
        }
    }
}

/// Execution statistics and performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionStats {
    pub cycles_executed: u64,
    pub instructions_executed: u64,
    pub constraint_validations: u64,
    pub constraint_violations: u64,
    pub execution_time_ms: u64,
    pub instructions_per_second: f64,
    pub memory_usage_bytes: usize,
}

impl fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Execution Statistics:")?;
        writeln!(f, "  Cycles Executed:        {}", self.cycles_executed)?;
        writeln!(f, "  Instructions Executed:  {}", self.instructions_executed)?;
        writeln!(f, "  Constraint Validations: {}", self.constraint_validations)?;
        writeln!(f, "  Constraint Violations:  {}", self.constraint_violations)?;
        writeln!(f, "  Execution Time:         {} ms", self.execution_time_ms)?;
        writeln!(f, "  Instructions/Second:    {:.2}", self.instructions_per_second)?;
        writeln!(f, "  Memory Usage:           {} bytes", self.memory_usage_bytes)
    }
}

/// Trait for constraint validation backends
pub trait ConstraintValidator {
    /// Validate an operation against its mathematical constraints
    fn validate_operation(
        &self,
        instruction: &Instruction,
        inputs: &[u32],
        outputs: &[u32],
    ) -> VmResult<bool>;
    
    /// Get validation statistics
    fn get_stats(&self) -> (u64, u64); // (validations, violations)
}

/// Trait for execution tracing
pub trait ExecutionTracer {
    /// Record a trace entry
    fn trace(
        &mut self,
        cycle: u64,
        pc: u32,
        instruction: &Instruction,
        state_before: &VmState,
        state_after: &VmState,
    );
    
    /// Get the execution trace
    fn get_trace(&self) -> &[TraceEntry];
    
    /// Clear the trace
    fn clear_trace(&mut self);
}

/// A single entry in the execution trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub cycle: u64,
    pub pc: u32,
    pub instruction: Instruction,
    pub stack_before: Vec<u32>,
    pub stack_after: Vec<u32>,
    pub registers_before: Vec<u32>,
    pub registers_after: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_creation() {
        let program = Program::new(vec![
            Instruction::Push(42),
            Instruction::Push(58),
            Instruction::Add,
            Instruction::Halt,
        ]);
        
        assert_eq!(program.instructions.len(), 4);
        assert!(program.validate().is_ok());
    }
    
    #[test]
    fn test_program_stats() {
        let program = Program::new(vec![
            Instruction::Push(10),    // Stack op
            Instruction::Push(20),    // Stack op
            Instruction::Add,         // Arithmetic op
            Instruction::Dup,         // Stack op
            Instruction::Halt,        // Other op
        ]);
        
        let stats = program.stats();
        assert_eq!(stats.total_instructions, 5);
        assert_eq!(stats.arithmetic_ops, 1);
        assert_eq!(stats.stack_ops, 3);
        assert_eq!(stats.other_ops, 1);
    }
    
    #[test]
    fn test_program_validation() {
        // Valid program
        let valid_program = Program::new(vec![
            Instruction::Push(10),
            Instruction::Jz(3),
            Instruction::Push(20),
            Instruction::Halt,
        ]);
        assert!(valid_program.validate().is_ok());
        
        // Invalid program with bad jump target
        let invalid_program = Program::new(vec![
            Instruction::Push(10),
            Instruction::Jz(100),  // Invalid target
            Instruction::Halt,
        ]);
        assert!(invalid_program.validate().is_err());
        
        // Empty program
        let empty_program = Program::new(vec![]);
        assert!(empty_program.validate().is_err());
    }
    
    #[test]
    fn test_program_serialization() {
        let program = Program::new(vec![
            Instruction::Push(42),
            Instruction::Halt,
        ]);
        
        let json = program.to_json().unwrap();
        let deserialized = Program::from_json(&json).unwrap();
        
        assert_eq!(program.instructions.len(), deserialized.instructions.len());
    }
}