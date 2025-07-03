#!/usr/bin/env python3
"""Debug what components are being generated"""

import sys
sys.path.append('.')

from subagents.isa_generator import ISAGenerator

# Generate ADD instruction
isa = ISAGenerator("build/debug")
result = isa.generate(["ADD"])["ADD"]

print(f"Total components: {len(result.components_generated)}")
print("\nComponent names:")
for i, comp in enumerate(result.components_generated):
    print(f"{i}: {comp.name}")
    
print(f"\nFirst component constraints ({len(result.components_generated[0].contract.constraints)} total):")
for j, constraint in enumerate(result.components_generated[0].contract.constraints[:5]):
    print(f"  {j}: {constraint}")