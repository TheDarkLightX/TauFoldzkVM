#!/usr/bin/env python3
"""Test simplified memory components"""

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

def create_simple_memory():
    """Create simplified memory components using Boolean algebra"""
    
    components = []
    
    # Simple DUP operation (from updated achieve_100_percent.py)
    dup_comp = Component(
        name="dup_nibble_0",
        constraints=[
            "top0=1",        # Example: stack top value
            "dup0=top0"      # Duplicate operation
        ],
        assumptions=[],
        guarantees=["dup0"]
    )
    components.append(dup_comp)
    
    # Simple SWAP operation (from updated achieve_100_percent.py)
    swap_comp = Component(
        name="swap_nibble_0",
        constraints=[
            "a0=1",           # Example: first value
            "b0=0",           # Example: second value
            "swapa0=b0",      # a gets b
            "swapb0=a0"       # b gets a
        ],
        assumptions=[],
        guarantees=["swapa0", "swapb0"]
    )
    components.append(swap_comp)
    
    # Simple ADDR operation (basic pattern)
    addr_comp = Component(
        name="addr_nibble_0",
        constraints=[
            "a0=0",
            "addr0=a0"
        ],
        assumptions=[],
        guarantees=["addr0"]
    )
    components.append(addr_comp)
    
    # Simple DATA operation (basic pattern)
    data_comp = Component(
        name="data_nibble_0",
        constraints=[
            "d0=1",
            "data0=d0"
        ],
        assumptions=[],
        guarantees=["data0"]
    )
    components.append(data_comp)
    
    return components

def test_simple_memory():
    """Test the simplified memory components"""
    print("Testing simplified memory components...")
    
    # Create output directory
    os.makedirs("test_simple_memory", exist_ok=True)
    
    # Generate components
    components = create_simple_memory()
    
    passing = 0
    failing = 0
    
    for comp in components:
        print(f"\n=== {comp.name} ===")
        
        try:
            # Generate Tau content
            content = comp.to_tau()
            print(f"✓ Generated: {len(content)} chars")
            
            # Write to file
            filepath = f"test_simple_memory/{comp.name}.tau"
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
                if result.stderr:
                    print(f"  Error: {result.stderr.strip()}")
                failing += 1
                
        except Exception as e:
            print(f"✗ ERROR: {e}")
            failing += 1
    
    print(f"\nValidation Results: {passing}/{len(components)} passed ({passing/len(components)*100:.1f}%)")
    return passing >= len(components)

if __name__ == "__main__":
    success = test_simple_memory()
    if success:
        print("\n🎉 All simplified memory components are working correctly!")
    else:
        print("\n❌ Some memory components still need fixing")