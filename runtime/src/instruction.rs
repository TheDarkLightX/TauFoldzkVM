//! Instruction set definition for TauFoldZKVM
//!
//! This module defines all 45 instructions supported by the TauFoldZKVM,
//! each with mathematical correctness guarantees through Tau constraints.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Complete 45-instruction set of TauFoldZKVM
///
/// Each instruction is mathematically verified through Tau constraints,
/// ensuring correctness by construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Instruction {
    // Arithmetic Operations (5 instructions)
    /// 32-bit addition with carry propagation
    Add,
    /// 32-bit subtraction with borrow handling
    Sub,
    /// 32-bit multiplication via partial products
    Mul,
    /// 32-bit division with remainder computation
    Div,
    /// 32-bit modulo operation
    Mod,

    // Bitwise Operations (5 instructions)
    /// Bitwise logical AND
    And,
    /// Bitwise logical OR
    Or,
    /// Bitwise exclusive OR
    Xor,
    /// Bitwise logical NOT
    Not,
    /// Left shift with carry propagation
    Shl,
    /// Right shift with carry propagation
    Shr,

    // Comparison Operations (6 instructions)
    /// Equality comparison
    Eq,
    /// Inequality comparison
    Neq,
    /// Less than comparison
    Lt,
    /// Greater than comparison
    Gt,
    /// Less than or equal comparison
    Lte,
    /// Greater than or equal comparison
    Gte,

    // Memory Operations (4 instructions)
    /// Load from memory address
    Load(Option<u32>),
    /// Store to memory address
    Store(Option<u32>),
    /// Secondary memory load operation
    Mload(Option<u32>),
    /// Secondary memory store operation
    Mstore(Option<u32>),

    // Stack Operations (4 instructions)
    /// Push immediate value to stack
    Push(u32),
    /// Pop value from stack
    Pop,
    /// Duplicate top stack element
    Dup,
    /// Swap top two stack elements
    Swap,

    // Control Flow (5 instructions)
    /// Unconditional jump
    Jmp(u32),
    /// Jump if zero
    Jz(u32),
    /// Jump if not zero
    Jnz(u32),
    /// Function call
    Call(u32),
    /// Function return
    Ret,

    // Cryptographic Operations (3 instructions)
    /// Cryptographic hash computation
    Hash,
    /// Digital signature verification
    Verify,
    /// Digital signature generation
    Sign,

    // System Operations (5 instructions)
    /// Halt execution
    Halt,
    /// No operation
    Nop,
    /// Debug output
    Debug,
    /// Assert condition
    Assert,
    /// Log value
    Log,

    // I/O Operations (5 instructions)
    /// Read from input
    Read,
    /// Write to output
    Write,
    /// Network send
    Send,
    /// Network receive
    Recv,

    // Utility Operations (3 instructions)
    /// Get timestamp
    Time,
    /// Generate random number
    Rand,
    /// Generate unique identifier
    Id,
}

