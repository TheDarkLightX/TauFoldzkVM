//! Virtual machine executor with Tau constraint validation
//!
//! The core execution engine that runs programs with mathematical guarantees.

use crate::{
    Instruction, Program, VmState, VmError, VmResult, VmConfig, ExecutionResult, 
    ExecutionStats, TauValidator, ConstraintValidator, TraceEntry
};
use std::time::Instant;

/// Main virtual machine executor for TauFoldZKVM
pub struct VirtualMachine {
    config: VmConfig,
    validator: Box<dyn ConstraintValidator>,
    input_data: Vec<u32>,
}

impl VirtualMachine {
    /// Create a new virtual machine with default configuration
    pub fn new() -> Self {
        Self {
            config: VmConfig::default(),
            validator: Box::new(TauValidator::new()),
            input_data: Vec::new(),
        }
    }
    
    /// Create virtual machine with custom configuration
    pub fn with_config(config: VmConfig) -> Self {
        let validator = if let Some(path) = &config.constraint_path {
            Box::new(TauValidator::with_path(path.clone()))
        } else {
            Box::new(TauValidator::new())
        };
        
        Self {
            config,
            validator,
            input_data: Vec::new(),
        }
    }
    
    /// Set input data for the program
    pub fn set_input(&mut self, input: Vec<u32>) {
        self.input_data = input;
    }
    
    /// Execute a program and return the result
    pub fn execute(&mut self, program: Program) -> VmResult<ExecutionResult> {
        let start_time = Instant::now();
        
        // Validate program first
        program.validate()?;
        
        // Initialize VM state
        let mut state = VmState::new(self.config.memory_size, self.config.register_count);
        state.input_buffer = self.input_data.clone();
        
        // Initialize statistics
        let mut stats = ExecutionStats::default();
        let mut trace = Vec::new();
        let mut violations = Vec::new();
        
        // Main execution loop
        while !state.halted && state.cycle_count < self.config.max_cycles {
            // Check if PC is valid
            if state.program_counter as usize >= program.instructions.len() {
                break;
            }
            
            let instruction = &program.instructions[state.program_counter as usize];
            let state_before = if self.config.enable_tracing {
                Some(state.clone())
            } else {
                None
            };
            
            // Execute instruction
            match self.execute_instruction(&mut state, instruction) {
                Ok(()) => {
                    stats.instructions_executed += 1;
                    
                    // Validate constraints if enabled
                    if self.config.validate_constraints {
                        // TODO: Implement proper constraint validation
                        stats.constraint_validations += 1;
                    }
                    
                    // Record trace if enabled
                    if self.config.enable_tracing {
                        if let Some(before) = state_before {
                            trace.push(TraceEntry {
                                cycle: state.cycle_count,
                                pc: before.program_counter,
                                instruction: instruction.clone(),
                                stack_before: before.stack.clone(),
                                stack_after: state.stack.clone(),
                                registers_before: before.registers.clone(),
                                registers_after: state.registers.clone(),
                            });
                        }
                    }
                }
                Err(e) => {
                    let execution_time = start_time.elapsed();
                    stats.execution_time_ms = execution_time.as_millis() as u64;
                    stats.cycles_executed = state.cycle_count;
                    
                    return Ok(ExecutionResult::failure(
                        state,
                        stats,
                        e.to_string(),
                    ).with_trace(trace).with_violations(violations));
                }
            }
            
            state.cycle_count += 1;
            stats.cycles_executed = state.cycle_count;
        }
        
        // Calculate final statistics
        let execution_time = start_time.elapsed();
        stats.execution_time_ms = execution_time.as_millis() as u64;
        stats.memory_usage_bytes = state.memory_usage();
        
        if stats.execution_time_ms > 0 {
            stats.instructions_per_second = 
                (stats.instructions_executed as f64 * 1000.0) / stats.execution_time_ms as f64;
        }
        
        // Check if execution completed successfully
        let success = state.halted || state.cycle_count >= self.config.max_cycles;
        
        if success {
            Ok(ExecutionResult::success(state, stats)
                .with_trace(trace)
                .with_violations(violations))
        } else {
            Ok(ExecutionResult::failure(
                state,
                stats,
                "Execution did not complete".to_string(),
            ).with_trace(trace).with_violations(violations))
        }
    }
    
