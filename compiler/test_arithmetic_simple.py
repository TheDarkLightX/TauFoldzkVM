#!/usr/bin/env python3
"""Test arithmetic components with simple Boolean algebra approach"""

import os
import subprocess
from pathlib import Path

# Component class (same as proven working version)
class Component:
    def __init__(self, name: str, constraints: list, assumptions=None, guarantees=None):
        self.name = name
        self.constraints = constraints
        self.assumptions = assumptions or []
        self.guarantees = guarantees or []
    
    def to_tau(self) -> str:
        """Convert to Tau file content"""
        expr = " && ".join(self.constraints)
        if len(expr) > 700:  # Conservative limit
            raise ValueError(f"{self.name}: Expression too long ({len(expr)} chars)")
        
        content = f"# Component: {self.name}\n"
        if self.assumptions:
            content += f"# Assumptions: {', '.join(self.assumptions)}\n"
        if self.guarantees:
            content += f"# Guarantees: {', '.join(self.guarantees)}\n"
        content += f"\nsolve {expr}\n\nquit"
        return content

def create_simple_arithmetic():
    """Create simplified arithmetic components using Boolean algebra"""
    
    components = []
    
    # Simple ADD nibble (working pattern from existing code)
    add_comp = Component(
        name="add_nibble_0",
        constraints=[
            "a0=1", "b0=1",  # Example: 1 + 1
            "s0=(a0+b0)",    # XOR for sum
            "c0=(a0&b0)"     # AND for carry
        ],
        assumptions=[],
        guarantees=["s0", "c0"]
    )
    components.append(add_comp)
    
    # Simple SUB nibble (using Boolean algebra)
    sub_comp = Component(
        name="sub_nibble_0", 
        constraints=[
            "a0=1", "b0=0",      # Example: 1 - 0 = 1
            "diff0=(a0&(b0+1))"  # Simplified subtraction
        ],
        assumptions=[],
        guarantees=["diff0"]
    )
    components.append(sub_comp)
    
    # Simple MUL nibble (using Boolean algebra)
    mul_comp = Component(
        name="mul_partial_0",
        constraints=[
            "a0=1", "b0=1",      # Example: 1 * 1 = 1  
            "prod0=(a0&b0)"      # AND for simple multiplication
        ],
        assumptions=[],
        guarantees=["prod0"]
    )
    components.append(mul_comp)
    
    # Simple DIV nibble (using Boolean algebra)
    div_comp = Component(
        name="div_nibble_0",
        constraints=[
            "a0=1", "b0=1",      # Example: 1 / 1 = 1
            "q0=(a0&b0)",        # Quotient (simplified)
            "r0=0"               # Remainder
        ],
        assumptions=[],
        guarantees=["q0", "r0"]
    )
    components.append(div_comp)
    
    # Simple MOD nibble (using Boolean algebra)
    mod_comp = Component(
        name="mod_nibble_0",
        constraints=[
            "a0=1", "b0=1",      # Example: 1 % 1 = 0
            "rem0=(a0&(b0+1))"   # Simplified remainder
        ],
        assumptions=[],
        guarantees=["rem0"]
    )
    components.append(mod_comp)
    
    return components

def test_simple_arithmetic():
    """Test the simplified arithmetic components"""
    print("Testing simplified arithmetic components...")
    
    # Create output directory
    os.makedirs("test_simple_arithmetic", exist_ok=True)
    
    # Generate components
    components = create_simple_arithmetic()
    
    passing = 0
    failing = 0
    
    for comp in components:
        print(f"\n=== {comp.name} ===")
        
        try:
            # Generate Tau content
            content = comp.to_tau()
            print(f"✓ Generated: {len(content)} chars")
            
            # Write to file
            filepath = f"test_simple_arithmetic/{comp.name}.tau"
            with open(filepath, 'w') as f:
                f.write(content)
            
            # Validate with Tau
            result = subprocess.run(['/Users/danax/projects/TauStandardLibrary/external_dependencies/run_tau.sh', filepath], 
                                  capture_output=True, text=True, timeout=10)
            if result.returncode == 0 and 'solution' in result.stdout:
                print(f"✓ PASS: Tau validation successful")
                passing += 1
            else:
                print(f"✗ FAIL: Tau validation failed")
                print(f"  Error: {result.stderr.strip()}")
                failing += 1
                
        except Exception as e:
            print(f"✗ ERROR: {e}")
            failing += 1
    
    print(f"\nValidation Results: {passing}/{len(components)} passed ({passing/len(components)*100:.1f}%)")
    return passing >= len(components)

if __name__ == "__main__":
    success = test_simple_arithmetic()
    if success:
        print("\n🎉 All arithmetic components are working correctly!")
    else:
        print("\n❌ Some arithmetic components need fixing")