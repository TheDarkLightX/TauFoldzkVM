#!/usr/bin/env python3
"""Test fixed aggregators with valid variable names"""

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

def test_fixed_aggregators():
    """Test aggregators with fixed variable names (no underscores)"""
    print("Testing fixed aggregators with valid variable names...")
    
    # Create output directory
    os.makedirs("test_aggregators_fixed", exist_ok=True)
    
    # Fixed aggregators (no underscores in variable names)
    components = [
        Component("dup_aggregator", ["allok=1", "dupcomplete=allok"], [], ["dupcomplete"]),
        Component("swap_aggregator", ["allswapped=1", "swapcomplete=allswapped"], [], ["swapcomplete"])
    ]
    
    passing = 0
    failing = 0
    
    for comp in components:
        print(f"\n=== {comp.name} ===")
        
        try:
            # Generate Tau content
            content = comp.to_tau()
            print(f"✓ Generated: {len(content)} chars")
            
            # Write to file
            filepath = f"test_aggregators_fixed/{comp.name}.tau"
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
    success = test_fixed_aggregators()
    if success:
        print("\n🎉 All fixed aggregators are working correctly!")
    else:
        print("\n❌ Some aggregators still need fixing")