use anyhow::{Result, Context};
use std::process::{Command, Stdio};
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use serde::Deserialize;

pub struct ZkVMRunner {
    pub program_path: String,
    pub tau_binary: String,
}

impl ZkVMRunner {
    pub fn new(program_path: &str) -> Self {
        Self {
            program_path: program_path.to_string(),
            tau_binary: std::env::var("TAU_BINARY").unwrap_or_else(|_| "tau".to_string()),
        }
    }

    pub async fn execute(&self, input: Vec<u32>) -> Result<ZkVMResult> {
        let start = Instant::now();
        
        // Check if program file exists
        if !Path::new(&self.program_path).exists() {
            anyhow::bail!("zkVM program not found: {}", self.program_path);
        }
        
        // Prepare input data
        let input_json = serde_json::to_string(&input)?;
        
        // Execute Tau program
        let mut child = Command::new(&self.tau_binary)
            .arg("run")
            .arg(&self.program_path)
            .arg("--prove")
            .arg("--json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to execute Tau runtime")?;
        
        // Send input
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input_json.as_bytes())?;
        }
        
        // Wait for completion
        let output = child.wait_with_output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Tau execution failed: {}", stderr);
        }
        
        // Parse output
        let stdout = String::from_utf8_lossy(&output.stdout);
        let execution_data: TauExecutionOutput = serde_json::from_str(&stdout)
            .context("Failed to parse Tau output")?;
        
        let execution_time = start.elapsed();
        
        Ok(ZkVMResult {
            output: execution_data.result,
            cycles: execution_data.cycles,
            constraints_generated: execution_data.constraints,
            folding_steps: execution_data.folding_steps,
            proof_size: execution_data.proof_size,
            verification_time_ms: execution_time.as_millis() as u64,
            trace_log: execution_data.trace_log,
        })
    }
    
    pub async fn verify_proof(&self, proof_path: &str) -> Result<bool> {
        let output = Command::new(&self.tau_binary)
            .arg("verify")
            .arg(proof_path)
            .output()
            .context("Failed to verify proof")?;
            
        Ok(output.status.success())
    }
}

#[derive(Debug, Deserialize)]
struct TauExecutionOutput {
    result: Vec<u32>,
    cycles: u64,
    constraints: u64,
    folding_steps: u64,
    proof_size: usize,
    trace_log: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ZkVMResult {
    pub output: Vec<u32>,
    pub cycles: u64,
    pub constraints_generated: u64,
    pub folding_steps: u64,
    pub proof_size: usize,
    pub verification_time_ms: u64,
    pub trace_log: Vec<String>,
}