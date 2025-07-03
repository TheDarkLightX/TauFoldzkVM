#!/usr/bin/env python3
"""Validate the existing system components to measure current progress"""

import os
import subprocess
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
import time

def validate_single_file(tau_file_path):
    """Validate a single Tau file"""
    try:
        result = subprocess.run(
            ['/Users/danax/projects/TauStandardLibrary/external_dependencies/run_tau.sh', str(tau_file_path)], 
            capture_output=True, 
            text=True, 
            timeout=10
        )
        
        if result.returncode == 0 and 'solution' in result.stdout:
            return tau_file_path.name, "PASS", ""
        else:
            error_msg = result.stderr.strip() if result.stderr else "No solution found"
            return tau_file_path.name, "FAIL", error_msg[:100]  # Truncate long errors
            
    except subprocess.TimeoutExpired:
        return tau_file_path.name, "TIMEOUT", "Validation timed out"
    except Exception as e:
        return tau_file_path.name, "ERROR", str(e)[:100]

def validate_existing_system():
    """Validate all existing system components"""
    print("Validating existing system components...")
    
    # Find all Tau files
    build_dir = Path("build/zkvm_100_percent")
    if not build_dir.exists():
        print("❌ Build directory not found!")
        return False
    
    tau_files = list(build_dir.rglob("*.tau"))
    print(f"Found {len(tau_files)} Tau files to validate")
    
    if len(tau_files) == 0:
        print("❌ No Tau files found!")
        return False
    
    # Track results by category
    results_by_category = {}
    passing = 0
    failing = 0
    timeouts = 0
    errors = 0
    
    # Use threading for I/O bound operations (faster than multiprocessing for this)
    print("\\nStarting validation (this may take a few minutes)...")
    start_time = time.time()
    
    with ThreadPoolExecutor(max_workers=4) as executor:
        # Submit all validation tasks
        future_to_file = {executor.submit(validate_single_file, tau_file): tau_file for tau_file in tau_files}
        
        # Process results as they complete
        completed = 0
        for future in as_completed(future_to_file):
            tau_file = future_to_file[future]
            file_name, status, error_msg = future.result()
            
            completed += 1
            if completed % 50 == 0:  # Progress indicator
                print(f"  Progress: {completed}/{len(tau_files)} ({completed/len(tau_files)*100:.1f}%)")
            
            # Categorize by directory
            category = tau_file.parent.name
            if category not in results_by_category:
                results_by_category[category] = {'PASS': 0, 'FAIL': 0, 'TIMEOUT': 0, 'ERROR': 0, 'details': []}
            
            results_by_category[category][status] += 1
            
            # Track overall stats
            if status == "PASS":
                passing += 1
            elif status == "FAIL": 
                failing += 1
                if len(results_by_category[category]['details']) < 3:  # Show first 3 failures per category
                    results_by_category[category]['details'].append(f"FAIL: {file_name} - {error_msg}")
            elif status == "TIMEOUT":
                timeouts += 1
            else:
                errors += 1
    
    end_time = time.time()
    validation_time = end_time - start_time
    
    # Print detailed results
    print(f"\\n=== SYSTEM VALIDATION RESULTS ===")
    print(f"Validation completed in {validation_time:.1f} seconds")
    
    # Sort categories by name for consistent output
    for category in sorted(results_by_category.keys()):
        results = results_by_category[category]
        total = results['PASS'] + results['FAIL'] + results['TIMEOUT'] + results['ERROR']
        pass_rate = (results['PASS'] / total * 100) if total > 0 else 0
        
        print(f"\\n📁 {category}: {results['PASS']}/{total} ({pass_rate:.1f}%)")
        if results['FAIL'] > 0:
            print(f"  ❌ Failures: {results['FAIL']}")
            for detail in results['details']:
                print(f"    {detail}")
        if results['TIMEOUT'] > 0:
            print(f"  ⏰ Timeouts: {results['TIMEOUT']}")
        if results['ERROR'] > 0:
            print(f"  💥 Errors: {results['ERROR']}")
    
    # Overall statistics
    total_files = len(tau_files)
    overall_pass_rate = (passing / total_files * 100) if total_files > 0 else 0
    
    print(f"\\n=== OVERALL SYSTEM STATUS ===")
    print(f"📊 Total files: {total_files}")
    print(f"✅ Passing: {passing} ({overall_pass_rate:.1f}%)")
    print(f"❌ Failing: {failing} ({failing/total_files*100:.1f}%)")
    print(f"⏰ Timeouts: {timeouts} ({timeouts/total_files*100:.1f}%)")
    print(f"💥 Errors: {errors} ({errors/total_files*100:.1f}%)")
    
    # Progress assessment
    if overall_pass_rate >= 90:
        print(f"\\n🎉 EXCELLENT! {overall_pass_rate:.1f}% validation rate - very close to 100%!")
    elif overall_pass_rate >= 70:
        print(f"\\n🚀 GREAT PROGRESS! {overall_pass_rate:.1f}% validation rate - well on track!")
    elif overall_pass_rate >= 50:
        print(f"\\n📈 GOOD PROGRESS! {overall_pass_rate:.1f}% validation rate - making solid headway!")
    else:
        print(f"\\n🔧 {overall_pass_rate:.1f}% validation rate - more work needed!")
    
    return overall_pass_rate >= 70

if __name__ == "__main__":
    success = validate_existing_system()
    print(f"\\nValidation complete!")