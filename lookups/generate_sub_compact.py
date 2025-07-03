#!/usr/bin/env python3
"""
Generate compact SUB operations for Tau.
Uses simplified two's complement approach to stay under length limit.
"""

def generate_sub_compact(a, b, expected):
    """Generate compact subtraction using simplified two's complement."""
    parts = []
    
    # Input bits
    for i in range(8):
        parts.append(f"a{i}={(a >> i) & 1}")
        parts.append(f"b{i}={(b >> i) & 1}")
    
    # For subtraction, we can use the fact that a - b = a + (~b) + 1
    # But we'll use a more compact representation
    # First bit: s0 = a0 XOR b0 XOR 1 (the +1 for two's complement)
    parts.append("s0=(a0+b0+1)")
    parts.append("c0=((a0|(b0+1))&(b0|1))")  # Simplified carry
    
    # Remaining bits with simplified carry propagation
    for i in range(1, 8):
        parts.append(f"s{i}=(a{i}+b{i}+c{i-1})")
        # Simplified carry calculation
        parts.append(f"c{i}=((a{i}|b{i})&c{i-1})")
    
    # Result check
    for i in range(8):
        if (expected >> i) & 1:
            parts.append(f"t{i}=s{i}")
        else:
            parts.append(f"t{i}=(s{i}+1)")
    
    # Final check - split into two parts to reduce length
    parts.append("r1=(t0&t1&t2&t3)")
    parts.append("r2=(t4&t5&t6&t7)")
    parts.append("result=(r1&r2)")
    
    return " && ".join(parts)

# Test cases
test_cases = [
    (100, 50, 50, "Normal subtraction"),
    (50, 100, 206, "Negative result (two's complement)"),
]

for a, b, expected, desc in test_cases:
    content = f"""# SUB lookup: {a} - {b} = {expected}
# {desc}
solve {generate_sub_compact(a, b, expected)}

quit"""
    
    filename = f"lut_sub_{a}_{b}_compact.tau"
    with open(filename, 'w') as f:
        f.write(content)
    print(f"Generated {filename} ({len(content)} chars)")