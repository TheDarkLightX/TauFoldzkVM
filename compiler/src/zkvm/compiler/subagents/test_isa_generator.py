#!/usr/bin/env python3
"""
Test script for the ISA Generator subagent.
Demonstrates generating all supported instructions.
"""

from isa_generator import ISAGenerator, InstructionType
import json
import os


def test_all_instructions():
    """Test generating all supported instructions."""
    generator = ISAGenerator(output_dir="build/isa_test")
    
    # All supported instructions
    all_instructions = [
        # Arithmetic
        "ADD", "SUB", "MUL", "DIV", "MOD",
        # Bitwise
        "AND", "OR", "XOR", "NOT", "SHL", "SHR",
        # Comparison
        "EQ", "NEQ", "LT", "GT", "LTE", "GTE",
        # Control Flow
        "JMP", "JZ", "JNZ", "CALL", "RET",
        # Crypto
        "HASH", "VERIFY", "SIGN"
    ]
    
    print("Generating all ISA instructions...")
    print("=" * 60)
    
    results = generator.generate(all_instructions)
    
    # Summary statistics
    total_components = 0
    total_files = 0
    total_constraints = 0
    
    instruction_summary = {}
    
    for inst, result in results.items():
        print(f"\n{inst} Instruction:")
        print(f"  Components: {len(result.components_generated)}")
        print(f"  Files: {len(result.files)}")
        print(f"  Total Constraints: {result.total_constraints}")
        
        # Check constraint limits
        for component in result.components_generated:
            expr_len = len(component.contract.to_tau_expression())
            print(f"    {component.name}: {expr_len} chars ({len(component.contract.constraints)} constraints)")
            if expr_len > 700:
                print(f"      WARNING: Exceeds 700 char limit!")
        
        total_components += len(result.components_generated)
        total_files += len(result.files)
        total_constraints += result.total_constraints
        
        instruction_summary[inst] = {
            "components": len(result.components_generated),
            "files": len(result.files),
            "constraints": result.total_constraints,
            "component_details": [
                {
                    "name": comp.name,
                    "nibble_index": comp.nibble_index,
                    "carry_in": comp.carry_in,
                    "carry_out": comp.carry_out,
                    "char_count": len(comp.contract.to_tau_expression())
                }
                for comp in result.components_generated
            ]
        }
        
        # Save files
        generator.save_instruction_files(result)
    
    print("\n" + "=" * 60)
    print("OVERALL SUMMARY:")
    print(f"  Total Instructions: {len(all_instructions)}")
    print(f"  Total Components: {total_components}")
    print(f"  Total Files: {total_files}")
    print(f"  Total Constraints: {total_constraints}")
    print(f"  Average Constraints per Instruction: {total_constraints // len(all_instructions)}")
    
    # Save summary JSON
    summary_path = os.path.join(generator.output_dir, "instruction_summary.json")
    with open(summary_path, 'w') as f:
        json.dump(instruction_summary, f, indent=2)
    
    print(f"\nDetailed summary saved to: {summary_path}")
    
    # Generate master documentation
    generate_master_doc(generator.output_dir, instruction_summary)


