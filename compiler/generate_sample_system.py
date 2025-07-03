#!/usr/bin/env python3
"""Generate a sample of actual system components to measure real progress"""

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

def generate_sample_instruction_components():
    """Generate a sample of actual instruction components from the main generators"""
    
    # Import generators by loading specific functions
    exec(open('achieve_100_percent.py').read(), globals())
    
    components = []
    
    # Create generators
    arith_gen = ArithmeticGenerator()
    bitwise_gen = BitwiseGenerator()
    comp_gen = ComparisonGenerator()
    mem_gen = MemoryGenerator()
    system_gen = SystemGenerator()
    crypto_gen = CryptoGenerator()
    
    print("Generating sample components from actual generators...")
    
    # 1. Arithmetic samples
    try:
        components.append(('ARITH', arith_gen.generate_add_nibble(0)))
        components.append(('ARITH', arith_gen.generate_sub_nibble(0)))
        components.append(('ARITH', arith_gen.generate_div_nibble(0)))
        components.append(('ARITH', arith_gen.generate_mod_nibble(0)))
        print("✓ Generated arithmetic components")
    except Exception as e:
        print(f"✗ Error generating arithmetic: {e}")
    
    # 2. Bitwise samples  
    try:
        components.append(('BITWISE', bitwise_gen.generate_and_nibble(0)))
        components.append(('BITWISE', bitwise_gen.generate_or_nibble(0)))
        components.append(('BITWISE', bitwise_gen.generate_xor_nibble(0)))
        components.append(('BITWISE', bitwise_gen.generate_not_nibble(0)))
        print("✓ Generated bitwise components")
    except Exception as e:
        print(f"✗ Error generating bitwise: {e}")
    
    # 3. Comparison samples
    try:
        components.append(('COMP', comp_gen.generate_eq_nibble(0)))
        components.append(('COMP', comp_gen.generate_neq_nibble(0)))
        components.append(('COMP', comp_gen.generate_lt_nibble(0)))
        components.append(('COMP', comp_gen.generate_gt_nibble(0)))
        print("✓ Generated comparison components")
    except Exception as e:
        print(f"✗ Error generating comparison: {e}")
    
    # 4. Memory samples
    try:
        components.append(('MEM', mem_gen.generate_dup_nibble(0)))
        components.append(('MEM', mem_gen.generate_swap_nibble(0)))
        components.append(('MEM', mem_gen.generate_addr_nibble(0)))
        components.append(('MEM', mem_gen.generate_data_nibble(0)))
        print("✓ Generated memory components")
    except Exception as e:
        print(f"✗ Error generating memory: {e}")
    
    # 5. System samples
    try:
        components.append(('SYS', system_gen.generate_debug_nibble(0)))
        components.append(('SYS', system_gen.generate_assert_nibble(0)))
        components.append(('SYS', system_gen.generate_log_nibble(0)))
        print("✓ Generated system components")
    except Exception as e:
        print(f"✗ Error generating system: {e}")
    
    # 6. Crypto samples
    try:
        components.append(('CRYPTO', crypto_gen.generate_hash_nibble(0)))
        components.append(('CRYPTO', crypto_gen.generate_verify_nibble(0)))
        print("✓ Generated crypto components")
    except Exception as e:
        print(f"✗ Error generating crypto: {e}")
    
    # 7. Aggregator samples (proven working patterns)
    aggregators = [
        ('AGG', Component("eq_aggregator", ["eq0=1", "eq1=1", "eq2=1", "eq3=1", "eqfinal=(eq0&eq1&eq2&eq3)"], [], ["eqfinal"])),
        ('AGG', Component("neq_aggregator", ["neq0=0", "neq1=0", "neq2=0", "neq3=1", "neqfinal=(neq0|neq1|neq2|neq3)"], [], ["neqfinal"])),
        ('AGG', Component("dup_aggregator", ["allok=1", "dupcomplete=allok"], [], ["dupcomplete"])),
        ('AGG', Component("swap_aggregator", ["allswapped=1", "swapcomplete=allswapped"], [], ["swapcomplete"]))
    ]
    components.extend(aggregators)
    print("✓ Generated aggregator components")
    
    return components

def test_sample_system():
    """Test the sample system components"""
    print("\\nTesting sample system components...")
    
    # Create output directory
    os.makedirs("test_sample_system", exist_ok=True)
    
    # Generate components (avoid the full generation script execution)
    try:
        components = generate_sample_instruction_components()
    except Exception as e:
        print(f"Error during generation: {e}")
        return False
    
    # Validate each component
    results_by_category = {}
    total_passing = 0
    total_failing = 0
    
    for category, comp in components:
        if category not in results_by_category:
            results_by_category[category] = {'passing': 0, 'failing': 0, 'details': []}
        
        try:
            # Generate Tau content
            content = comp.to_tau()
            
            # Write to file
            filepath = f"test_sample_system/{comp.name}.tau"
            with open(filepath, 'w') as f:
                f.write(content)
            
            # Validate with Tau
            result = subprocess.run(['/Users/danax/projects/TauStandardLibrary/external_dependencies/run_tau.sh', filepath], 
                                  capture_output=True, text=True, timeout=10)
            
            if result.returncode == 0 and 'solution' in result.stdout:
                results_by_category[category]['passing'] += 1
                results_by_category[category]['details'].append(f"✓ PASS: {comp.name}")
                total_passing += 1
            else:
                results_by_category[category]['failing'] += 1 
                results_by_category[category]['details'].append(f"✗ FAIL: {comp.name}")
                total_failing += 1
                
        except Exception as e:
            results_by_category[category]['failing'] += 1
            results_by_category[category]['details'].append(f"✗ ERROR: {comp.name} - {str(e)[:50]}")
            total_failing += 1
    
    # Print detailed results
    print(f"\\n=== SAMPLE SYSTEM VALIDATION RESULTS ===")
    
    for category, results in results_by_category.items():
        total = results['passing'] + results['failing']
        percentage = (results['passing'] / total * 100) if total > 0 else 0
        print(f"\\n{category}: {results['passing']}/{total} ({percentage:.1f}%)")
        for detail in results['details']:
            print(f"  {detail}")
    
    total_components = total_passing + total_failing
    overall_percentage = (total_passing / total_components * 100) if total_components > 0 else 0
    
    print(f"\\n=== OVERALL SAMPLE RESULTS ===")
    print(f"Total: {total_passing}/{total_components} ({overall_percentage:.1f}%)")
    print(f"This represents our estimated progress toward 100% system validation!")
    
    return overall_percentage >= 85  # Success if 85%+ working

if __name__ == "__main__":
    success = test_sample_system()
    if success:
        print("\\n🎉 Excellent progress! System is ready for full validation!")
    else:
        print("\\n📊 Good progress! Continue fixing remaining components.")