impl Instruction {
    /// Get the instruction category
    pub fn category(&self) -> InstructionCategory {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod => {
                InstructionCategory::Arithmetic
            }
            
            Self::And | Self::Or | Self::Xor | Self::Not | Self::Shl | Self::Shr => {
                InstructionCategory::Bitwise
            }
            
            Self::Eq | Self::Neq | Self::Lt | Self::Gt | Self::Lte | Self::Gte => {
                InstructionCategory::Comparison
            }
            
            Self::Load(_) | Self::Store(_) | Self::Mload(_) | Self::Mstore(_) => {
                InstructionCategory::Memory
            }
            
            Self::Push(_) | Self::Pop | Self::Dup | Self::Swap => {
                InstructionCategory::Stack
            }
            
            Self::Jmp(_) | Self::Jz(_) | Self::Jnz(_) | Self::Call(_) | Self::Ret => {
                InstructionCategory::ControlFlow
            }
            
            Self::Hash | Self::Verify | Self::Sign => {
                InstructionCategory::Cryptographic
            }
            
            Self::Halt | Self::Nop | Self::Debug | Self::Assert | Self::Log => {
                InstructionCategory::System
            }
            
            Self::Read | Self::Write | Self::Send | Self::Recv => {
                InstructionCategory::IO
            }
            
            Self::Time | Self::Rand | Self::Id => {
                InstructionCategory::Utility
            }
        }
    }
    
    /// Get the number of stack inputs required
    pub fn stack_inputs(&self) -> usize {
        match self {
            // No stack inputs
            Self::Push(_) | Self::Halt | Self::Nop | Self::Read | Self::Recv | 
            Self::Time | Self::Rand | Self::Id => 0,
            
            // One stack input
            Self::Not | Self::Pop | Self::Dup | Self::Debug | Self::Assert | 
            Self::Log | Self::Write | Self::Send => 1,
            
            // Two stack inputs
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod |
            Self::And | Self::Or | Self::Xor | Self::Shl | Self::Shr |
            Self::Eq | Self::Neq | Self::Lt | Self::Gt | Self::Lte | Self::Gte |
            Self::Swap => 2,
            
            // Three stack inputs
            Self::Verify => 3,
            
            // Variable inputs (depends on arguments and stack)
            Self::Load(addr) | Self::Mload(addr) => if addr.is_some() { 0 } else { 1 },
            Self::Store(addr) | Self::Mstore(addr) => if addr.is_some() { 1 } else { 2 },
            
            // Control flow inputs
            Self::Jmp(_) | Self::Call(_) => 0,
            Self::Jz(_) | Self::Jnz(_) => 1,
            Self::Ret => 1,
            
            // Crypto inputs
            Self::Hash => 1,
            Self::Sign => 2,
        }
    }
    
    /// Get the number of stack outputs produced
    pub fn stack_outputs(&self) -> usize {
        match self {
            // No stack outputs
            Self::Pop | Self::Halt | Self::Debug | Self::Assert | Self::Log | 
            Self::Write | Self::Send => 0,
            
            // Control flow (variable outputs)
            Self::Jmp(_) | Self::Jz(_) | Self::Jnz(_) | Self::Call(_) | Self::Ret => 0,
            
            // Store operations
            Self::Store(_) | Self::Mstore(_) => 0,
            
            // One stack output (most operations)
            _ => 1,
        }
    }
    
    /// Check if instruction modifies program counter
    pub fn modifies_pc(&self) -> bool {
        matches!(self, 
            Self::Jmp(_) | Self::Jz(_) | Self::Jnz(_) | 
            Self::Call(_) | Self::Ret | Self::Halt
        )
    }
    
    /// Check if instruction accesses memory
    pub fn accesses_memory(&self) -> bool {
        matches!(self, 
            Self::Load(_) | Self::Store(_) | 
            Self::Mload(_) | Self::Mstore(_)
        )
    }
    
    /// Check if instruction is deterministic
    pub fn is_deterministic(&self) -> bool {
        !matches!(self, Self::Rand | Self::Time | Self::Id | Self::Recv)
    }
    
    /// Get the mnemonic string representation
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Mod => "mod",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Not => "not",
            Self::Shl => "shl",
            Self::Shr => "shr",
            Self::Eq => "eq",
            Self::Neq => "neq",
            Self::Lt => "lt",
            Self::Gt => "gt",
            Self::Lte => "lte",
            Self::Gte => "gte",
            Self::Load(_) => "load",
            Self::Store(_) => "store",
            Self::Mload(_) => "mload",
            Self::Mstore(_) => "mstore",
            Self::Push(_) => "push",
            Self::Pop => "pop",
            Self::Dup => "dup",
            Self::Swap => "swap",
            Self::Jmp(_) => "jmp",
            Self::Jz(_) => "jz",
            Self::Jnz(_) => "jnz",
            Self::Call(_) => "call",
            Self::Ret => "ret",
            Self::Hash => "hash",
            Self::Verify => "verify",
            Self::Sign => "sign",
            Self::Halt => "halt",
            Self::Nop => "nop",
            Self::Debug => "debug",
            Self::Assert => "assert",
            Self::Log => "log",
            Self::Read => "read",
            Self::Write => "write",
            Self::Send => "send",
            Self::Recv => "recv",
            Self::Time => "time",
            Self::Rand => "rand",
            Self::Id => "id",
        }
    }
    
    /// Parse instruction from mnemonic and arguments
    pub fn parse(mnemonic: &str, args: &[u32]) -> Result<Self, String> {
        match mnemonic.to_lowercase().as_str() {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "mul" => Ok(Self::Mul),
            "div" => Ok(Self::Div),
            "mod" => Ok(Self::Mod),
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "xor" => Ok(Self::Xor),
            "not" => Ok(Self::Not),
            "shl" => Ok(Self::Shl),
            "shr" => Ok(Self::Shr),
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            "lt" => Ok(Self::Lt),
            "gt" => Ok(Self::Gt),
            "lte" => Ok(Self::Lte),
            "gte" => Ok(Self::Gte),
            "load" => Ok(Self::Load(args.get(0).copied())),
            "store" => Ok(Self::Store(args.get(0).copied())),
            "mload" => Ok(Self::Mload(args.get(0).copied())),
            "mstore" => Ok(Self::Mstore(args.get(0).copied())),
            "push" => {
                if args.is_empty() {
                    Err("PUSH requires an immediate value".to_string())
                } else {
                    Ok(Self::Push(args[0]))
                }
            }
            "pop" => Ok(Self::Pop),
            "dup" => Ok(Self::Dup),
            "swap" => Ok(Self::Swap),
            "jmp" => {
                if args.is_empty() {
                    Err("JMP requires a target address".to_string())
                } else {
                    Ok(Self::Jmp(args[0]))
                }
            }
            "jz" => {
                if args.is_empty() {
                    Err("JZ requires a target address".to_string())
                } else {
                    Ok(Self::Jz(args[0]))
                }
            }
            "jnz" => {
                if args.is_empty() {
                    Err("JNZ requires a target address".to_string())
                } else {
                    Ok(Self::Jnz(args[0]))
                }
            }
            "call" => {
                if args.is_empty() {
                    Err("CALL requires a target address".to_string())
                } else {
                    Ok(Self::Call(args[0]))
                }
            }
            "ret" => Ok(Self::Ret),
            "hash" => Ok(Self::Hash),
            "verify" => Ok(Self::Verify),
            "sign" => Ok(Self::Sign),
            "halt" => Ok(Self::Halt),
            "nop" => Ok(Self::Nop),
            "debug" => Ok(Self::Debug),
            "assert" => Ok(Self::Assert),
            "log" => Ok(Self::Log),
            "read" => Ok(Self::Read),
            "write" => Ok(Self::Write),
            "send" => Ok(Self::Send),
            "recv" => Ok(Self::Recv),
            "time" => Ok(Self::Time),
            "rand" => Ok(Self::Rand),
            "id" => Ok(Self::Id),
            _ => Err(format!("Unknown instruction: {}", mnemonic)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load(Some(addr)) => write!(f, "load {}", addr),
            Self::Store(Some(addr)) => write!(f, "store {}", addr),
            Self::Mload(Some(addr)) => write!(f, "mload {}", addr),
            Self::Mstore(Some(addr)) => write!(f, "mstore {}", addr),
            Self::Push(value) => write!(f, "push {}", value),
            Self::Jmp(target) => write!(f, "jmp {}", target),
            Self::Jz(target) => write!(f, "jz {}", target),
            Self::Jnz(target) => write!(f, "jnz {}", target),
            Self::Call(target) => write!(f, "call {}", target),
            _ => write!(f, "{}", self.mnemonic()),
        }
    }
}

