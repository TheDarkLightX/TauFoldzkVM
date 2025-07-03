"""
Test script for Memory Generator.

This script tests the memory generator's ability to create compositional contracts
for various memory operations.

Copyright (c) 2025 Dana Edwards. All rights reserved.
"""

import sys
import os
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent.parent.parent.parent))

from src.zkvm.compiler.subagents.memory_generator import MemoryGenerator


def test_memory_operations():
    """Test memory operation generation."""
    print("Testing Memory Generator...")
    print("=" * 60)
    
    # Initialize generator
    generator = MemoryGenerator(output_dir="test_output/memory")
    
    # Test different memory operations
    operations = ["LOAD", "STORE", "MLOAD", "MSTORE", "PUSH", "POP", "DUP", "SWAP"]
    
    results = generator.generate(operations)
    
    # Print results
    for op, result in results.items():
        print(f"\n{op} Operation:")
        print(f"  Components: {len(result.components_generated)}")
        print(f"  Files: {len(result.files)}")
        print(f"  Total Constraints: {result.total_constraints}")
        
        # Show first few components
        print(f"  Sample Components:")
        for i, comp in enumerate(result.components_generated[:3]):
            print(f"    - {comp.name}: nibble={comp.nibble_index}, "
                  f"addr={comp.is_address}, data={comp.is_data}")
        
        if len(result.components_generated) > 3:
            print(f"    ... and {len(result.components_generated) - 3} more")
    
    # Test specific operation details
    print("\n" + "=" * 60)
    print("Detailed LOAD Operation Analysis:")
    load_result = results["LOAD"]
    
    # Count component types
    addr_components = sum(1 for c in load_result.components_generated if c.is_address)
    data_components = sum(1 for c in load_result.components_generated if c.is_data)
    other_components = len(load_result.components_generated) - addr_components - data_components
    
    print(f"  Address Components: {addr_components}")
    print(f"  Data Components: {data_components}")
    print(f"  Other Components: {other_components}")
    
    # Show a sample contract
    if load_result.components_generated:
        sample = load_result.components_generated[0]
        print(f"\nSample Contract ({sample.name}):")
        print(f"  Variables: {len(sample.contract.variables)}")
        print(f"  Constraints: {len(sample.contract.constraints)}")
        print(f"  Assumptions: {sample.contract.assumptions}")
        print(f"  Guarantees: {sample.contract.guarantees}")
        
        # Show constraint length
        expr = sample.contract.to_tau_expression()
        print(f"  Expression Length: {len(expr)} chars (max: {sample.contract.max_chars})")
    
    # Test stack operations
    print("\n" + "=" * 60)
    print("Stack Operations Analysis:")
    for op in ["PUSH", "POP", "DUP", "SWAP"]:
        result = results[op]
        print(f"\n{op}:")
        
        # Find unique component types
        comp_types = set()
        for comp in result.components_generated:
            if "SP" in comp.name:
                comp_types.add("Stack Pointer")
            elif "DATA" in comp.name:
                comp_types.add("Data Transfer")
            elif "CHECK" in comp.name:
                comp_types.add("Safety Check")
            elif "COORD" in comp.name:
                comp_types.add("Coordination")
            else:
                comp_types.add("Other")
        
        print(f"  Component Types: {', '.join(sorted(comp_types))}")
    
    print("\n" + "=" * 60)
    print("Memory Generator Test Complete!")
    
    # Save some files for inspection
    print("\nSaving sample files...")
    os.makedirs("test_output/memory", exist_ok=True)
    
    # Save LOAD operation files
    generator.save_memory_files(results["LOAD"])
    print("  Saved LOAD operation files to test_output/memory/load/")
    
    # Save PUSH operation files  
    generator.save_memory_files(results["PUSH"])
    print("  Saved PUSH operation files to test_output/memory/push/")


def test_contract_size_limits():
    """Test that contracts stay within size limits."""
    print("\n" + "=" * 60)
    print("Testing Contract Size Limits...")
    
    generator = MemoryGenerator()
    
    # Generate all operations
    all_ops = ["LOAD", "STORE", "MLOAD", "MSTORE", "PUSH", "POP", "DUP", "SWAP"]
    results = generator.generate(all_ops)
    
    oversized = []
    for op, result in results.items():
        for comp in result.components_generated:
            try:
                expr = comp.contract.to_tau_expression()
                if len(expr) > 650:  # Warning threshold
                    oversized.append((op, comp.name, len(expr)))
            except ValueError as e:
                oversized.append((op, comp.name, "ERROR: " + str(e)))
    
    if oversized:
        print("\nComponents near or over size limit:")
        for op, name, size in oversized:
            print(f"  {op}/{name}: {size}")
    else:
        print("\nAll components within size limits!")
    
    # Show size distribution
    print("\nSize Distribution:")
    sizes = []
    for op, result in results.items():
        for comp in result.components_generated:
            try:
                expr = comp.contract.to_tau_expression()
                sizes.append(len(expr))
            except:
                pass
    
    if sizes:
        print(f"  Min: {min(sizes)} chars")
        print(f"  Max: {max(sizes)} chars")
        print(f"  Avg: {sum(sizes) // len(sizes)} chars")


def test_specific_operations():
    """Test specific memory operation scenarios."""
    print("\n" + "=" * 60)
    print("Testing Specific Operations...")
    
    generator = MemoryGenerator()
    
    # Test just LOAD
    print("\nGenerating LOAD operation...")
    result = generator.generate(["LOAD"])["LOAD"]
    
    print(f"LOAD Components ({len(result.components_generated)} total):")
    
    # Group by type
    addr_nibbles = [c for c in result.components_generated 
                    if "ADDR_" in c.name and c.nibble_index >= 0]
    data_nibbles = [c for c in result.components_generated 
                    if "DATA_" in c.name]
    validators = [c for c in result.components_generated 
                  if "VALID" in c.name or "CONSISTENCY" in c.name]
    
    print(f"  Address Nibbles: {len(addr_nibbles)}")
    print(f"  Data Nibbles: {len(data_nibbles)}")
    print(f"  Validators: {len(validators)}")
    
    # Show sample Tau file
    if result.files:
        first_file = list(result.files.items())[0]
        print(f"\nSample Tau File ({first_file[0]}):")
        print("-" * 40)
        lines = first_file[1].split('\n')[:10]
        for line in lines:
            print(f"  {line}")
        if len(first_file[1].split('\n')) > 10:
            print("  ...")


if __name__ == "__main__":
    test_memory_operations()
    test_contract_size_limits()
    test_specific_operations()