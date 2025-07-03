# zkVM Compiler Subagents

This directory contains specialized subagents that assist in the zkVM compilation process. Each subagent handles a specific aspect of generating Tau constraints for the TauFoldZKVM.

## ISA Generator (`isa_generator.py`)

The ISA Generator is responsible for creating compositional contracts for all zkVM instructions. It decomposes complex 32-bit operations into nibble-sized (4-bit) components that respect Tau's 800 character constraint limit.

### Key Features

1. **Compositional Design**: Each 32-bit instruction is broken down into:
   - 8 nibble operations (4 bits each)
   - 7 carry/borrow contracts linking nibbles
   - Result aggregation contracts (for comparisons)

2. **Assume-Guarantee Contracts**: Each component uses formal contracts with:
   - Assumptions (preconditions)
   - Guarantees (postconditions)
   - Explicit variable sets
   - Tau constraints under 700 characters

3. **Supported Instructions**:
   - **Arithmetic**: ADD, SUB, MUL, DIV, MOD
   - **Bitwise**: AND, OR, XOR, NOT, SHL, SHR
   - **Comparison**: EQ, NEQ, LT, GT, LTE, GTE
   - **Control Flow**: JMP, JZ, JNZ, CALL, RET
   - **Cryptographic**: HASH, VERIFY, SIGN

### Usage

```python
from subagents.isa_generator import ISAGenerator

# Create generator
generator = ISAGenerator(output_dir="build/isa")

# Generate instructions
results = generator.generate(["ADD", "SUB", "JMP"])

# Process results
for inst, result in results.items():
    print(f"{inst}: {result.total_constraints} constraints")
    
    # Save generated files
    generator.save_instruction_files(result)
```

### Architecture

The generator follows a hierarchical pattern:

1. **Main generate() method**: Routes instructions to specific generators
2. **Type-specific generators**: Handle each instruction category
3. **Component generators**: Create individual nibble operations
4. **Contract builders**: Ensure formal correctness

### Example: ADD Instruction

The 32-bit ADD instruction generates:
- 8 nibble adder components (one per 4-bit chunk)
- 7 carry propagation contracts
- Total: 15 components, each under 700 characters

Each nibble adder implements:
```tau
# Nibble 0 adder (bits 0-3)
solve a0_0=0 && a0_1=0 && a0_2=0 && a0_3=0 &&
      b0_0=0 && b0_1=0 && b0_2=0 && b0_3=0 &&
      s0_0=(a0_0+b0_0) && c0_0=(a0_0&b0_0) &&
      s0_1=(a0_1+b0_1+c0_0) && c0_1=((a0_1&b0_1)|((a0_1+b0_1)&c0_0)) &&
      s0_2=(a0_2+b0_2+c0_1) && c0_2=((a0_2&b0_2)|((a0_2+b0_2)&c0_1)) &&
      s0_3=(a0_3+b0_3+c0_2) && c0_3=((a0_3&b0_3)|((a0_3+b0_3)&c0_2)) &&
      cout0=c0_3
quit
```

### Testing

Run the test suite to verify all instructions:

```bash
python test_isa_generator.py
```

This will:
1. Generate all supported instructions
2. Verify constraint character limits
3. Create detailed documentation
4. Save test results to `build/isa_test/`

### Integration Points

The ISA generator integrates with:
- **Contract Framework**: Uses assume-guarantee reasoning
- **Tau Compiler**: Generates valid Tau constraint files
- **zkVM Verifier**: Ensures instruction correctness
- **Folding Module**: Compatible with ProtoStar accumulation

### Future Subagents

Planned subagents for the compiler:
- **Memory Manager**: Handle zkVM memory model
- **Folding Orchestrator**: Coordinate ProtoStar folding
- **Optimization Engine**: Reduce constraint counts
- **Verification Generator**: Create proof obligations