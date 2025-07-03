//! TauFoldZKVM CLI
//!
//! Command-line interface for the TauFoldZKVM runtime.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use taufold_zkvm::{VirtualMachine, Program, VmConfig};

#[derive(Parser)]
#[command(name = "taufold-zkvm")]
#[command(about = "TauFoldZKVM: Production-ready zero-knowledge virtual machine")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a program
    Run {
        /// Program file to execute (JSON format)
        #[arg(short, long)]
        program: PathBuf,
        
        /// Enable constraint validation
        #[arg(short, long, default_value = "true")]
        validate: bool,
        
        /// Enable execution tracing
        #[arg(short, long)]
        trace: bool,
        
        /// Maximum execution cycles
        #[arg(short, long, default_value = "1000000")]
        max_cycles: u64,
        
        /// Input file (JSON array of numbers)
        #[arg(short, long)]
        input: Option<PathBuf>,
        
        /// Output file for results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Validate a program
    Validate {
        /// Program file to validate
        #[arg(short, long)]
        program: PathBuf,
    },
    
    /// Show program statistics
    Stats {
        /// Program file to analyze
        #[arg(short, long)]
        program: PathBuf,
    },
    
    /// Create example programs
    Examples {
        /// Output directory for examples
        #[arg(short, long, default_value = "examples")]
        output_dir: PathBuf,
    },
    
    /// Run benchmarks
    Benchmark {
        /// Benchmark to run
        #[arg(short, long, default_value = "all")]
        benchmark: String,
        
        /// Number of iterations
        #[arg(short, long, default_value = "10")]
        iterations: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { 
            program, 
            validate, 
            trace, 
            max_cycles, 
            input, 
            output 
        } => {
            run_program(program, validate, trace, max_cycles, input, output).await?;
        }
        
        Commands::Validate { program } => {
            validate_program(program).await?;
        }
        
        Commands::Stats { program } => {
            show_program_stats(program).await?;
        }
        
        Commands::Examples { output_dir } => {
            create_examples(output_dir).await?;
        }
        
        Commands::Benchmark { benchmark, iterations } => {
            run_benchmarks(&benchmark, iterations).await?;
        }
    }
    
    Ok(())
}

async fn run_program(
    program_path: PathBuf,
    validate: bool,
    trace: bool,
    max_cycles: u64,
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TauFoldZKVM Runtime v1.0.0");
    println!("Loading program: {}", program_path.display());
    
    // Load program
    let program_json = std::fs::read_to_string(&program_path)?;
    let program = Program::from_json(&program_json)?;
    
    // Validate program
    program.validate()?;
    println!("✅ Program validation passed");
    
    // Load input if provided
    let input = if let Some(input_path) = input_path {
        let input_json = std::fs::read_to_string(&input_path)?;
        serde_json::from_str::<Vec<u32>>(&input_json)?
    } else {
        Vec::new()
    };
    
    // Configure VM
    let config = VmConfig {
        max_cycles,
        validate_constraints: validate,
        enable_tracing: trace,
        debug_mode: true,
        ..Default::default()
    };
    
    // Create and run VM
    let mut vm = VirtualMachine::with_config(config);
    if !input.is_empty() {
        vm.set_input(input);
    }
    
    println!("🔄 Executing program...");
    let start_time = std::time::Instant::now();
    
    let result = vm.execute(program)?;
    
    let execution_time = start_time.elapsed();
    
    // Print results
    println!("\n📊 Execution Results:");
    println!("Success: {}", result.success);
    println!("Cycles: {}", result.stats.cycles_executed);
    println!("Instructions: {}", result.stats.instructions_executed);
    println!("Execution time: {:.2}ms", execution_time.as_millis());
    println!("Instructions/second: {:.2}", result.stats.instructions_per_second);
    
    if validate {
        println!("Constraint validations: {}", result.stats.constraint_validations);
        println!("Constraint violations: {}", result.stats.constraint_violations);
    }
    
    println!("\n📁 Final State:");
    println!("Stack: {:?}", result.final_state.stack);
    if !result.final_state.output_buffer.is_empty() {
        println!("Output: {:?}", result.final_state.output_buffer);
    }
    
    if result.has_violations() {
        println!("\n⚠️  Constraint Violations:");
        for violation in &result.constraint_violations {
            println!("  Cycle {}: {} - {}", violation.cycle, violation.instruction, violation.details);
        }
    }
    
    // Save output if requested
    if let Some(output_path) = output_path {
        let output_json = serde_json::to_string_pretty(&result)?;
        std::fs::write(&output_path, output_json)?;
        println!("💾 Results saved to: {}", output_path.display());
    }
    
    if result.success {
        println!("✅ Execution completed successfully!");
    } else {
        println!("❌ Execution failed: {}", result.error.unwrap_or_default());
        std::process::exit(1);
    }
    
    Ok(())
}