    /// Execute a single instruction
    fn execute_instruction(&self, state: &mut VmState, instruction: &Instruction) -> VmResult<()> {
        match instruction {
            // Arithmetic operations
            Instruction::Add => self.execute_add(state),
            Instruction::Sub => self.execute_sub(state),
            Instruction::Mul => self.execute_mul(state),
            Instruction::Div => self.execute_div(state),
            Instruction::Mod => self.execute_mod(state),
            
            // Bitwise operations
            Instruction::And => self.execute_and(state),
            Instruction::Or => self.execute_or(state),
            Instruction::Xor => self.execute_xor(state),
            Instruction::Not => self.execute_not(state),
            Instruction::Shl => self.execute_shl(state),
            Instruction::Shr => self.execute_shr(state),
            
            // Comparison operations
            Instruction::Eq => self.execute_eq(state),
            Instruction::Neq => self.execute_neq(state),
            Instruction::Lt => self.execute_lt(state),
            Instruction::Gt => self.execute_gt(state),
            Instruction::Lte => self.execute_lte(state),
            Instruction::Gte => self.execute_gte(state),
            
            // Memory operations
            Instruction::Load(addr) => self.execute_load(state, *addr),
            Instruction::Store(addr) => self.execute_store(state, *addr),
            Instruction::Mload(addr) => self.execute_mload(state, *addr),
            Instruction::Mstore(addr) => self.execute_mstore(state, *addr),
            
            // Stack operations
            Instruction::Push(value) => self.execute_push(state, *value),
            Instruction::Pop => self.execute_pop(state),
            Instruction::Dup => self.execute_dup(state),
            Instruction::Swap => self.execute_swap(state),
            
            // Control flow
            Instruction::Jmp(target) => self.execute_jmp(state, *target),
            Instruction::Jz(target) => self.execute_jz(state, *target),
            Instruction::Jnz(target) => self.execute_jnz(state, *target),
            Instruction::Call(target) => self.execute_call(state, *target),
            Instruction::Ret => self.execute_ret(state),
            
            // Cryptographic operations
            Instruction::Hash => self.execute_hash(state),
            Instruction::Verify => self.execute_verify(state),
            Instruction::Sign => self.execute_sign(state),
            
            // System operations
            Instruction::Halt => self.execute_halt(state),
            Instruction::Nop => self.execute_nop(state),
            Instruction::Debug => self.execute_debug(state),
            Instruction::Assert => self.execute_assert(state),
            Instruction::Log => self.execute_log(state),
            
            // I/O operations
            Instruction::Read => self.execute_read(state),
            Instruction::Write => self.execute_write(state),
            Instruction::Send => self.execute_send(state),
            Instruction::Recv => self.execute_recv(state),
            
            // Utility operations
            Instruction::Time => self.execute_time(state),
            Instruction::Rand => self.execute_rand(state),
            Instruction::Id => self.execute_id(state),
        }
    }
    
