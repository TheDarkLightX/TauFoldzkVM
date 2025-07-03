# TauFoldZKVM Instruction Set Architecture

## Overview

8-instruction ISA mixing lookup-based and constraint-based operations:

| Opcode | Mnemonic | Description | Type |
|--------|----------|-------------|------|
| 000 | LUT8 | 8-bit lookup operation | Lookup |
| 001 | LUT16 | 16-bit decomposed lookup | Lookup |
| 010 | FOLD | Fold current state | Constraint |
| 011 | COMM | Commit to value | Constraint |
| 100 | LOAD | Memory read | Constraint |
| 101 | STORE | Memory write | Constraint |
| 110 | COND | Conditional execution | Constraint |
| 111 | HALT | Stop execution | Constraint |

## Instruction Formats

### LUT8/LUT16
```
LUT8  rd, rs1, rs2, table_id
LUT16 rd, rs1, rs2, table_id
```
- table_id selects operation (AND, OR, XOR, ADD, etc.)
- Result stored in rd

### FOLD
```
FOLD instance
```
- Folds current instance with accumulator
- Updates noise vector

### COMM
```
COMM rd, rs, randomness
```
- Commits to value using RC hash
- Stores commitment in rd

### Memory Operations
```
LOAD  rd, addr
STORE rs, addr
```
- Simple memory model
- Address space depends on configuration

### Control Flow
```
COND flag, target
HALT
```
- COND: Branch if flag is set
- HALT: Stop execution

## Implementation Files

1. **isa_decoder.tau** - Instruction decoding logic
2. **isa_lut8.tau** - LUT8 instruction execution
3. **isa_fold.tau** - FOLD instruction for ProtoStar
4. **isa_comm.tau** - Commitment instruction
5. **isa_memory.tau** - LOAD/STORE operations
6. **isa_trace.tau** - Example execution trace

## Constraint Analysis

- Instruction decode: ~20 constraints
- LUT8/16 operation: ~50 constraints (using lookups)
- FOLD operation: ~100 constraints
- Memory access: ~40 constraints
- Per instruction average: ~60 constraints

## Design Rationale

1. **Mixed approach**: Lookups for arithmetic, constraints for control
2. **ProtoStar native**: FOLD instruction for efficient accumulation
3. **Commitment built-in**: COMM for zkVM-specific operations
4. **Simple memory**: Constraint-based for flexibility
5. **Minimal ISA**: 8 instructions sufficient for universality
