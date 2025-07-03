#!/usr/bin/env python3
"""
Test Framework for TauFoldZKVM
Validates all generated Tau files and tests VM functionality.
"""

import os
import subprocess
import json
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
import concurrent.futures
import time

@dataclass
class TestResult:
    """Result of a single test."""
    file: str
    success: bool
    satisfiable: bool
    error: Optional[str]
    execution_time: float

class ZKVMTestFramework:
    """
    Comprehensive test framework for the zkVM.
    Tests all generated Tau files for satisfiability.
    """
    
    def __init__(self, tau_command: str = "./external_dependencies/run_tau.sh"):
        self.tau_command = tau_command
        self.results: List[TestResult] = []
        self.project_root = "/Users/danax/projects/TauStandardLibrary"
    
    def test_single_file(self, filepath: str) -> TestResult:
        """Test a single Tau file for satisfiability."""
        start_time = time.time()
        
        try:
            # Change to project root for tau command
            original_cwd = os.getcwd()
            os.chdir(self.project_root)
            
            # Run Tau on the file
            result = subprocess.run(
                [self.tau_command, filepath],
                capture_output=True,
                text=True,
                timeout=30  # 30 second timeout
            )
            
            os.chdir(original_cwd)
            
            # Check if satisfiable
            output = result.stdout
            error = result.stderr
            
            # Look for solution in output
            satisfiable = "solution:" in output
            success = result.returncode == 0 or satisfiable
            
            # Extract error if any
            error_msg = None
            if not success and error:
                error_msg = error.strip()
            elif not success and "Error" in output:
                # Extract error from output
                lines = output.split('\n')
                for line in lines:
                    if "Error" in line:
                        error_msg = line.strip()
                        break
            
            execution_time = time.time() - start_time
            
            return TestResult(
                file=filepath,
                success=success,
                satisfiable=satisfiable,
                error=error_msg,
                execution_time=execution_time
            )
            
        except subprocess.TimeoutExpired:
            execution_time = time.time() - start_time
            return TestResult(
                file=filepath,
                success=False,
                satisfiable=False,
                error="Timeout after 30 seconds",
                execution_time=execution_time
            )
        except Exception as e:
            execution_time = time.time() - start_time
            return TestResult(
                file=filepath,
                success=False,
                satisfiable=False,
                error=str(e),
                execution_time=execution_time
            )
    
    def test_module(self, module_name: str, files: List[str]) -> Dict[str, TestResult]:
        """Test all files in a module."""
        module_results = {}
        
        print(f"\nTesting module: {module_name}")
        print("-" * 50)
        
        for file in files:
            print(f"  Testing {file}...", end="", flush=True)
            result = self.test_single_file(file)
            module_results[file] = result
            self.results.append(result)
            
            if result.satisfiable:
                print(f" ✓ SATISFIABLE ({result.execution_time:.2f}s)")
            elif result.success:
                print(f" ✓ SUCCESS ({result.execution_time:.2f}s)")
            else:
                print(f" ✗ FAILED: {result.error}")
        
        return module_results
    
    def test_all_modules(self, manifest_path: str) -> Dict[str, Dict[str, TestResult]]:
        """Test all modules from manifest."""
        # Load manifest
        with open(manifest_path, 'r') as f:
            manifest = json.load(f)
        
        all_results = {}
        output_dir = manifest['output_dir']
        
        # Test each module
        for module_name, files in manifest['modules'].items():
            # Convert relative paths to full paths
            full_paths = [
                os.path.join(output_dir, file) 
                for file in files
            ]
            
            module_results = self.test_module(module_name, full_paths)
            all_results[module_name] = module_results
        
        return all_results
    
    def parallel_test_all(self, manifest_path: str, max_workers: int = 4) -> Dict[str, Dict[str, TestResult]]:
        """Test all modules in parallel."""
        # Load manifest
        with open(manifest_path, 'r') as f:
            manifest = json.load(f)
        
        all_results = {}
        output_dir = manifest['output_dir']
        
        print(f"\nRunning parallel tests with {max_workers} workers...")
        
        # Collect all test tasks
        test_tasks = []
        for module_name, files in manifest['modules'].items():
            for file in files:
                full_path = os.path.join(output_dir, file)
                test_tasks.append((module_name, file, full_path))
        
        # Run tests in parallel
        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
            # Submit all tasks
            future_to_task = {
                executor.submit(self.test_single_file, task[2]): task
                for task in test_tasks
            }
            
            # Process results as they complete
            for future in concurrent.futures.as_completed(future_to_task):
                module_name, file, full_path = future_to_task[future]
                result = future.result()
                
                # Store result
                if module_name not in all_results:
                    all_results[module_name] = {}
                all_results[module_name][file] = result
                self.results.append(result)
                
                # Print progress
                if result.satisfiable:
                    status = "✓ SAT"
                elif result.success:
                    status = "✓ OK"
                else:
                    status = "✗ FAIL"
                
                print(f"{status} {module_name}/{file} ({result.execution_time:.2f}s)")
        
        return all_results
    
    def generate_report(self, results: Dict[str, Dict[str, TestResult]]) -> str:
        """Generate a comprehensive test report."""
        report = []
        report.append("=" * 70)
        report.append("TauFoldZKVM Test Report")
        report.append("=" * 70)
        report.append("")
        
        # Summary statistics
        total_files = len(self.results)
        satisfiable = sum(1 for r in self.results if r.satisfiable)
        successful = sum(1 for r in self.results if r.success)
        failed = sum(1 for r in self.results if not r.success)
        total_time = sum(r.execution_time for r in self.results)
        
        report.append("Summary:")
        report.append(f"  Total files tested: {total_files}")
        report.append(f"  Satisfiable: {satisfiable}")
        report.append(f"  Successful: {successful}")
        report.append(f"  Failed: {failed}")
        report.append(f"  Total execution time: {total_time:.2f}s")
        report.append(f"  Average time per file: {total_time/total_files:.2f}s")
        report.append("")
        
        # Module breakdown
        report.append("Module Results:")
        for module_name, module_results in results.items():
            module_sat = sum(1 for r in module_results.values() if r.satisfiable)
            module_total = len(module_results)
            report.append(f"\n  {module_name}: {module_sat}/{module_total} satisfiable")
            
            # Show any errors
            for file, result in module_results.items():
                if not result.success:
                    report.append(f"    ✗ {file}: {result.error}")
        
        report.append("")
        
        # Performance analysis
        report.append("Performance Analysis:")
        
        # Slowest files
        slowest = sorted(self.results, key=lambda r: r.execution_time, reverse=True)[:5]
        report.append("\n  Slowest files:")
        for result in slowest:
            report.append(f"    {result.file}: {result.execution_time:.2f}s")
        
        # Error analysis
        if failed > 0:
            report.append("\n  Error Summary:")
            error_types = {}
            for result in self.results:
                if result.error:
                    error_type = result.error.split(':')[0] if ':' in result.error else result.error
                    error_types[error_type] = error_types.get(error_type, 0) + 1
            
            for error_type, count in sorted(error_types.items(), key=lambda x: x[1], reverse=True):
                report.append(f"    {error_type}: {count} occurrences")
        
        report.append("")
        report.append("=" * 70)
        
        return "\n".join(report)
    
    def save_report(self, report: str, filepath: str):
        """Save test report to file."""
        with open(filepath, 'w') as f:
            f.write(report)
        print(f"\nReport saved to: {filepath}")

def main():
    """Run comprehensive zkVM tests."""
    print("TauFoldZKVM Test Framework")
    print("=" * 50)
    
    # Create test framework
    framework = ZKVMTestFramework()
    
    # Find manifest
    manifest_path = "build/zkvm/manifest.json"
    if not os.path.exists(manifest_path):
        print(f"Error: Manifest not found at {manifest_path}")
        print("Please run zkvm_full_implementation.py first")
        return
    
    # Run tests (parallel for speed)
    results = framework.parallel_test_all(manifest_path, max_workers=8)
    
    # Generate report
    report = framework.generate_report(results)
    print("\n" + report)
    
    # Save report
    framework.save_report(report, "build/zkvm/test_report.txt")
    
    # Create simplified test files for debugging
    print("\nCreating simplified test files for failed cases...")
    
    failed_files = [r for r in framework.results if not r.success]
    if failed_files:
        os.makedirs("build/zkvm/debug", exist_ok=True)
        
        for result in failed_files[:5]:  # Debug first 5 failures
            if "Syntax Error" in str(result.error):
                print(f"  Creating debug file for {result.file}")
                # Would create simplified version here

if __name__ == "__main__":
    main()