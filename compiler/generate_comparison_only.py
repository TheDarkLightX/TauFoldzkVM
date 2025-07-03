#!/usr/bin/env python3
"""Generate only comparison components to test the fixed aggregators"""

import os
import sys
from pathlib import Path

# Add current directory to path
sys.path.insert(0, str(Path(__file__).parent))

from achieve_100_percent import *

def generate_comparison_components():
    """Generate only the comparison instruction components"""
    
    print("Generating comparison components...")
    
    # Create output directory
    output_dir = "build/zkvm_comparison_test"
    os.makedirs(output_dir, exist_ok=True)
    
    # Create generators
    comp_gen = ComparisonGenerator()
    arith_gen = ArithmeticGenerator()  # For aggregators
    
    components = []
    
    # Generate comparison nibbles (EQ, NEQ, LT, GT, LTE, GTE)
    for instruction in ['eq', 'neq', 'lt', 'gt', 'lte', 'gte']:
        print(f"Generating {instruction.upper()} components...")
        
        # Create instruction directory
        inst_dir = f"{output_dir}/{instruction}"
        os.makedirs(inst_dir, exist_ok=True)
        
        # Generate nibble components
        for nibble in range(4):  # Use 4 nibbles for testing
            if instruction == 'eq':
                comp = comp_gen.generate_eq_nibble(nibble)
            elif instruction == 'neq':
                comp = comp_gen.generate_neq_nibble(nibble)
            elif instruction == 'lt':
                comp = comp_gen.generate_lt_nibble(nibble)
            elif instruction == 'gt':
                comp = comp_gen.generate_gt_nibble(nibble)
            elif instruction == 'lte':
                comp = comp_gen.generate_lte_nibble(nibble)
            elif instruction == 'gte':
                comp = comp_gen.generate_gte_nibble(nibble)
            
            # Save component
            filepath = f"{inst_dir}/{comp.name}.tau"
            with open(filepath, 'w') as f:
                f.write(comp.to_tau())
            components.append((filepath, comp.name))
            
        # Generate aggregator
        if instruction == 'eq':
            agg = arith_gen.generate_eq_aggregator()
        elif instruction == 'neq':
            agg = arith_gen.generate_neq_aggregator()
        elif instruction == 'lt':
            agg = arith_gen.generate_lt_aggregator()
        elif instruction == 'gt':
            agg = arith_gen.generate_gt_aggregator()
        elif instruction == 'lte':
            agg = arith_gen.generate_lte_aggregator()
        elif instruction == 'gte':
            agg = arith_gen.generate_gte_aggregator()
            
        # Save aggregator
        agg_filepath = f"{inst_dir}/{agg.name}.tau"
        with open(agg_filepath, 'w') as f:
            f.write(agg.to_tau())
        components.append((agg_filepath, agg.name))
    
    print(f"Generated {len(components)} comparison components")
    return components

def validate_components(components):
    """Validate all generated components"""
    print("\\nValidating components...")
    
    passing = 0
    failing = 0
    
    for filepath, name in components:
        try:
            result = subprocess.run(['/Users/danax/projects/TauStandardLibrary/external_dependencies/run_tau.sh', filepath], 
                                  capture_output=True, text=True, timeout=10)
            if result.returncode == 0 and 'solution' in result.stdout:
                print(f"✓ PASS: {name}")
                passing += 1
            else:
                print(f"✗ FAIL: {name}")
                failing += 1
        except Exception as e:
            print(f"✗ ERROR: {name} - {e}")
            failing += 1
    
    print(f"\\nValidation Results: {passing}/{len(components)} passed ({passing/len(components)*100:.1f}%)")
    return passing, failing

if __name__ == "__main__":
    import subprocess
    
    # Generate components
    components = generate_comparison_components()
    
    # Validate them
    validate_components(components)