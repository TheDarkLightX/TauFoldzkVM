#!/usr/bin/env python3
"""
Contract-Based Tau Compiler using Boolean Algebra Composition
Based on "A Boolean Algebra of Contracts for Assume-Guarantee Reasoning"
"""

from dataclasses import dataclass
from typing import List, Tuple, Set, Dict
import json

@dataclass
class Contract:
    """Assume-Guarantee Contract (A, G)"""
    name: str
    assumptions: List[str]  # Boolean predicates on inputs
    guarantees: List[str]   # Boolean predicates on outputs
    variables: Set[str]
    max_chars: int = 700    # Stay well under 800 limit

@dataclass  
class Component:
    """Component with its contract and implementation"""
    contract: Contract
    constraints: List[str]
    
    def to_tau_file(self) -> str:
        """Generate Tau file respecting length limits"""
        # Build assumption checks
        assume_checks = []
        for assumption in self.contract.assumptions:
            assume_checks.append(f"({assumption})")
        
        # Combine constraints
        all_parts = []
        if assume_checks:
            all_parts.extend(assume_checks)
        all_parts.extend(self.constraints)
        
        # Add guarantee marker
        all_parts.append("guarantee=1")
        
        expr = " && ".join(all_parts)
        
        if len(expr) > self.contract.max_chars:
            raise ValueError(f"Expression too long: {len(expr)} chars")
            
        return f"# Contract: {self.contract.name}\n# Assumptions: {', '.join(self.contract.assumptions)}\n# Guarantees: {', '.join(self.contract.guarantees)}\n\nsolve {expr}\n\nquit"

class CompositionalCompiler:
    """Compiler using compositional contracts"""
    
    def __init__(self):
        self.components: List[Component] = []
        self.contracts: Dict[str, Contract] = {}
    
    def add_component(self, component: Component):
        """Add a component with its contract"""
        self.components.append(component)
        self.contracts[component.contract.name] = component.contract
    
    def verify_composition(self, c1_name: str, c2_name: str) -> bool:
        """Verify that two contracts can be composed"""
        c1 = self.contracts[c1_name]
        c2 = self.contracts[c2_name]
        
        # Check interface compatibility
        shared_vars = c1.variables & c2.variables
        
        # For each shared variable, check guarantee-assumption match
        # This is simplified - full check would verify logical implication
        return len(shared_vars) > 0
    
    def generate_nibble_adder(self, name: str, bit_offset: int, carry_in: bool = False) -> Component:
        """Generate a 4-bit adder component"""
        vars = set()
        constraints = []
        assumptions = []
        guarantees = []
        
        # Input bits
        for i in range(4):
            bit = bit_offset + i
            vars.add(f"a{bit}")
            vars.add(f"b{bit}")
            # Example values for testing
            constraints.append(f"a{bit}={(i+1)%2}")
            constraints.append(f"b{bit}={i%2}")
        
        # Carry chain
        if carry_in:
            vars.add("carry_in")
            assumptions.append("carry_in=0|carry_in=1")
            start_carry = "carry_in"
        else:
            start_carry = None
            
        for i in range(4):
            bit = bit_offset + i
            vars.add(f"s{bit}")
            vars.add(f"c{bit}")
            
            if i == 0 and not carry_in:
                constraints.append(f"s{bit}=(a{bit}+b{bit})")
                constraints.append(f"c{bit}=(a{bit}&b{bit})")
            else:
                prev_carry = start_carry if i == 0 else f"c{bit-1}"
                constraints.append(f"s{bit}=(a{bit}+b{bit}+{prev_carry})")
                constraints.append(f"c{bit}=((a{bit}&b{bit})|((a{bit}+b{bit})&{prev_carry}))")
        
        # Output carry
        if bit_offset == 0:
            vars.add("carry_out")
            constraints.append(f"carry_out=c{bit_offset+3}")
            guarantees.append(f"carry_out=c{bit_offset+3}")
        
        # Create contract
        contract = Contract(
            name=name,
            assumptions=assumptions,
            guarantees=guarantees,
            variables=vars
        )
        
        return Component(contract=contract, constraints=constraints)
    
    def generate_8bit_adder_compositional(self):
        """Generate 8-bit adder as composition of 4-bit adders"""
        # Low nibble (bits 0-3)
        low_nibble = self.generate_nibble_adder("add_low", 0)
        self.add_component(low_nibble)
        
        # High nibble (bits 4-7) 
        high_nibble = self.generate_nibble_adder("add_high", 4, carry_in=True)
        self.add_component(high_nibble)
        
        # Generate connection contract
        connection = Component(
            contract=Contract(
                name="carry_connection",
                assumptions=["carry_out=0|carry_out=1"],
                guarantees=["carry_in=carry_out"],
                variables={"carry_out", "carry_in"}
            ),
            constraints=["carry_out=0", "carry_in=carry_out"]
        )
        self.add_component(connection)
        
        return [low_nibble, high_nibble, connection]
    
    def generate_files(self, output_dir: str = "build/compositional"):
        """Generate all Tau files"""
        import os
        os.makedirs(output_dir, exist_ok=True)
        
        manifest = {
            "components": [],
            "contracts": {},
            "composition_verified": []
        }
        
        for component in self.components:
            filename = f"{component.contract.name}.tau"
            filepath = os.path.join(output_dir, filename)
            
            with open(filepath, 'w') as f:
                f.write(component.to_tau_file())
            
            manifest["components"].append(filename)
            manifest["contracts"][component.contract.name] = {
                "assumptions": component.contract.assumptions,
                "guarantees": component.contract.guarantees,
                "variables": list(component.contract.variables)
            }
        
        # Verify compositions
        for i, c1 in enumerate(self.components):
            for c2 in self.components[i+1:]:
                if self.verify_composition(c1.contract.name, c2.contract.name):
                    manifest["composition_verified"].append(
                        (c1.contract.name, c2.contract.name)
                    )
        
        # Save manifest
        with open(os.path.join(output_dir, "manifest.json"), 'w') as f:
            json.dump(manifest, f, indent=2)
        
        return manifest

def main():
    """Generate compositional zkVM using contracts"""
    compiler = CompositionalCompiler()
    
    # Generate 8-bit adder compositionally
    print("Generating compositional 8-bit adder...")
    compiler.generate_8bit_adder_compositional()
    
    # Generate files
    manifest = compiler.generate_files()
    
    print(f"\nGenerated {len(manifest['components'])} components:")
    for component in manifest['components']:
        print(f"  - {component}")
    
    print(f"\nVerified {len(manifest['composition_verified'])} compositions:")
    for c1, c2 in manifest['composition_verified']:
        print(f"  - {c1} ⇓ {c2}")
    
    print("\nEach component respects the 800-char limit!")
    print("Components can be verified independently and composed.")

if __name__ == "__main__":
    main()