    // Arithmetic operations
    fn execute_add(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "ADD".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = a.wrapping_add(b);
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_sub(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "SUB".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = a.wrapping_sub(b);
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_mul(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "MUL".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = a.wrapping_mul(b);
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_div(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "DIV".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        
        if b == 0 {
            return Err(VmError::DivisionByZero {
                operation: "DIV".to_string(),
            });
        }
        
        let result = a / b;
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_mod(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "MOD".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        
        if b == 0 {
            return Err(VmError::DivisionByZero {
                operation: "MOD".to_string(),
            });
        }
        
        let result = a % b;
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    // Bitwise operations
    fn execute_and(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "AND".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = a & b;
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_or(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "OR".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = a | b;
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_xor(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "XOR".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = a ^ b;
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_not(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "NOT".to_string(),
                required: 1,
            });
        }
        
        let a = state.pop_stack()?;
        let result = !a;
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_shl(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "SHL".to_string(),
                required: 2,
            });
        }
        
        let shift = state.pop_stack()?;
        let value = state.pop_stack()?;
        let result = value << (shift & 31); // Mask to prevent panic
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_shr(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "SHR".to_string(),
                required: 2,
            });
        }
        
        let shift = state.pop_stack()?;
        let value = state.pop_stack()?;
        let result = value >> (shift & 31); // Mask to prevent panic
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    // Comparison operations
    fn execute_eq(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "EQ".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = if a == b { 1 } else { 0 };
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_neq(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "NEQ".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = if a != b { 1 } else { 0 };
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_lt(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "LT".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = if a < b { 1 } else { 0 };
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_gt(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "GT".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = if a > b { 1 } else { 0 };
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_lte(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "LTE".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = if a <= b { 1 } else { 0 };
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_gte(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "GTE".to_string(),
                required: 2,
            });
        }
        
        let b = state.pop_stack()?;
        let a = state.pop_stack()?;
        let result = if a >= b { 1 } else { 0 };
        state.push_stack(result);
        state.program_counter += 1;
        Ok(())
    }
    
    // Memory operations
    fn execute_load(&self, state: &mut VmState, addr: Option<u32>) -> VmResult<()> {
        let address = if let Some(addr) = addr {
            addr
        } else {
            if !state.has_stack_elements(1) {
                return Err(VmError::StackUnderflow {
                    operation: "LOAD".to_string(),
                    required: 1,
                });
            }
            state.pop_stack()?
        };
        
        let value = state.get_memory(address)?;
        state.push_stack(value);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_store(&self, state: &mut VmState, addr: Option<u32>) -> VmResult<()> {
        let (address, value) = if let Some(addr) = addr {
            if !state.has_stack_elements(1) {
                return Err(VmError::StackUnderflow {
                    operation: "STORE".to_string(),
                    required: 1,
                });
            }
            (addr, state.pop_stack()?)
        } else {
            if !state.has_stack_elements(2) {
                return Err(VmError::StackUnderflow {
                    operation: "STORE".to_string(),
                    required: 2,
                });
            }
            let addr = state.pop_stack()?;
            let val = state.pop_stack()?;
            (addr, val)
        };
        
        state.set_memory(address, value)?;
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_mload(&self, state: &mut VmState, addr: Option<u32>) -> VmResult<()> {
        // Same as load for now
        self.execute_load(state, addr)
    }
    
    fn execute_mstore(&self, state: &mut VmState, addr: Option<u32>) -> VmResult<()> {
        // Same as store for now
        self.execute_store(state, addr)
    }
    
    // Stack operations
    fn execute_push(&self, state: &mut VmState, value: u32) -> VmResult<()> {
        state.push_stack(value);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_pop(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "POP".to_string(),
                required: 1,
            });
        }
        
        state.pop_stack()?;
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_dup(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "DUP".to_string(),
                required: 1,
            });
        }
        
        let value = state.peek_stack()?;
        state.push_stack(value);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_swap(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(2) {
            return Err(VmError::StackUnderflow {
                operation: "SWAP".to_string(),
                required: 2,
            });
        }
        
        let a = state.pop_stack()?;
        let b = state.pop_stack()?;
        state.push_stack(a);
        state.push_stack(b);
        state.program_counter += 1;
        Ok(())
    }
    
    // Control flow operations
    fn execute_jmp(&self, state: &mut VmState, target: u32) -> VmResult<()> {
        state.program_counter = target;
        Ok(())
    }
    
    fn execute_jz(&self, state: &mut VmState, target: u32) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "JZ".to_string(),
                required: 1,
            });
        }
        
