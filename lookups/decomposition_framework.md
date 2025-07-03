# Lookup Decomposition Framework
# General pattern for decomposing n-bit operations into m-bit lookups

# Key principles:
# 1. Split n-bit values into chunks of m bits
# 2. Perform operation on each chunk independently
# 3. Handle inter-chunk dependencies (like carry propagation)
# 4. Reconstruct final n-bit result

# For operations without inter-chunk dependencies (AND, OR, XOR):
# - Simply split, apply operation to each chunk, concatenate results

# For operations with dependencies (ADD, SUB, MUL):
# - Must propagate carries/borrows between chunks
# - May need multiple rounds of lookups

# Example: 32-bit operation using 8-bit lookups
# Input: a[31:0], b[31:0]
# Split into: a3[7:0], a2[7:0], a1[7:0], a0[7:0]
#            b3[7:0], b2[7:0], b1[7:0], b0[7:0]
# Apply 8-bit operations and handle dependencies
