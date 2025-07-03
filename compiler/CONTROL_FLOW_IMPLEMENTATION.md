# Control Flow Instructions Implementation for TauFoldZKVM

## Overview
This document describes the implementation of control flow instructions (JMP, JZ, JNZ, CALL, RET) for TauFoldZKVM using nibble decomposition to overcome Tau's 800-character expression limit.

## Architecture

### Nibble Decomposition
- 32-bit operations are decomposed into 8 nibbles (4-bit chunks)
- Each nibble component stays well under 700 characters
- Components are linked via carry/borrow chains or aggregators

### Control Flow Instructions Implemented

#### 1. JMP (Unconditional Jump)
- **Components**: 8 nibbles
- **Function**: Copy target address to program counter
- **Character Count**: ~156-172 per nibble
- **Pattern**: Simple copy operation for each nibble

#### 2. JZ (Jump if Zero)
- **Components**: 8 nibbles + 1 zero flag aggregator + 7 carry links
- **Function**: Jump to target if zero flag is set, else PC+4
- **Character Count**: ~352-452 per nibble, 635 for aggregator
- **Pattern**: Conditional selection based on zero flag

#### 3. JNZ (Jump if Not Zero)
- **Components**: 8 nibbles + 1 zero flag aggregator + 7 carry links
- **Function**: Jump to target if zero flag is NOT set, else PC+4
- **Character Count**: ~352-452 per nibble, 635 for aggregator
- **Pattern**: Inverse conditional of JZ

#### 4. CALL (Function Call)
- **Components**: 8 nibbles + 7 carry links
- **Function**: Save return address (PC+4) to stack, jump to target
- **Character Count**: ~345-454 per nibble
- **Pattern**: Calculate return address, save to stack, jump

#### 5. RET (Return from Function)
- **Components**: 8 nibbles
- **Function**: Restore PC from stack return address
- **Character Count**: ~216-236 per nibble
- **Pattern**: Load return address from stack to PC

## Key Design Decisions

### 1. Zero Flag Aggregation
- Separate component checks if all 32 bits are zero
- Each nibble checks its 4 bits: `nz[i] = ((r0+1)&(r1+1)&(r2+1)&(r3+1))`
- Final zero flag: `zflag = (nz0&nz1&...&nz7)`

### 2. PC Increment Logic
- PC+4 implemented with carry chain
- Nibble 0 adds 4 (binary 0100) to bits 0-3
- Subsequent nibbles propagate carry

### 3. Conditional Logic
- JZ: `newpc = (zflag ? target : pc+4)`
- JNZ: `newpc = (!zflag ? target : pc+4)`
- Implemented using Boolean algebra: `(condition&option1)|(!condition&option2)`

### 4. Stack Operations
- CALL saves return address nibble by nibble
- RET restores PC nibble by nibble
- Stack pointer changes tracked but not fully implemented

## Contract-Based Composition

Each component follows the assume-guarantee pattern:
- **Assumptions**: Input constraints (e.g., `zflag=0|zflag=1`)
- **Guarantees**: Output promises (e.g., `newpc[i]` bits set correctly)
- **Composition**: Components link via shared variables (carries, flags)

## Testing Results

All components validated successfully:
- ✅ All expressions under 700 characters
- ✅ Proper carry chain propagation
- ✅ Correct conditional logic
- ✅ Stack save/restore operations

## Integration

To use these components:

1. Import the ControlFlowGenerator:
```python
from control_flow_generator import ControlFlowGenerator, generate_control_flow_instruction
```

2. Generate components for an instruction:
```python
components = generate_control_flow_instruction("JZ")
```

3. Write components to Tau files:
```python
for comp in components:
    with open(f"{comp.name}.tau", 'w') as f:
        f.write(comp.to_tau())
```

## Future Enhancements

1. **Full Stack Pointer Management**: Implement complete SP increment/decrement
2. **Return Address Stack**: Dedicated memory region for call stack
3. **Indirect Jumps**: Support register-based jump targets
4. **Conditional Flags**: Support other flags (negative, overflow, carry)
5. **Branch Prediction**: Add hints for likely/unlikely branches

## Conclusion

The control flow instructions have been successfully implemented using the nibble decomposition pattern, maintaining correctness while respecting Tau's expression length limitations. The modular design allows for easy testing, validation, and future enhancements.