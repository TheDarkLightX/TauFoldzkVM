# TauFoldZKVM System Instructions

This document describes the implementation of system instructions (NOP, HALT, DEBUG, ASSERT) for the TauFoldZKVM.

## Overview

System instructions provide essential control and debugging capabilities for the zkVM:

1. **NOP** - No operation, useful for padding and timing
2. **HALT** - Stop execution
3. **DEBUG** - Output debug information
4. **ASSERT** - Runtime assertion checking

## Implementation Details

### NOP (No Operation)

- **Components**: 1 single component
- **Behavior**: Maintains all state unchanged, only increments PC
- **Constraints**: ~17 constraints
- **Use cases**: Padding, timing alignment, placeholder

```tau
# Example NOP constraint
solve valid=1 && nop=1 && p0=0 && n0=0 && ... # PC unchanged
```

### HALT

- **Components**: 1 single component  
- **Behavior**: Sets halt flag, stops PC advancement
- **Constraints**: ~17 constraints
- **Use cases**: Program termination, error states

```tau
# Example HALT constraint
solve valid=1 && halt=1 && p0=0 && n0=p0 && ... # Next PC = Current PC
```

### DEBUG

- **Components**: 8 nibble components (for 32-bit output)
- **Behavior**: Outputs debug data through dedicated channels
- **Constraints**: ~9 per nibble (72 total)
- **Use cases**: Debugging, tracing, logging

```tau
# Example DEBUG nibble constraint
solve valid=1 && d0=0 && o0=d0 && ... && debug=1
```

### ASSERT

- **Components**: 8 nibble components (for 32-bit condition)
- **Behavior**: Checks condition, sets error flag on failure
- **Constraints**: ~10 per nibble (80 total)
- **Use cases**: Runtime validation, invariant checking

```tau
# Example ASSERT nibble constraint
solve valid=1 && c0=0 && e0=0 && ... && ok=(...) && err=(ok+1)
```

## Boolean Algebra Implementation

All system instructions use Tau's Boolean algebra:
- `+` = XOR (exclusive or)
- `&` = AND
- `|` = OR  
- `(x+1)` = NOT x (avoiding precedence issues with `x'`)

## Integration

The system instructions are integrated into the ISAGenerator:

1. Added methods:
   - `_generate_nop_component()`
   - `_generate_halt_component()`
   - `_generate_debug_nibble()`
   - `_generate_assert_nibble()`
   - `_generate_system()`

2. Updated `generate()` method to handle system instructions

3. All components respect the 700-character limit for Tau expressions

## Testing

Run the test suite with:

```bash
python3 src/zkvm/compiler/subagents/test_system_instructions.py
```

This validates:
- Component generation
- Constraint counts
- File size limits
- Tau syntax validity

## Usage Example

```python
from subagents.isa_generator import ISAGenerator

generator = ISAGenerator()
results = generator.generate(["NOP", "HALT", "DEBUG", "ASSERT"])

# Each result contains:
# - components_generated: List of InstructionComponent objects
# - files: Dict of filename -> Tau content
# - contracts: List of Contract objects
# - total_constraints: Total constraint count
```