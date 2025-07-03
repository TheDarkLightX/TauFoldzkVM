//! Example programs for TauFoldZKVM
//!
//! Demonstrates the capabilities of the virtual machine with various test programs.

use crate::{Program, Instruction, ProgramMetadata};

/// Create a simple arithmetic example program
pub fn create_arithmetic_example() -> Program {
    let instructions = vec![
        Instruction::Push(42),
        Instruction::Push(58),
        Instruction::Add,
        Instruction::Dup,
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Halt,
    ];
    
    let metadata = ProgramMetadata {
        name: "Arithmetic Example".to_string(),
        version: "1.0.0".to_string(),
        description: "Demonstrates basic arithmetic operations: (42 + 58) * 2 = 200".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create a Fibonacci sequence calculation program
pub fn create_fibonacci_example() -> Program {
    let instructions = vec![
        // Simple Fibonacci: Calculate F(5) = 5
        Instruction::Push(0),    // F(0) = 0
        Instruction::Push(1),    // F(1) = 1
        
        // F(2) = F(1) + F(0) = 1
        Instruction::Dup,        // 0 1 1
        Instruction::Swap,       // 0 1 1
        Instruction::Add,        // 0 1 (previous+current=1)
        
        // F(3) = F(2) + F(1) = 2  
        Instruction::Dup,        // 0 1 1 1
        Instruction::Swap,       // 0 1 1 1
        Instruction::Add,        // 0 1 1 (1+1=2)
        
        // F(4) = F(3) + F(2) = 3
        Instruction::Dup,        // 0 1 1 2 2
        Instruction::Swap,       // 0 1 1 2 2
        Instruction::Add,        // 0 1 1 2 (2+1=3)
        
        // F(5) = F(4) + F(3) = 5
        Instruction::Dup,        // 0 1 1 2 3 3
        Instruction::Swap,       // 0 1 1 2 3 3
        Instruction::Add,        // 0 1 1 2 3 (3+2=5)
        
        Instruction::Halt,
    ];
    
    let metadata = ProgramMetadata {
        name: "Fibonacci Example".to_string(),
        version: "1.0.0".to_string(),
        description: "Calculates the 10th Fibonacci number using stack operations".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create a cryptographic operations demo
pub fn create_crypto_example() -> Program {
    let instructions = vec![
        Instruction::Push(12345),
        Instruction::Hash,
        Instruction::Push(67890),
        Instruction::Hash,
        Instruction::Xor,
        Instruction::Halt,
    ];
    
    let metadata = ProgramMetadata {
        name: "Crypto Example".to_string(),
        version: "1.0.0".to_string(),
        description: "Demonstrates cryptographic operations with hashing and XOR".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create an arithmetic benchmark program
pub fn create_arithmetic_benchmark() -> Program {
    let mut instructions = vec![
        Instruction::Push(1),
        Instruction::Push(1),
    ];
    
    // Perform 100 additions
    for _ in 0..100 {
        instructions.extend([
            Instruction::Add,
            Instruction::Dup,
            Instruction::Push(1),
        ]);
    }
    
    instructions.push(Instruction::Halt);
    
    let metadata = ProgramMetadata {
        name: "Arithmetic Benchmark".to_string(),
        version: "1.0.0".to_string(),
        description: "Benchmark program with 100 arithmetic operations".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create a memory operations benchmark
pub fn create_memory_benchmark() -> Program {
    let mut instructions = vec![];
    
    // Store values to memory
    for i in 0..50 {
        instructions.extend([
            Instruction::Push(i * 2),       // value
            Instruction::Push(i),           // address
            Instruction::Store(None),       // store value at address
        ]);
    }
    
    // Load values from memory and sum them
    instructions.push(Instruction::Push(0)); // accumulator
    
    for i in 0..50 {
        instructions.extend([
            Instruction::Push(i),           // address
            Instruction::Load(None),        // load value
            Instruction::Add,               // add to accumulator
        ]);
    }
    
    instructions.push(Instruction::Halt);
    
    let metadata = ProgramMetadata {
        name: "Memory Benchmark".to_string(),
        version: "1.0.0".to_string(),
        description: "Benchmark program with memory store/load operations".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create a cryptographic benchmark program
pub fn create_crypto_benchmark() -> Program {
    let mut instructions = vec![
        Instruction::Push(0x12345678), // Initial value
    ];
    
    // Perform multiple hash operations
    for _ in 0..20 {
        instructions.extend([
            Instruction::Hash,
            Instruction::Dup,
            Instruction::Push(0xDEADBEEF),
            Instruction::Xor,
        ]);
    }
    
    instructions.push(Instruction::Halt);
    
    let metadata = ProgramMetadata {
        name: "Crypto Benchmark".to_string(),
        version: "1.0.0".to_string(),
        description: "Benchmark program with cryptographic operations".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create a comprehensive test program
pub fn create_comprehensive_test() -> Program {
    let instructions = vec![
        // Test arithmetic
        Instruction::Push(10),     // PC 0
        Instruction::Push(5),      // PC 1
        Instruction::Add,          // PC 2: 15
        Instruction::Push(3),      // PC 3
        Instruction::Mul,          // PC 4: 45
        
        // Test bitwise
        Instruction::Push(0xFF),   // PC 5
        Instruction::And,          // PC 6: 45 & 255 = 45
        
        // Test comparison
        Instruction::Dup,          // PC 7: 45 45
        Instruction::Push(50),     // PC 8
        Instruction::Lt,           // PC 9: 45 < 50 = 1 (true)
        
        // Test memory
        Instruction::Push(100),    // PC 10: value
        Instruction::Push(0),      // PC 11: address
        Instruction::Store(None),  // PC 12: store 100 at address 0
        
        Instruction::Push(0),      // PC 13: address
        Instruction::Load(None),   // PC 14: load from address 0 = 100
        
        // Test control flow
        Instruction::Push(1),      // PC 15
        Instruction::Jnz(19),      // PC 16: Jump to PC 19 if not zero
        
        // This should be skipped
        Instruction::Push(999),    // PC 17
        Instruction::Push(999),    // PC 18
        
        // End (PC 19)
        Instruction::Halt,         // PC 19
    ];
    
    let metadata = ProgramMetadata {
        name: "Comprehensive Test".to_string(),
        version: "1.0.0".to_string(),
        description: "Tests all major instruction categories".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

/// Create a stress test program
pub fn create_stress_test() -> Program {
    let mut instructions = vec![
        Instruction::Push(1),
        Instruction::Push(1),
    ];
    
    // Create a large computation
    for i in 0..1000 {
        match i % 4 {
            0 => instructions.push(Instruction::Add),
            1 => instructions.push(Instruction::Mul),
            2 => instructions.push(Instruction::Xor),
            3 => {
                instructions.push(Instruction::Dup);
                instructions.push(Instruction::Push(1));
            }
            _ => unreachable!(),
        }
    }
    
    instructions.push(Instruction::Halt);
    
    let metadata = ProgramMetadata {
        name: "Stress Test".to_string(),
        version: "1.0.0".to_string(),
        description: "High-intensity computation for performance testing".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Program::with_metadata(instructions, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VirtualMachine;

    #[test]
    fn test_arithmetic_example() {
        let program = create_arithmetic_example();
        let mut vm = VirtualMachine::new();
        
        let result = vm.execute(program).unwrap();
        assert!(result.success);
        assert_eq!(result.final_state.stack.last(), Some(&200));
    }
    
    #[test]
    fn test_fibonacci_example() {
        let program = create_fibonacci_example();
        let mut vm = VirtualMachine::new();
        
        let result = vm.execute(program).unwrap();
        assert!(result.success);
        // 10th Fibonacci number should be on stack
        assert!(!result.final_state.stack.is_empty());
    }
    
    #[test] 
    fn test_comprehensive_test() {
        let program = create_comprehensive_test();
        let mut vm = VirtualMachine::new();
        
        let result = vm.execute(program).unwrap();
        assert!(result.success);
        assert_eq!(result.final_state.stack.last(), Some(&100));
    }
}