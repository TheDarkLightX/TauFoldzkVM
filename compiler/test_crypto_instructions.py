#!/usr/bin/env python3
"""Test crypto instructions generation for TauFoldZKVM"""

import sys
sys.path.append('.')
from achieve_100_percent import generate_instruction_components, Component

def test_crypto_instructions():
    """Test that crypto instructions generate valid components"""
    
    crypto_instructions = ["HASH", "VERIFY", "SIGN"]
    
    for instruction in crypto_instructions:
        print(f"\nTesting {instruction}:")
        components = generate_instruction_components(instruction)
        
        print(f"  Generated {len(components)} components")
        
        # Check each component
        for comp in components:
            try:
                tau_content = comp.to_tau()
                expr_len = len(" && ".join(comp.constraints))
                print(f"  ✓ {comp.name}: {expr_len} chars")
                
                # Verify it's under the limit
                assert expr_len < 700, f"{comp.name} exceeds 700 char limit: {expr_len}"
                
            except Exception as e:
                print(f"  ✗ {comp.name}: {str(e)}")
                return False
    
    return True

if __name__ == "__main__":
    print("Testing Crypto Instruction Generation")
    print("="*40)
    
    if test_crypto_instructions():
        print("\n✅ All crypto instructions generated successfully!")
    else:
        print("\n❌ Some crypto instructions failed!")
        sys.exit(1)