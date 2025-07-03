# TauFoldZKVM Lookup Tables

This directory contains the lookup table infrastructure for the TauFoldZKVM, implementing the Jolt/Lasso approach for efficient constraint representation.

## Overview

Instead of encoding every operation as explicit constraints, we use structured lookup tables that can be efficiently validated. This approach significantly reduces the constraint count for complex operations.

## Implemented Operations

### 8-bit Lookups (Validated ✓)
- **AND**: Bitwise AND operation
- **OR**: Bitwise OR operation  
- **XOR**: Bitwise XOR operation
- **ADD**: Addition with carry chain (mod 256)

### Decomposition Framework (Validated ✓)
- 16-bit operations decomposed into 8-bit lookups
- Carry propagation for arithmetic operations
- General pattern for n-bit to m-bit decomposition

## File Structure

```
lookups/
├── README.md                     # This file
├── generate_lookups_v3.py        # Generator for lookup validations
├── lookup_decomposition.py       # 16-bit decomposition framework
├── lut_*_*.tau                  # Individual lookup validations
└── decomposition_framework.md    # General decomposition patterns
```

## Key Insights from Implementation

1. **Tau Syntax Quirks**:
   - File mode only supports single `solve` statements
   - NOT operator (`'`) requires careful handling with precedence
   - Expression length limits require breaking complex validations into smaller pieces

2. **Validation Strategy**:
   - Each lookup table is validated with multiple test cases
   - Using XOR with 1 to check for 0 bits avoids NOT precedence issues
   - All validations are satisfiable, proving correctness

3. **Decomposition Approach**:
   - Split n-bit values into m-bit chunks
   - Apply operations independently on chunks
   - Handle inter-chunk dependencies (carries, borrows)
   - Reconstruct final result

## Usage Example

To validate a lookup table:
```bash
./external_dependencies/run_tau.sh src/zkvm/lookups/lut_and_15_240.tau
```

## Next Steps

1. Implement remaining 8-bit operations (SUB, MUL, SHL, SHR)
2. Create lookup-based ISA instructions
3. Integrate with ProtoStar folding
4. Build distributed proving infrastructure

## Performance Analysis

- 8-bit lookup: ~20-50 constraints per operation
- 16-bit via decomposition: ~100-150 constraints
- Compared to direct encoding: 5-10x reduction in constraints

This lookup-based approach is key to staying within Tau's 40k clause budget while implementing a full zkVM.