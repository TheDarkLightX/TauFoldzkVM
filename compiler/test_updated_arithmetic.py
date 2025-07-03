#!/usr/bin/env python3
"""Test the updated arithmetic components"""

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

# Import the updated generators by loading specific functions
def test_updated_arithmetic():
    """Test the updated arithmetic components using the actual generator classes"""
    
    # Load the actual classes from the main file
    exec(open('achieve_100_percent.py').read(), globals())
    
    print("Testing updated arithmetic components...")
    
    # Create output directory
    os.makedirs("test_updated_arithmetic", exist_ok=True)
    
    # Create generator
    gen = ArithmeticGenerator()
    
    # Test components
    test_components = []
    
    # Test SUB nibble
    try:
        sub_comp = gen.generate_sub_nibble(0)
        test_components.append(('SUB', sub_comp))
    except Exception as e:
        print(f"Error generating SUB: {e}")
    
    # Test DIV nibble
    try:
        div_comp = gen.generate_div_nibble(0)
        test_components.append(('DIV', div_comp))
    except Exception as e:
        print(f"Error generating DIV: {e}")
        
    # Test MOD nibble
    try:
        mod_comp = gen.generate_mod_nibble(0)
        test_components.append(('MOD', mod_comp))
    except Exception as e:
        print(f"Error generating MOD: {e}")
    
    # Validate each component
    passing = 0
    failing = 0
    
    for op_name, comp in test_components:
        print(f"\n=== {op_name}: {comp.name} ===")
        
        try:
            # Generate Tau content
            content = comp.to_tau()
            print(f"✓ Generated: {len(content)} chars")
            
            # Write to file
            filepath = f"test_updated_arithmetic/{comp.name}.tau"
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
    
    print(f"\nValidation Results: {passing}/{len(test_components)} passed ({passing/len(test_components)*100:.1f}%)")
    return passing >= len(test_components)

if __name__ == "__main__":
    success = test_updated_arithmetic()
    if success:
        print("\n🎉 All updated arithmetic components are working correctly!")
    else:
        print("\n❌ Some arithmetic components still need fixing")