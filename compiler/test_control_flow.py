#!/usr/bin/env python3
"""
Test script for control flow instruction components
"""

import os
import sys
from pathlib import Path
from control_flow_generator import ControlFlowGenerator, generate_control_flow_instruction

def test_control_flow_components():
    """Test generation of control flow components"""
    cf_gen = ControlFlowGenerator()
    
    print("=== Testing Control Flow Component Generation ===\n")
    
    # Test each control flow instruction
    instructions = ["JMP", "JZ", "JNZ", "CALL", "RET"]
    
    for instruction in instructions:
        print(f"\nGenerating components for {instruction}...")
        
        try:
            components = generate_control_flow_instruction(instruction)
            print(f"  Generated {len(components)} components")
            
            # Check each component
            for i, comp in enumerate(components):
                expr = " && ".join(comp.constraints)
                expr_len = len(expr)
                
                print(f"  [{i+1}] {comp.name}: {expr_len} chars", end="")
                
                if expr_len > 700:
                    print(" ❌ TOO LONG!")
                else:
                    print(" ✓")
                
                # Show assumptions and guarantees
                if comp.assumptions:
                    print(f"       Assumptions: {', '.join(comp.assumptions)}")
                if comp.guarantees:
                    print(f"       Guarantees: {', '.join(comp.guarantees)}")
        
        except Exception as e:
            print(f"  ERROR: {e}")
    
    # Test individual nibble generation
    print("\n\n=== Testing Individual Nibble Components ===\n")
    
    test_cases = [
        ("JMP nibble 0", lambda: cf_gen.generate_jmp_nibble(0)),
        ("JZ nibble 0", lambda: cf_gen.generate_jz_nibble(0)),
        ("JNZ nibble 4", lambda: cf_gen.generate_jnz_nibble(4)),
        ("CALL nibble 7", lambda: cf_gen.generate_call_nibble(7)),
        ("RET nibble 3", lambda: cf_gen.generate_ret_nibble(3)),
        ("Zero flag aggregator", lambda: cf_gen.generate_zero_flag_aggregator()),
        ("PC carry link 0->1", lambda: cf_gen.generate_pc_carry_link(0, 1)),
    ]
    
    for test_name, generator in test_cases:
        try:
            comp = generator()
            expr = " && ".join(comp.constraints)
            expr_len = len(expr)
            
            print(f"{test_name}: {expr_len} chars", end="")
            if expr_len > 700:
                print(" ❌ TOO LONG!")
            else:
                print(" ✓")
        except Exception as e:
            print(f"{test_name}: ERROR - {e}")
    
    # Write example components to files for inspection
    print("\n\n=== Writing Example Components ===\n")
    
    output_dir = Path("build/control_flow_test")
    output_dir.mkdir(parents=True, exist_ok=True)
    
    example_components = [
        cf_gen.generate_jmp_nibble(0),
        cf_gen.generate_jz_nibble(0),
        cf_gen.generate_call_nibble(0),
        cf_gen.generate_ret_nibble(0),
        cf_gen.generate_zero_flag_aggregator(),
    ]
    
    for comp in example_components:
        filepath = output_dir / f"{comp.name}.tau"
        with open(filepath, 'w') as f:
            f.write(comp.to_tau())
        print(f"Wrote {filepath}")
    
    print(f"\nExample components written to {output_dir}/")

if __name__ == "__main__":
    test_control_flow_components()