/// Categories of instructions for analysis and optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstructionCategory {
    Arithmetic,
    Bitwise,
    Comparison,
    Memory,
    Stack,
    ControlFlow,
    Cryptographic,
    System,
    IO,
    Utility,
}

impl fmt::Display for InstructionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Arithmetic => "Arithmetic",
            Self::Bitwise => "Bitwise",
            Self::Comparison => "Comparison",
            Self::Memory => "Memory",
            Self::Stack => "Stack",
            Self::ControlFlow => "Control Flow",
            Self::Cryptographic => "Cryptographic",
            Self::System => "System",
            Self::IO => "I/O",
            Self::Utility => "Utility",
        };
        write!(f, "{}", name)
    }
}

/// Instruction complexity metrics for performance analysis
#[derive(Debug, Clone, Copy)]
pub struct InstructionComplexity {
    pub constraint_count: u32,
    pub cycles: u32,
    pub memory_accesses: u32,
    pub stack_operations: u32,
}

impl Instruction {
    /// Get complexity metrics for this instruction
    pub fn complexity(&self) -> InstructionComplexity {
        match self.category() {
            InstructionCategory::Arithmetic => InstructionComplexity {
                constraint_count: 200,  // 32-bit arithmetic
                cycles: 1,
                memory_accesses: 0,
                stack_operations: 2,
            },
            
            InstructionCategory::Bitwise => InstructionComplexity {
                constraint_count: 64,   // Simpler bitwise ops
                cycles: 1,
                memory_accesses: 0,
                stack_operations: 2,
            },
            
            InstructionCategory::Comparison => InstructionComplexity {
                constraint_count: 120,  // Comparison logic
                cycles: 1,
                memory_accesses: 0,
                stack_operations: 2,
            },
            
            InstructionCategory::Memory => InstructionComplexity {
                constraint_count: 96,   // Memory validation
                cycles: 2,              // Memory access penalty
                memory_accesses: 1,
                stack_operations: 1,
            },
            
            InstructionCategory::Stack => InstructionComplexity {
                constraint_count: 50,   // Simple stack ops
                cycles: 1,
                memory_accesses: 0,
                stack_operations: (self.stack_inputs() + self.stack_outputs()) as u32,
            },
            
            InstructionCategory::ControlFlow => InstructionComplexity {
                constraint_count: 80,   // Control flow logic
                cycles: 1,
                memory_accesses: 0,
                stack_operations: self.stack_inputs() as u32,
            },
            
            InstructionCategory::Cryptographic => InstructionComplexity {
                constraint_count: 280,  // Complex crypto ops
                cycles: 3,              // Crypto penalty
                memory_accesses: 0,
                stack_operations: (self.stack_inputs() + self.stack_outputs()) as u32,
            },
            
            InstructionCategory::System => InstructionComplexity {
                constraint_count: 30,   // Simple system ops
                cycles: 1,
                memory_accesses: 0,
                stack_operations: self.stack_inputs() as u32,
            },
            
            InstructionCategory::IO => InstructionComplexity {
                constraint_count: 40,   // I/O operations
                cycles: 2,              // I/O penalty
                memory_accesses: 0,
                stack_operations: (self.stack_inputs() + self.stack_outputs()) as u32,
            },
            
            InstructionCategory::Utility => InstructionComplexity {
                constraint_count: 20,   // Simple utilities
                cycles: 1,
                memory_accesses: 0,
                stack_operations: self.stack_outputs() as u32,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parsing() {
        assert_eq!(
            Instruction::parse("add", &[]).unwrap(),
            Instruction::Add
        );
        
        assert_eq!(
            Instruction::parse("push", &[42]).unwrap(),
            Instruction::Push(42)
        );
        
        assert_eq!(
            Instruction::parse("jmp", &[100]).unwrap(),
            Instruction::Jmp(100)
        );
        
        assert!(Instruction::parse("invalid", &[]).is_err());
        assert!(Instruction::parse("push", &[]).is_err());
    }
    
    #[test]
    fn test_instruction_properties() {
        assert_eq!(Instruction::Add.stack_inputs(), 2);
        assert_eq!(Instruction::Add.stack_outputs(), 1);
        assert!(!Instruction::Add.modifies_pc());
        assert!(!Instruction::Add.accesses_memory());
        assert!(Instruction::Add.is_deterministic());
        
        assert_eq!(Instruction::Load(Some(100)).stack_inputs(), 0);
        assert_eq!(Instruction::Load(None).stack_inputs(), 1);
        assert!(Instruction::Load(Some(100)).accesses_memory());
        
        assert!(Instruction::Jmp(0).modifies_pc());
        assert!(!Instruction::Rand.is_deterministic());
    }
    
    #[test]
    fn test_instruction_categories() {
        assert_eq!(Instruction::Add.category(), InstructionCategory::Arithmetic);
        assert_eq!(Instruction::And.category(), InstructionCategory::Bitwise);
        assert_eq!(Instruction::Eq.category(), InstructionCategory::Comparison);
        assert_eq!(Instruction::Load(None).category(), InstructionCategory::Memory);
        assert_eq!(Instruction::Push(0).category(), InstructionCategory::Stack);
        assert_eq!(Instruction::Jmp(0).category(), InstructionCategory::ControlFlow);
        assert_eq!(Instruction::Hash.category(), InstructionCategory::Cryptographic);
        assert_eq!(Instruction::Halt.category(), InstructionCategory::System);
        assert_eq!(Instruction::Read.category(), InstructionCategory::IO);
        assert_eq!(Instruction::Time.category(), InstructionCategory::Utility);
    }
    
    #[test]
    fn test_instruction_display() {
        assert_eq!(Instruction::Add.to_string(), "add");
        assert_eq!(Instruction::Push(42).to_string(), "push 42");
        assert_eq!(Instruction::Jmp(100).to_string(), "jmp 100");
        assert_eq!(Instruction::Load(Some(200)).to_string(), "load 200");
        assert_eq!(Instruction::Load(None).to_string(), "load");
    }
    
    #[test]
    fn test_instruction_complexity() {
        let add_complexity = Instruction::Add.complexity();
        assert_eq!(add_complexity.constraint_count, 200);
        assert_eq!(add_complexity.cycles, 1);
        
        let crypto_complexity = Instruction::Hash.complexity();
        assert!(crypto_complexity.constraint_count > add_complexity.constraint_count);
        assert!(crypto_complexity.cycles >= add_complexity.cycles);
    }
}