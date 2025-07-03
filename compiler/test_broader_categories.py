#!/usr/bin/env python3
"""Test broader categories of components to find remaining issues"""

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

def create_test_components_from_categories():
    """Create test components from various categories to check for remaining issues"""
    
    components = []
    
    # 1. Bitwise operations (simple patterns)
    and_comp = Component("and_nibble_0", ["a0=1", "b0=1", "and0=(a0&b0)"], [], ["and0"])
    or_comp = Component("or_nibble_0", ["a0=1", "b0=0", "or0=(a0|b0)"], [], ["or0"])
    xor_comp = Component("xor_nibble_0", ["a0=1", "b0=1", "xor0=(a0+b0)"], [], ["xor0"])
    not_comp = Component("not_nibble_0", ["a0=1", "not0=(a0+1)"], [], ["not0"])
    
    components.extend([and_comp, or_comp, xor_comp, not_comp])
    
    # 2. System operations (simple patterns)
    debug_comp = Component("debug_nibble_0", ["a0=0", "b0=1", "out0=(a0&b0)", "debug=1"], [], ["debug"])
    assert_comp = Component("assert_nibble_0", ["cond0=1", "assert0=cond0"], [], ["assert0"])
    log_comp = Component("log_nibble_0", ["data0=1", "log0=data0"], [], ["log0"])
    
    components.extend([debug_comp, assert_comp, log_comp])
    
    # 3. Crypto operations (simple patterns)
    hash_comp = Component("hash_nibble_0", ["in0=1", "hash0=(in0+1)"], [], ["hash0"])
    verify_comp = Component("verify_nibble_0", ["sig0=1", "msg0=1", "verify0=(sig0&msg0)"], [], ["verify0"])
    
    components.extend([hash_comp, verify_comp])
    
    # 4. Control flow (simple patterns)
    jmp_comp = Component("jmp_simple", ["addr0=1", "jmp0=addr0"], [], ["jmp0"])
    call_comp = Component("call_simple", ["target0=1", "call0=target0"], [], ["call0"])
    
    components.extend([jmp_comp, call_comp])
    
    # 5. Shift operations (simple patterns)
    shl_comp = Component("shl_nibble_0", ["a0=1", "shift0=1", "shl0=(a0&shift0)"], [], ["shl0"])
    shr_comp = Component("shr_nibble_0", ["a0=1", "shift0=1", "shr0=(a0&shift0)"], [], ["shr0"])
    
    components.extend([shl_comp, shr_comp])
    
    return components

def test_broader_categories():
    """Test components from broader categories"""
    print("Testing broader categories of components...")
    
    # Create output directory
    os.makedirs("test_broader_categories", exist_ok=True)
    
    # Generate components
    components = create_test_components_from_categories()
    
    passing = 0
    failing = 0
    results_by_category = {
        "Bitwise": [],
        "System": [], 
        "Crypto": [],
        "Control": [],
        "Shift": []
    }
    
    for comp in components:
        try:
            # Generate Tau content
            content = comp.to_tau()
            
            # Write to file
            filepath = f"test_broader_categories/{comp.name}.tau"
            with open(filepath, 'w') as f:
                f.write(content)
            
            # Validate with Tau
            result = subprocess.run(['/Users/danax/projects/TauStandardLibrary/external_dependencies/run_tau.sh', filepath], 
                                  capture_output=True, text=True, timeout=10)
            
            status = "PASS" if (result.returncode == 0 and 'solution' in result.stdout) else "FAIL"
            
            # Categorize results
            if "and_" in comp.name or "or_" in comp.name or "xor_" in comp.name or "not_" in comp.name:
                results_by_category["Bitwise"].append(f"{status}: {comp.name}")
            elif "debug_" in comp.name or "assert_" in comp.name or "log_" in comp.name:
                results_by_category["System"].append(f"{status}: {comp.name}")
            elif "hash_" in comp.name or "verify_" in comp.name:
                results_by_category["Crypto"].append(f"{status}: {comp.name}")
            elif "jmp_" in comp.name or "call_" in comp.name:
                results_by_category["Control"].append(f"{status}: {comp.name}")
            elif "shl_" in comp.name or "shr_" in comp.name:
                results_by_category["Shift"].append(f"{status}: {comp.name}")
            
            if status == "PASS":
                passing += 1
            else:
                failing += 1
                
        except Exception as e:
            failing += 1
            print(f"ERROR: {comp.name} - {e}")
    
    # Print results by category
    print(f"\n=== CATEGORY VALIDATION RESULTS ===")
    for category, results in results_by_category.items():
        category_passing = len([r for r in results if r.startswith("PASS")])
        category_total = len(results)
        print(f"\n{category}: {category_passing}/{category_total} ({category_passing/category_total*100:.1f}% if category_total > 0 else 0)" if category_total > 0 else f"\n{category}: 0/0 (0%)")
        for result in results:
            print(f"  {result}")
    
    print(f"\n=== OVERALL RESULTS ===")
    print(f"Total: {passing}/{len(components)} ({passing/len(components)*100:.1f}%)")
    
    return passing, failing

if __name__ == "__main__":
    passing, failing = test_broader_categories()
    if failing == 0:
        print(f"\n🎉 All {passing} components from broader categories are working!")
    else:
        print(f"\n📊 Progress: {passing} working, {failing} need attention")
        print("This gives us a good sample of what still needs fixing!")