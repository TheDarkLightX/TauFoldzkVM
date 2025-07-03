//! Advanced debugger functionality for TauFoldZKVM

use anyhow::Result;
use std::collections::HashMap;
use taufold_zkvm::{Instruction, Program, VmError, VmState};

pub struct DebuggerState {
    pub step_count: u64,
    pub breakpoint_hits: HashMap<u32, usize>,
    pub instruction_count: HashMap<String, usize>,
    pub state_snapshots: Vec<VmStateSnapshot>,
    pub watch_values: HashMap<String, WatchValue>,
}

#[derive(Clone)]
pub struct VmStateSnapshot {
    pub cycle: u64,
    pub pc: u32,
    pub stack: Vec<u32>,
    pub registers: Vec<u32>,
    pub memory_checksum: u64,
}

#[derive(Clone)]
pub enum WatchValue {
    Stack(usize),
    Register(usize),
    Memory(u32),
    Expression(String),
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            step_count: 0,
            breakpoint_hits: HashMap::new(),
            instruction_count: HashMap::new(),
            state_snapshots: Vec::new(),
            watch_values: HashMap::new(),
        }
    }
    
    pub fn step(&mut self, state: &mut VmState, program: &Program) -> Result<()> {
        if state.halted {
            return Ok(());
        }
        
        // Check if PC is valid
        if state.program_counter as usize >= program.instructions.len() {
            state.halted = true;
            return Ok(());
        }
        
        // Take snapshot before execution
        self.take_snapshot(state);
        
        // Get current instruction
        let instruction = &program.instructions[state.program_counter as usize];
        
        // Update instruction count
        *self.instruction_count
            .entry(instruction.mnemonic().to_string())
            .or_insert(0) += 1;
        
        // Execute instruction
        self.execute_instruction(state, instruction)?;
        
        // Update step count
        self.step_count += 1;
        state.cycle_count += 1;
        
        // Update watch values
        self.update_watch_values(state);
        
        Ok(())
    }
    
    fn take_snapshot(&mut self, state: &VmState) {
        let snapshot = VmStateSnapshot {
            cycle: state.cycle_count,
            pc: state.program_counter,
            stack: state.stack.clone(),
            registers: state.registers.clone(),
            memory_checksum: self.calculate_memory_checksum(&state.memory),
        };
        
        self.state_snapshots.push(snapshot);
        
        // Keep only last 100 snapshots
        if self.state_snapshots.len() > 100 {
            self.state_snapshots.remove(0);
        }
    }
    
    fn calculate_memory_checksum(&self, memory: &[u32]) -> u64 {
        memory.iter()
            .enumerate()
            .map(|(i, &val)| (i as u64 + 1) * val as u64)
            .sum()
    }
    
    fn update_watch_values(&mut self, state: &VmState) {
        // Update watch expressions with current values
        // This is a placeholder - real implementation would evaluate expressions
    }
    
    fn execute_instruction(&self, state: &mut VmState, instruction: &Instruction) -> Result<()> {
        // This is a simplified version - real implementation would use the VM executor
        match instruction {
            Instruction::Push(value) => {
                state.push_stack(*value);
                state.program_counter += 1;
            }
            Instruction::Add => {
                if state.stack.len() >= 2 {
                    let b = state.pop_stack().unwrap();
                    let a = state.pop_stack().unwrap();
                    state.push_stack(a.wrapping_add(b));
                }
                state.program_counter += 1;
            }
            Instruction::Halt => {
                state.halted = true;
            }
            // ... implement other instructions as needed
            _ => {
                state.program_counter += 1;
            }
        }
        
        Ok(())
    }
    
    pub fn add_watch(&mut self, name: String, value: WatchValue) {
        self.watch_values.insert(name, value);
    }
    
    pub fn remove_watch(&mut self, name: &str) {
        self.watch_values.remove(name);
    }
    
    pub fn get_instruction_stats(&self) -> Vec<(String, usize, f64)> {
        let total = self.instruction_count.values().sum::<usize>() as f64;
        
        let mut stats: Vec<_> = self.instruction_count
            .iter()
            .map(|(inst, &count)| {
                let percentage = (count as f64 / total) * 100.0;
                (inst.clone(), count, percentage)
            })
            .collect();
        
        stats.sort_by_key(|(_, count, _)| std::cmp::Reverse(*count));
        stats
    }
    
    pub fn rewind_to_snapshot(&mut self, state: &mut VmState, index: usize) -> Result<()> {
        if index < self.state_snapshots.len() {
            let snapshot = &self.state_snapshots[index];
            state.cycle_count = snapshot.cycle;
            state.program_counter = snapshot.pc;
            state.stack = snapshot.stack.clone();
            state.registers = snapshot.registers.clone();
            // Note: Full memory restore would be needed for complete rewind
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid snapshot index"))
        }
    }
}