async fn validate_program(program_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Validating program: {}", program_path.display());
    
    let program_json = std::fs::read_to_string(&program_path)?;
    let program = Program::from_json(&program_json)?;
    
    match program.validate() {
        Ok(()) => {
            println!("✅ Program validation passed");
            println!("Instructions: {}", program.instructions.len());
            
            let stats = program.stats();
            println!("📊 Instruction breakdown:");
            println!("  Arithmetic: {}", stats.arithmetic_ops);
            println!("  Bitwise: {}", stats.bitwise_ops);
            println!("  Comparison: {}", stats.comparison_ops);
            println!("  Memory: {}", stats.memory_ops);
            println!("  Stack: {}", stats.stack_ops);
            println!("  Control Flow: {}", stats.control_flow_ops);
            println!("  Crypto: {}", stats.crypto_ops);
            println!("  Other: {}", stats.other_ops);
        }
        Err(e) => {
            println!("❌ Program validation failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn show_program_stats(program_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Analyzing program: {}", program_path.display());
    
    let program_json = std::fs::read_to_string(&program_path)?;
    let program = Program::from_json(&program_json)?;
    
    let stats = program.stats();
    println!("{}", stats);
    
    // Estimate execution complexity
    let mut total_constraints = 0;
    let mut total_cycles = 0;
    
    for instruction in &program.instructions {
        let complexity = instruction.complexity();
        total_constraints += complexity.constraint_count;
        total_cycles += complexity.cycles;
    }
    
    println!("\n🔢 Complexity Estimates:");
    println!("Total constraints: {}", total_constraints);
    println!("Estimated cycles: {}", total_cycles);
    println!("Constraint budget usage: {:.1}%", (total_constraints as f64 / 40000.0) * 100.0);
    
    Ok(())
}

async fn create_examples(output_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Creating example programs in: {}", output_dir.display());
    
    std::fs::create_dir_all(&output_dir)?;
    
    // Create simple arithmetic example
    let arithmetic_example = taufold_zkvm::examples::create_arithmetic_example();
    let arithmetic_json = arithmetic_example.to_json()?;
    std::fs::write(output_dir.join("arithmetic.json"), arithmetic_json)?;
    
    // Create Fibonacci example
    let fibonacci_example = taufold_zkvm::examples::create_fibonacci_example();
    let fibonacci_json = fibonacci_example.to_json()?;
    std::fs::write(output_dir.join("fibonacci.json"), fibonacci_json)?;
    
    // Create cryptographic example
    let crypto_example = taufold_zkvm::examples::create_crypto_example();
    let crypto_json = crypto_example.to_json()?;
    std::fs::write(output_dir.join("crypto.json"), crypto_json)?;
    
    println!("✅ Created example programs:");
    println!("  - arithmetic.json: Basic arithmetic operations");
    println!("  - fibonacci.json: Fibonacci sequence calculation");
    println!("  - crypto.json: Cryptographic operations demo");
    
    Ok(())
}

async fn run_benchmarks(benchmark: &str, iterations: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Running benchmarks: {} ({} iterations)", benchmark, iterations);
    
    match benchmark {
        "all" => {
            run_arithmetic_benchmark(iterations).await?;
            run_memory_benchmark(iterations).await?;
            run_crypto_benchmark(iterations).await?;
        }
        "arithmetic" => run_arithmetic_benchmark(iterations).await?,
        "memory" => run_memory_benchmark(iterations).await?,
        "crypto" => run_crypto_benchmark(iterations).await?,
        _ => {
            println!("❌ Unknown benchmark: {}", benchmark);
            println!("Available benchmarks: all, arithmetic, memory, crypto");
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn run_arithmetic_benchmark(iterations: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n➕ Arithmetic Benchmark");
    
    let program = taufold_zkvm::examples::create_arithmetic_benchmark();
    let mut vm = VirtualMachine::new();
    
    let start_time = std::time::Instant::now();
    
    for _ in 0..iterations {
        let result = vm.execute(program.clone())?;
        if !result.success {
            return Err("Benchmark execution failed".into());
        }
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / iterations;
    
    println!("Total time: {:.2}ms", total_time.as_millis());
    println!("Average time: {:.2}ms", avg_time.as_millis());
    println!("Operations/second: {:.2}", 1000.0 / avg_time.as_millis() as f64);
    
    Ok(())
}

async fn run_memory_benchmark(iterations: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 Memory Benchmark");
    
    let program = taufold_zkvm::examples::create_memory_benchmark();
    let mut vm = VirtualMachine::new();
    
    let start_time = std::time::Instant::now();
    
    for _ in 0..iterations {
        let result = vm.execute(program.clone())?;
        if !result.success {
            return Err("Benchmark execution failed".into());
        }
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / iterations;
    
    println!("Total time: {:.2}ms", total_time.as_millis());
    println!("Average time: {:.2}ms", avg_time.as_millis());
    println!("Operations/second: {:.2}", 1000.0 / avg_time.as_millis() as f64);
    
    Ok(())
}

async fn run_crypto_benchmark(iterations: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔐 Cryptographic Benchmark");
    
    let program = taufold_zkvm::examples::create_crypto_benchmark();
    let mut vm = VirtualMachine::new();
    
    let start_time = std::time::Instant::now();
    
    for _ in 0..iterations {
        let result = vm.execute(program.clone())?;
        if !result.success {
            return Err("Benchmark execution failed".into());
        }
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / iterations;
    
    println!("Total time: {:.2}ms", total_time.as_millis());
    println!("Average time: {:.2}ms", avg_time.as_millis());
    println!("Operations/second: {:.2}", 1000.0 / avg_time.as_millis() as f64);
    
    Ok(())
}