#!/usr/bin/env python3
"""Test the fixed aggregators directly"""

import os
import sys
from pathlib import Path

# Add current directory to path
sys.path.insert(0, str(Path(__file__).parent))

from achieve_100_percent import Component

def create_fixed_aggregators():
    """Create the fixed aggregators using Boolean algebra"""
    
    aggregators = []
    
    # EQ aggregator: ALL nibbles must be equal (AND operation)
    eq_agg = Component(
        name="eq_aggregator",
        constraints=[
            "eq0=1", "eq1=1", "eq2=1", "eq3=1",  # Example: all equal
            "eq_final=(eq0&eq1&eq2&eq3)"
        ],
        assumptions=[],
        guarantees=["eq_final"]
    )
    aggregators.append(eq_agg)
    
    # NEQ aggregator: ANY nibble different (OR operation)
    neq_agg = Component(
        name="neq_aggregator", 
        constraints=[
            "neq0=0", "neq1=0", "neq2=0", "neq3=1",  # Example: last different
            "neq_final=(neq0|neq1|neq2|neq3)"
        ],
        assumptions=[],
        guarantees=["neq_final"]
    )
    aggregators.append(neq_agg)
    
    # LT aggregator: Lexicographic comparison (MSB priority)
    lt_agg = Component(
        name="lt_aggregator",
        constraints=[
            "lt0=0", "eq0=1", "lt1=0", "eq1=1", 
            "lt2=0", "eq2=1", "lt3=1", "eq3=0",  # Example: equal until nibble 3
            "lt_final=(lt0|(eq0&(lt1|(eq1&(lt2|(eq2&lt3))))))"
        ],
        assumptions=[],
        guarantees=["lt_final"]
    )
    aggregators.append(lt_agg)
    
    # GT aggregator: Lexicographic comparison (MSB priority)
    gt_agg = Component(
        name="gt_aggregator",
        constraints=[
            "gt0=0", "eq0=1", "gt1=0", "eq1=1",
            "gt2=0", "eq2=1", "gt3=1", "eq3=0",  # Example: equal until nibble 3
            "gt_final=(gt0|(eq0&(gt1|(eq1&(gt2|(eq2&gt3))))))"
        ],
        assumptions=[],
        guarantees=["gt_final"]
    )
    aggregators.append(gt_agg)
    
    # LTE aggregator: LT OR EQ
    lte_agg = Component(
        name="lte_aggregator",
        constraints=[
            "lt_result=0", "eq_result=1",  # Example: equal
            "lte_final=(lt_result|eq_result)"
        ],
        assumptions=[],
        guarantees=["lte_final"]
    )
    aggregators.append(lte_agg)
    
    # GTE aggregator: GT OR EQ  
    gte_agg = Component(
        name="gte_aggregator",
        constraints=[
            "gt_result=0", "eq_result=1",  # Example: equal
            "gte_final=(gt_result|eq_result)"
        ],
        assumptions=[],
        guarantees=["gte_final"]
    )
    aggregators.append(gte_agg)
    
    return aggregators

def test_aggregators():
    """Test the fixed aggregators"""
    print("Testing fixed aggregators...")
    
    # Create output directory
    os.makedirs("test_aggregators", exist_ok=True)
    
    # Generate aggregators
    aggregators = create_fixed_aggregators()
    
    # Test each aggregator
    for agg in aggregators:
        print(f"\n=== {agg.name} ===")
        
        # Generate Tau content
        try:
            content = agg.to_tau()
            print(f"✓ Generated: {len(content)} chars")
            
            # Write to file
            filepath = f"test_aggregators/{agg.name}.tau"
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"✓ Saved: {filepath}")
            
            # Show content
            print(f"Content:\n{content}")
            
        except Exception as e:
            print(f"✗ Error: {e}")
    
    print(f"\nGenerated {len(aggregators)} aggregators")

if __name__ == "__main__":
    test_aggregators()