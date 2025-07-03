# ProtoStar Folding in TauFoldZKVM

## Overview

ProtoStar is an advanced folding scheme that improves upon Nova/SuperNova with:
1. **Explicit noise vectors** - Clean handling of accumulation errors
2. **High-degree gate support** - No additional MSM costs
3. **O(d) lookup efficiency** - Native lookup integration
4. **Cheap cross-terms** - Field elements as commitments

## Key Components

### Instance Structure
```
Instance = (C, r, u, E)
- C: Commitment vector (using RC hash)
- r: Challenge vector
- u: Slack scalar
- E: Noise vector (tracks accumulation)
```

### Folding Operation
```
fold(I1, I2, β) → I_folded
- C_fold = C1 + β·C2
- r_fold = concat(r1, r2, β)  
- u_fold = u1 + β·u2
- E_fold = E1 + β·(cross_terms) + β²·E2
```

### Binary Field Adaptation
In Tau's binary field (F2):
- Addition is XOR
- Multiplication is AND
- β² = β for β ∈ {0,1}

## Implementation Files

1. **protostar_instance.tau** - Basic instance folding
2. **protostar_noise.tau** - Noise vector evolution
3. **protostar_degree3.tau** - High-degree gate example
4. **protostar_lookup_fold.tau** - Lookup integration

## Performance Analysis

- **Folding step**: ~150 constraints
- **Noise tracking**: ~50 constraints
- **Lookup integration**: O(d) where d = lookup degree
- **Total per fold**: ~200-300 constraints

## Next Steps

1. Integrate with ISA implementation
2. Build commitment scheme using RC hash
3. Create distributed folding protocol
