//! Program executor with advanced features

use anyhow::Result;
use std::time::{Duration, Instant};
use taufold_zkvm::{Program, VirtualMachine, VmConfig};

pub struct ProgramExecutor {
    pub config: VmConfig,
    pub execution_stats: ExecutionStatistics,
}

#[derive(Default)]
pub struct ExecutionStatistics {
    pub total_executions: usize,
    pub successful_executions: usize,
    pub failed_executions: usize,
    pub average_cycles: f64,
    pub average_time_ms: f64,
    pub constraint_violation_rate: f64,
}

impl ProgramExecutor {
    pub fn new() -> Self {
        Self {
            config: VmConfig::default(),
            execution_stats: ExecutionStatistics::default(),
        }
    }
    
    pub fn execute(&mut self, program: Program) -> Result<taufold_zkvm::ExecutionResult> {
        let mut vm = VirtualMachine::with_config(self.config.clone());
        let start = Instant::now();
        
        let result = vm.execute(program)?;
        let elapsed = start.elapsed();
        
        // Update statistics
        self.update_stats(&result, elapsed);
        
        Ok(result)
    }
    
    pub fn execute_with_input(&mut self, program: Program, input: Vec<u32>) -> Result<taufold_zkvm::ExecutionResult> {
        let mut vm = VirtualMachine::with_config(self.config.clone());
        vm.set_input(input);
        
        let start = Instant::now();
        let result = vm.execute(program)?;
        let elapsed = start.elapsed();
        
        self.update_stats(&result, elapsed);
        
        Ok(result)
    }
    
    pub fn benchmark(&mut self, program: Program, iterations: usize) -> BenchmarkResult {
        let mut times = Vec::new();
        let mut cycles = Vec::new();
        let mut successes = 0;
        
        for _ in 0..iterations {
            let mut vm = VirtualMachine::with_config(self.config.clone());
            let start = Instant::now();
            
            if let Ok(result) = vm.execute(program.clone()) {
                let elapsed = start.elapsed();
                times.push(elapsed);
                cycles.push(result.stats.cycles_executed);
                
                if result.success {
                    successes += 1;
                }
            }
        }
        
        BenchmarkResult {
            iterations,
            successes,
            average_time: Self::calculate_average_duration(&times),
            min_time: times.iter().min().cloned().unwrap_or_default(),
            max_time: times.iter().max().cloned().unwrap_or_default(),
            average_cycles: cycles.iter().sum::<u64>() as f64 / cycles.len() as f64,
        }
    }
    
    fn update_stats(&mut self, result: &taufold_zkvm::ExecutionResult, elapsed: Duration) {
        self.execution_stats.total_executions += 1;
        
        if result.success {
            self.execution_stats.successful_executions += 1;
        } else {
            self.execution_stats.failed_executions += 1;
        }
        
        // Update averages
        let n = self.execution_stats.total_executions as f64;
        self.execution_stats.average_cycles = 
            (self.execution_stats.average_cycles * (n - 1.0) + result.stats.cycles_executed as f64) / n;
        
        self.execution_stats.average_time_ms = 
            (self.execution_stats.average_time_ms * (n - 1.0) + elapsed.as_millis() as f64) / n;
        
        if result.stats.constraint_validations > 0 {
            let violation_rate = result.stats.constraint_violations as f64 / 
                               result.stats.constraint_validations as f64;
            self.execution_stats.constraint_violation_rate = 
                (self.execution_stats.constraint_violation_rate * (n - 1.0) + violation_rate) / n;
        }
    }
    
    fn calculate_average_duration(times: &[Duration]) -> Duration {
        if times.is_empty() {
            return Duration::default();
        }
        
        let total: Duration = times.iter().sum();
        total / times.len() as u32
    }
}

pub struct BenchmarkResult {
    pub iterations: usize,
    pub successes: usize,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub average_cycles: f64,
}