#!/usr/bin/env python3
"""Test aggregator method access"""

import sys
from pathlib import Path

# Add current directory to path
sys.path.insert(0, str(Path(__file__).parent))

# Import classes from the main file
exec(open('achieve_100_percent.py').read())

def test_aggregator_access():
    """Test if aggregator methods are accessible"""
    
    # Create generator instance
    gen = ArithmeticGenerator()
    
    print("Testing aggregator method access...")
    
    # Test each aggregator method
    methods = [
        'generate_eq_aggregator',
        'generate_neq_aggregator', 
        'generate_lt_aggregator',
        'generate_gt_aggregator',
        'generate_lte_aggregator',
        'generate_gte_aggregator'
    ]
    
    for method_name in methods:
        try:
            method = getattr(gen, method_name)
            comp = method()
            print(f"✓ {method_name}: {comp.name}")
        except AttributeError:
            print(f"✗ {method_name}: Method not found")
        except Exception as e:
            print(f"✗ {method_name}: Error - {e}")

if __name__ == "__main__":
    test_aggregator_access()