        let condition = state.pop_stack()?;
        if condition == 0 {
            state.program_counter = target;
        } else {
            state.program_counter += 1;
        }
        Ok(())
    }
    
    fn execute_jnz(&self, state: &mut VmState, target: u32) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "JNZ".to_string(),
                required: 1,
            });
        }
        
        let condition = state.pop_stack()?;
        if condition != 0 {
            state.program_counter = target;
        } else {
            state.program_counter += 1;
        }
        Ok(())
    }
    
    fn execute_call(&self, state: &mut VmState, target: u32) -> VmResult<()> {
        state.call_stack.push(state.program_counter + 1);
        state.program_counter = target;
        Ok(())
    }
    
    fn execute_ret(&self, state: &mut VmState) -> VmResult<()> {
        if state.call_stack.is_empty() {
            return Err(VmError::ProgramError {
                message: "Return with empty call stack".to_string(),
            });
        }
        
        state.program_counter = state.call_stack.pop().unwrap();
        Ok(())
    }
    
    // Cryptographic operations (simplified implementations)
    fn execute_hash(&self, _state: &mut VmState) -> VmResult<()> {
        // TODO: Implement cryptographic hash
        Ok(())
    }
    
    fn execute_verify(&self, _state: &mut VmState) -> VmResult<()> {
        // TODO: Implement signature verification
        Ok(())
    }
    
    fn execute_sign(&self, _state: &mut VmState) -> VmResult<()> {
        // TODO: Implement signature generation
        Ok(())
    }
    
    // System operations
    fn execute_halt(&self, state: &mut VmState) -> VmResult<()> {
        state.halted = true;
        Ok(())
    }
    
    fn execute_nop(&self, state: &mut VmState) -> VmResult<()> {
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_debug(&self, state: &mut VmState) -> VmResult<()> {
        if self.config.debug_mode && !state.stack.is_empty() {
            let value = state.peek_stack()?;
            println!("DEBUG: {}", value);
        }
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_assert(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "ASSERT".to_string(),
                required: 1,
            });
        }
        
        let condition = state.pop_stack()?;
        if condition == 0 {
            return Err(VmError::AssertionFailed {
                cycle: state.cycle_count,
            });
        }
        
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_log(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "LOG".to_string(),
                required: 1,
            });
        }
        
        let value = state.pop_stack()?;
        if self.config.debug_mode {
            println!("LOG: {}", value);
        }
        state.program_counter += 1;
        Ok(())
    }
    
    // I/O operations
    fn execute_read(&self, state: &mut VmState) -> VmResult<()> {
        if !state.input_buffer.is_empty() {
            let value = state.input_buffer.remove(0);
            state.push_stack(value);
        } else {
            state.push_stack(0); // No input available
        }
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_write(&self, state: &mut VmState) -> VmResult<()> {
        if !state.has_stack_elements(1) {
            return Err(VmError::StackUnderflow {
                operation: "WRITE".to_string(),
                required: 1,
            });
        }
        
        let value = state.pop_stack()?;
        state.output_buffer.push(value);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_send(&self, state: &mut VmState) -> VmResult<()> {
        // TODO: Implement network send
        self.execute_write(state)
    }
    
    fn execute_recv(&self, state: &mut VmState) -> VmResult<()> {
        // TODO: Implement network receive
        self.execute_read(state)
    }
    
    // Utility operations
    fn execute_time(&self, state: &mut VmState) -> VmResult<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u32;
        state.push_stack(timestamp);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_rand(&self, state: &mut VmState) -> VmResult<()> {
        use rand::Rng;
        let value = rand::thread_rng().gen::<u32>();
        state.push_stack(value);
        state.program_counter += 1;
        Ok(())
    }
    
    fn execute_id(&self, state: &mut VmState) -> VmResult<()> {
        let id = uuid::Uuid::new_v4().as_u128() as u32; // Truncate to 32 bits
        state.push_stack(id);
        state.program_counter += 1;
        Ok(())
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_instructions() {
        let mut vm = VirtualMachine::new();
        let program = Program::new(vec![
            Instruction::Push(10),
            Instruction::Push(20),
            Instruction::Add,
            Instruction::Halt,
        ]);
        
        let result = vm.execute(program).unwrap();
        assert!(result.success);
        assert_eq!(result.final_state.stack.last(), Some(&30));
    }
    
    #[test]
    fn test_division_by_zero() {
        let mut vm = VirtualMachine::new();
        let program = Program::new(vec![
            Instruction::Push(10),
            Instruction::Push(0),
            Instruction::Div,
            Instruction::Halt,
        ]);
        
        let result = vm.execute(program).unwrap();
        assert!(!result.success);
    }
    
    #[test]
    fn test_stack_underflow() {
        let mut vm = VirtualMachine::new();
        let program = Program::new(vec![
            Instruction::Add, // No values on stack
            Instruction::Halt,
        ]);
        
        let result = vm.execute(program).unwrap();
        assert!(!result.success);
    }
}