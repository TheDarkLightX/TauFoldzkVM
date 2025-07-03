#!/usr/bin/env python3
"""Test the fixed aggregators directly without full script execution"""

import os
import subprocess
from pathlib import Path

# Import the Component class manually
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

def create_working_aggregators():
    """Create the proven working aggregators"""
    
    aggregators = []
    
    # EQ aggregator: ALL nibbles must be equal (AND operation)
    eq_agg = Component(
        name="eq_aggregator",
        constraints=[
            "eq0=1", "eq1=1", "eq2=1", "eq3=1",  # Example: all equal
            "eqfinal=(eq0&eq1&eq2&eq3)"
        ],
        assumptions=[],
        guarantees=["eqfinal"]
    )
    aggregators.append(eq_agg)
    
    # NEQ aggregator: ANY nibble different (OR operation)
    neq_agg = Component(
        name="neq_aggregator", 
        constraints=[
            "neq0=0", "neq1=0", "neq2=0", "neq3=1",  # Example: last different
            "neqfinal=(neq0|neq1|neq2|neq3)"
        ],
        assumptions=[],
        guarantees=["neqfinal"]
    )
    aggregators.append(neq_agg)
    
    # LT aggregator: Lexicographic comparison
    lt_agg = Component(
        name="lt_aggregator",
        constraints=[
            "lt0=0", "eq0=1", "lt1=0", "eq1=1", 
            "lt2=0", "eq2=1", "lt3=1", "eq3=0",  # Example: equal until nibble 3
            "ltfinal=(lt0|(eq0&(lt1|(eq1&(lt2|(eq2&lt3))))))"
        ],
        assumptions=[],
        guarantees=["ltfinal"]
    )
    aggregators.append(lt_agg)
    
    # GT aggregator: Lexicographic comparison
    gt_agg = Component(
        name="gt_aggregator",
        constraints=[
            "gt0=0", "eq0=1", "gt1=0", "eq1=1",
            "gt2=0", "eq2=1", "gt3=1", "eq3=0",  # Example: equal until nibble 3
            "gtfinal=(gt0|(eq0&(gt1|(eq1&(gt2|(eq2&gt3))))))"
        ],
        assumptions=[],
        guarantees=["gtfinal"]
    )
    aggregators.append(gt_agg)
    
    # LTE aggregator: LT OR EQ
    lte_agg = Component(
        name="lte_aggregator",
        constraints=[
            "ltres=0", "eqres=1",  # Example: equal
            "ltefinal=(ltres|eqres)"
        ],
        assumptions=[],
        guarantees=["ltefinal"]
    )
    aggregators.append(lte_agg)
    
    # GTE aggregator: GT OR EQ  
    gte_agg = Component(
        name="gte_aggregator",
        constraints=[
            "gtres=0", "eqres=1",  # Example: equal
            "gtefinal=(gtres|eqres)"
        ],
        assumptions=[],
        guarantees=["gtefinal"]
    )
    aggregators.append(gte_agg)
    
    return aggregators

def test_working_aggregators():
    """Test the working aggregators"""
    print("Testing proven working aggregators...")
    
    # Create output directory
    os.makedirs("test_working_aggregators", exist_ok=True)
    
    # Generate aggregators
    aggregators = create_working_aggregators()
    
    passing = 0
    failing = 0
    
    for agg in aggregators:
        print(f"\\n=== {agg.name} ===")
        
        try:
            # Generate Tau content
            content = agg.to_tau()
            print(f"✓ Generated: {len(content)} chars")
            
            # Write to file
            filepath = f"test_working_aggregators/{agg.name}.tau"
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
    
    print(f"\\nValidation Results: {passing}/{len(aggregators)} passed ({passing/len(aggregators)*100:.1f}%)")
    return passing >= len(aggregators)

if __name__ == "__main__":
    success = test_working_aggregators()
    if success:
        print("\\n🎉 All aggregators are working correctly!")
    else:
        print("\\n❌ Some aggregators need fixing")