#!/usr/bin/env python3
"""
Automated validation script for TauFoldZKVM components
"""

import subprocess
import json
import os
import sys
from pathlib import Path
from datetime import datetime

class TauValidator:
    def __init__(self, tau_executable, output_dir="build/validation_results"):
        self.tau_executable = tau_executable
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.results = {}
        self.stats = {
            "unknown": 0,
            "total": 0,
            "satisfiable": 0,
            "unsatisfiable": 0,
            "error": 0,
            "too_large": 0,
            "timeout": 0
        }
    
    def check_file_size(self, file_path):
        """Check if file is within Tau limits"""
        with open(file_path, 'r') as f:
            content = f.read()
            # Find the solve statement
            if 'solve' in content:
                solve_start = content.index('solve') + 6
                solve_end = content.find('\n', solve_start)
                if solve_end == -1:
                    solve_end = len(content)
                solve_expr = content[solve_start:solve_end].strip()
                return len(solve_expr), solve_expr
        return 0, ""
    
    def validate_file(self, tau_file):
        """Validate a single Tau file"""
        file_path = Path(tau_file)
        
        # Check file size first
        expr_len, expr = self.check_file_size(file_path)
        
        if expr_len > 800:
            return {
                "status": "too_large",
                "expression_length": expr_len,
                "error": f"Expression too long: {expr_len} chars (limit: 800)"
            }
        
        # Run Tau validation
        try:
            result = subprocess.run(
                [self.tau_executable, str(file_path)],
                capture_output=True,
                text=True,
                timeout=5
            )
            
            output = result.stdout
            error = result.stderr
            
            if "solution:" in output:
                # Extract solution
                solution_start = output.index("solution:")
                solution_end = output.find("history", solution_start)
                if solution_end == -1:
                    solution_end = len(output)
                solution = output[solution_start:solution_end].strip()
                
                return {
                    "status": "satisfiable",
                    "expression_length": expr_len,
                    "solution": solution[:200] + "..." if len(solution) > 200 else solution
                }
            elif "UNSAT" in output or "unsatisfiable" in output.lower():
                return {
                    "status": "unsatisfiable",
                    "expression_length": expr_len,
                    "output": output[:200]
                }
            elif error:
                # Parse error message
                error_msg = error.strip().split('\n')[0]
                return {
                    "status": "error",
                    "expression_length": expr_len,
                    "error": error_msg
                }
            else:
                return {
                    "status": "unknown",
                    "expression_length": expr_len,
                    "output": output[:200]
                }
                
        except subprocess.TimeoutExpired:
            return {
                "status": "timeout",
                "expression_length": expr_len,
                "error": "Validation exceeded 5 seconds"
            }
        except Exception as e:
            return {
                "status": "error",
                "expression_length": expr_len,
                "error": str(e)
            }
    
    def validate_directory(self, directory):
        """Validate all Tau files in a directory"""
        tau_files = sorted(Path(directory).glob("*.tau"))
        
        if not tau_files:
            print(f"No .tau files found in {directory}")
            return
        
        print(f"\nValidating {len(tau_files)} files in {directory}...")
        print("-" * 80)
        
        for i, tau_file in enumerate(tau_files):
            print(f"[{i+1}/{len(tau_files)}] {tau_file.name:<30}", end=" ")
            
            result = self.validate_file(tau_file)
            self.results[str(tau_file)] = result
            self.stats["total"] += 1
            self.stats[result["status"]] += 1
            
            # Print status
            status = result["status"]
            if status == "satisfiable":
                print(f"✓ SATISFIABLE ({result['expression_length']} chars)")
            elif status == "too_large":
                print(f"✗ TOO LARGE ({result['expression_length']} chars)")
            elif status == "error":
                print(f"✗ ERROR: {result['error'][:50]}...")
            elif status == "timeout":
                print(f"✗ TIMEOUT")
            else:
                print(f"? {status.upper()}")
    
    def generate_report(self):
        """Generate validation report"""
        report = {
            "timestamp": datetime.now().isoformat(),
            "statistics": self.stats,
            "success_rate": self.stats["satisfiable"] / self.stats["total"] if self.stats["total"] > 0 else 0,
            "directories_tested": list(set(str(Path(f).parent) for f in self.results.keys())),
            "detailed_results": self.results
        }
        
        # Save JSON report
        report_path = self.output_dir / "validation_report.json"
        with open(report_path, 'w') as f:
            json.dump(report, f, indent=2)
        
        # Print summary
        print("\n" + "=" * 80)
        print("VALIDATION SUMMARY")
        print("=" * 80)
        print(f"Total files tested: {self.stats['total']}")
        print(f"Satisfiable: {self.stats['satisfiable']} ({self.stats['satisfiable']/self.stats['total']*100:.1f}%)")
        print(f"Too large: {self.stats['too_large']} ({self.stats['too_large']/self.stats['total']*100:.1f}%)")
        print(f"Errors: {self.stats['error']} ({self.stats['error']/self.stats['total']*100:.1f}%)")
        print(f"Timeouts: {self.stats['timeout']} ({self.stats['timeout']/self.stats['total']*100:.1f}%)")
        print(f"\nDetailed report: {report_path}")
        
        # List files that need fixing
        if self.stats['too_large'] > 0:
            print("\n⚠️  Files that exceed 800 char limit:")
            for file, result in self.results.items():
                if result['status'] == 'too_large':
                    print(f"  - {Path(file).name}: {result['expression_length']} chars")
        
        return report_path

def main():
    # Parse arguments
    if len(sys.argv) > 1:
        directories = sys.argv[1:]
    else:
        # Default directories to test
        directories = [
            "build/compositional",
            "build/compositional_fixed", 
            "build/zkvm_compositional",
            "build/zkvm"
        ]
    
    # Create validator
    tau_path = "../../../external_dependencies/run_tau.sh"
    validator = TauValidator(tau_path)
    
    # Validate each directory
    for directory in directories:
        if os.path.exists(directory):
            validator.validate_directory(directory)
        else:
            print(f"Directory not found: {directory}")
    
    # Generate report
    if validator.stats["total"] > 0:
        validator.generate_report()
    else:
        print("No files validated.")

if __name__ == "__main__":
    main()