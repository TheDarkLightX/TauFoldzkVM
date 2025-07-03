#!/usr/bin/env python3
"""
Lookup Decomposition Framework
Decompose 16-bit operations into 8-bit lookups following Jolt/Lasso approach.
"""

def generate_16bit_and_decomposition():
    """
    16-bit AND using 8-bit lookups.
    Split 16-bit values into high and low bytes, perform 8-bit ANDs.
    """
    content = """# 16-bit AND via 8-bit lookup decomposition
# Test: 0x0F0F & 0xF0F0 = 0x0000

# Input: a = 0x0F0F (3855), b = 0xF0F0 (61680)
# Expected: result = 0x0000 (0)

solve """
    
    # 16-bit input a = 0x0F0F
    a_hi = 0x0F  # bits 15-8
    a_lo = 0x0F  # bits 7-0
    
    # 16-bit input b = 0xF0F0
    b_hi = 0xF0  # bits 15-8
    b_lo = 0xF0  # bits 7-0
    
    # Expected results
    r_hi = a_hi & b_hi  # 0x0F & 0xF0 = 0x00
    r_lo = a_lo & b_lo  # 0x0F & 0xF0 = 0x00
    
    parts = []
    
    # Set high byte of a
    for i in range(8):
        bit = (a_hi >> i) & 1
        parts.append(f"ah{i}={bit}")
    
    # Set low byte of a
    for i in range(8):
        bit = (a_lo >> i) & 1
        parts.append(f"al{i}={bit}")
    
    # Set high byte of b
    for i in range(8):
        bit = (b_hi >> i) & 1
        parts.append(f"bh{i}={bit}")
    
    # Set low byte of b
    for i in range(8):
        bit = (b_lo >> i) & 1
        parts.append(f"bl{i}={bit}")
    
    # Perform 8-bit AND on high bytes
    for i in range(8):
        parts.append(f"rh{i}=(ah{i}&bh{i})")
    
    # Perform 8-bit AND on low bytes
    for i in range(8):
        parts.append(f"rl{i}=(al{i}&bl{i})")
    
    # Verify high byte result (should be 0)
    for i in range(8):
        parts.append(f"th{i}=(rh{i}+1)")  # All bits should be 0
    
    # Verify low byte result (should be 0)
    for i in range(8):
        parts.append(f"tl{i}=(rl{i}+1)")  # All bits should be 0
    
    # Final verification
    parts.append("high_ok=(th0&th1&th2&th3&th4&th5&th6&th7)")
    parts.append("low_ok=(tl0&tl1&tl2&tl3&tl4&tl5&tl6&tl7)")
    parts.append("result=(high_ok&low_ok)")
    
    content += " && ".join(parts)
    content += "\n\nquit"
    
    return content

def generate_16bit_add_decomposition():
    """
    16-bit ADD using 8-bit lookups with carry propagation.
    This is more complex as we need to handle carry between bytes.
    """
    content = """# 16-bit ADD via 8-bit lookup decomposition with carry
# Test: 0x00FF + 0x0001 = 0x0100

# Input: a = 0x00FF (255), b = 0x0001 (1)
# Expected: result = 0x0100 (256)

solve """
    
    # 16-bit inputs
    a = 0x00FF
    b = 0x0001
    expected = 0x0100
    
    a_hi = (a >> 8) & 0xFF
    a_lo = a & 0xFF
    b_hi = (b >> 8) & 0xFF
    b_lo = b & 0xFF
    
    # Expected results
    expected_hi = (expected >> 8) & 0xFF
    expected_lo = expected & 0xFF
    
    parts = []
    
    # Set bytes
    for i in range(8):
        parts.append(f"ah{i}={(a_hi >> i) & 1}")
        parts.append(f"al{i}={(a_lo >> i) & 1}")
        parts.append(f"bh{i}={(b_hi >> i) & 1}")
        parts.append(f"bl{i}={(b_lo >> i) & 1}")
    
    # Add low bytes with carry chain
    parts.append("sl0=(al0+bl0)")
    parts.append("cl0=(al0&bl0)")
    
    for i in range(1, 8):
        parts.append(f"sl{i}=(al{i}+bl{i}+cl{i-1})")
        parts.append(f"cl{i}=((al{i}&bl{i})|((al{i}+bl{i})&cl{i-1}))")
    
    # Carry from low byte to high byte
    parts.append("carry_to_high=cl7")
    
    # Add high bytes with carry from low
    parts.append("sh0=(ah0+bh0+carry_to_high)")
    parts.append("ch0=((ah0&bh0)|((ah0+bh0)&carry_to_high))")
    
    for i in range(1, 8):
        parts.append(f"sh{i}=(ah{i}+bh{i}+ch{i-1})")
        parts.append(f"ch{i}=((ah{i}&bh{i})|((ah{i}+bh{i})&ch{i-1}))")
    
    # Verify results
    for i in range(8):
        if (expected_lo >> i) & 1:
            parts.append(f"tl{i}=sl{i}")
        else:
            parts.append(f"tl{i}=(sl{i}+1)")
    
    for i in range(8):
        if (expected_hi >> i) & 1:
            parts.append(f"th{i}=sh{i}")
        else:
            parts.append(f"th{i}=(sh{i}+1)")
    
    parts.append("low_ok=(tl0&tl1&tl2&tl3&tl4&tl5&tl6&tl7)")
    parts.append("high_ok=(th0&th1&th2&th3&th4&th5&th6&th7)")
    parts.append("result=(low_ok&high_ok)")
    
    content += " && ".join(parts)
    content += "\n\nquit"
    
    return content

def generate_decomposition_framework():
    """
    Generate a general framework for decomposing n-bit operations.
    This demonstrates the pattern for arbitrary bit widths.
    """
    content = """# Lookup Decomposition Framework
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
"""
    
    return content

if __name__ == "__main__":
    import os
    
    files = [
        ("lut_16bit_and_decomp.tau", generate_16bit_and_decomposition()),
        ("lut_16bit_add_decomp.tau", generate_16bit_add_decomposition()),
        ("decomposition_framework.md", generate_decomposition_framework()),
    ]
    
    for filename, content in files:
        filepath = os.path.join(os.path.dirname(__file__), filename)
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Generated {filename}")