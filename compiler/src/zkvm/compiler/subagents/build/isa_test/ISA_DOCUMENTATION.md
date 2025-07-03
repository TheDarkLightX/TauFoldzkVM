# TauFoldZKVM ISA Documentation

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


### Arithmetic Instructions

#### ADD
- Components: 15
- Total Constraints: 164
- Files Generated: 15

#### SUB
- Components: 15
- Total Constraints: 204
- Files Generated: 15

#### MUL
- Components: 1
- Total Constraints: 36
- Files Generated: 1

#### DIV
- Components: 1
- Total Constraints: 25
- Files Generated: 1

#### MOD
- Components: 1
- Total Constraints: 25
- Files Generated: 1


### Bitwise Instructions

#### AND
- Components: 8
- Total Constraints: 96
- Files Generated: 8

#### OR
- Components: 8
- Total Constraints: 96
- Files Generated: 8

#### XOR
- Components: 8
- Total Constraints: 96
- Files Generated: 8

#### NOT
- Components: 8
- Total Constraints: 64
- Files Generated: 8

#### SHL
- Components: 1
- Total Constraints: 21
- Files Generated: 1

#### SHR
- Components: 1
- Total Constraints: 21
- Files Generated: 1


### Comparison Instructions

#### EQ
- Components: 9
- Total Constraints: 130
- Files Generated: 9

#### NEQ
- Components: 9
- Total Constraints: 130
- Files Generated: 9

#### LT
- Components: 9
- Total Constraints: 130
- Files Generated: 9

#### GT
- Components: 9
- Total Constraints: 130
- Files Generated: 9

#### LTE
- Components: 9
- Total Constraints: 130
- Files Generated: 9

#### GTE
- Components: 9
- Total Constraints: 130
- Files Generated: 9


### Control Flow Instructions

#### JMP
- Components: 1
- Total Constraints: 24
- Files Generated: 1

#### JZ
- Components: 1
- Total Constraints: 25
- Files Generated: 1

#### JNZ
- Components: 1
- Total Constraints: 25
- Files Generated: 1

#### CALL
- Components: 1
- Total Constraints: 41
- Files Generated: 1

#### RET
- Components: 1
- Total Constraints: 32
- Files Generated: 1


### Cryptographic Instructions

#### HASH
- Components: 1
- Total Constraints: 40
- Files Generated: 1

#### VERIFY
- Components: 1
- Total Constraints: 33
- Files Generated: 1

#### SIGN
- Components: 1
- Total Constraints: 25
- Files Generated: 1


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