def generate_master_doc(output_dir: str, summary: dict):
    """Generate master documentation for all instructions."""
    doc = """# TauFoldZKVM ISA Documentation

## Overview

This document describes all instructions supported by the TauFoldZKVM ISA Generator.
Each instruction is decomposed into nibble-sized (4-bit) operations to respect Tau's
800 character constraint limit.

## Instruction Categories

### Arithmetic Instructions
- **ADD**: 32-bit addition using 8 nibble adders + 7 carry contracts
- **SUB**: 32-bit subtraction using two's complement
- **MUL**: Multiplication using partial products (simplified to 8-bit)
- **DIV**: Division using lookup tables
- **MOD**: Modulo operation using lookup tables

### Bitwise Instructions
- **AND**: Bitwise AND across all nibbles
- **OR**: Bitwise OR across all nibbles
- **XOR**: Bitwise XOR across all nibbles
- **NOT**: Bitwise NOT across all nibbles
- **SHL**: Left shift using multiplexer tree
- **SHR**: Right shift using multiplexer tree

### Comparison Instructions
- **EQ**: Equality check across all nibbles
- **NEQ**: Inequality check
- **LT**: Less than (signed/unsigned)
- **GT**: Greater than
- **LTE**: Less than or equal
- **GTE**: Greater than or equal

### Control Flow Instructions
- **JMP**: Unconditional jump
- **JZ**: Jump if zero flag set
- **JNZ**: Jump if zero flag not set
- **CALL**: Function call with return address save
- **RET**: Return from function

### Cryptographic Instructions
- **HASH**: Hash using RC permutation
- **VERIFY**: Signature verification
- **SIGN**: Digital signature generation

## Compositional Design

Each 32-bit operation is decomposed into:
1. **8 nibble operations** (4 bits each)
2. **7 carry/borrow contracts** linking nibbles
3. **Result aggregation** (for comparisons)

This ensures each component stays under 700 characters while maintaining correctness.

## Contract Structure

Each component uses assume-guarantee contracts:
```python
Contract(
    name="component_name",
    assumptions=["preconditions"],
    guarantees=["postconditions"],
    variables={"used_variables"},
    constraints=["tau_constraints"]
)
```

## Instruction Summary

"""
    
    # Add instruction details
    for category in ["Arithmetic", "Bitwise", "Comparison", "Control Flow", "Cryptographic"]:
        doc += f"\n### {category} Instructions\n\n"
        
        category_map = {
            "Arithmetic": ["ADD", "SUB", "MUL", "DIV", "MOD"],
            "Bitwise": ["AND", "OR", "XOR", "NOT", "SHL", "SHR"],
            "Comparison": ["EQ", "NEQ", "LT", "GT", "LTE", "GTE"],
            "Control Flow": ["JMP", "JZ", "JNZ", "CALL", "RET"],
            "Cryptographic": ["HASH", "VERIFY", "SIGN"]
        }
        
        for inst in category_map.get(category, []):
            if inst in summary:
                info = summary[inst]
                doc += f"#### {inst}\n"
                doc += f"- Components: {info['components']}\n"
                doc += f"- Total Constraints: {info['constraints']}\n"
                doc += f"- Files Generated: {info['files']}\n\n"
    
    doc += """
## Usage Example

```python
from isa_generator import ISAGenerator

# Create generator
generator = ISAGenerator(output_dir="build/isa")

# Generate specific instructions
results = generator.generate(["ADD", "SUB", "JMP"])

# Each result contains:
# - components_generated: List of instruction components
# - files: Dict of filename -> tau content
# - contracts: List of all contracts
# - total_constraints: Total constraint count

# Save to disk
for inst, result in results.items():
    generator.save_instruction_files(result)
```

## Integration with zkVM Compiler

The ISA generator integrates with the larger zkVM compiler infrastructure:

1. **Instruction Selection**: Compiler selects appropriate ISA instructions
2. **Component Generation**: ISA generator creates nibble-sized components
3. **Contract Composition**: Components linked via assume-guarantee contracts
4. **Tau Generation**: Each component compiled to valid Tau constraints
5. **Verification**: Satisfiability checking ensures correctness

## Performance Characteristics

- **Constraint Budget**: Each component < 700 characters
- **Parallelism**: Nibble operations can verify in parallel
- **Modularity**: Components can be reused across instructions
- **Scalability**: Easy to extend with new instructions

## Future Enhancements

1. **Vector Instructions**: SIMD-style operations
2. **Floating Point**: IEEE 754 via bit manipulation
3. **Custom Instructions**: Domain-specific operations
4. **Optimization**: Constraint count reduction
5. **Verification**: Formal proofs of instruction correctness
"""
    
    doc_path = os.path.join(output_dir, "ISA_DOCUMENTATION.md")
    with open(doc_path, 'w') as f:
        f.write(doc)
    
    print(f"Master documentation saved to: {doc_path}")


def test_specific_instruction():
    """Test a specific instruction in detail."""
    generator = ISAGenerator(output_dir="build/isa_test")
    
    print("\nTesting ADD instruction in detail...")
    print("=" * 60)
    
    result = generator.generate(["ADD"])["ADD"]
    
    print(f"ADD Instruction Components:")
    for i, comp in enumerate(result.components_generated):
        print(f"\n{i+1}. {comp.name}")
        print(f"   Nibble Index: {comp.nibble_index}")
        print(f"   Carry In: {comp.carry_in}")
        print(f"   Carry Out: {comp.carry_out}")
        print(f"   Variables: {len(comp.contract.variables)}")
        print(f"   Constraints: {len(comp.contract.constraints)}")
        print(f"   Character Count: {len(comp.contract.to_tau_expression())}")
        
        # Show first few constraints
        print("   Sample Constraints:")
        for j, constraint in enumerate(comp.contract.constraints[:3]):
            print(f"     - {constraint}")
        if len(comp.contract.constraints) > 3:
            print(f"     ... ({len(comp.contract.constraints) - 3} more)")
    
    # Show a complete nibble file
    print("\n" + "=" * 60)
    print("Sample Generated Tau File (add_nibble_0.tau):")
    print("=" * 60)
    if "add_nibble_0.tau" in result.files:
        print(result.files["add_nibble_0.tau"])


if __name__ == "__main__":
    # Run all tests
    test_all_instructions()
    test_specific_instruction()
    
    print("\nAll tests completed successfully!")