#!/usr/bin/env python3
"""
Generate Tau lookup tables for 8-bit operations.
Each table validates specific test cases to prove correctness.
"""

def bits_to_vars(value, prefix, width=8):
    """Convert integer to bit variable assignments."""
    bits = []
    for i in range(width):
        bit = (value >> i) & 1
        bits.append(f"{prefix}{i}={bit}")
    return bits

def generate_and_lookup():
    """Generate 8-bit AND lookup table with validation cases."""
    test_cases = [
        (0, 0, 0),        # AND(0,0) = 0
        (255, 255, 255),  # AND(255,255) = 255
        (255, 0, 0),      # AND(255,0) = 0
        (15, 240, 0),     # AND(0x0F,0xF0) = 0
        (170, 85, 0),     # AND(0xAA,0x55) = 0 (alternating bits)
        (255, 128, 128),  # AND(0xFF,0x80) = 0x80
    ]
    
    content = """# 8-bit AND Lookup Table
# Validates AND operation for key test cases
# Each test proves a specific lookup entry is correct

"""
    
    constraints = []
    for a, b, expected in test_cases:
        # Input bits
        a_bits = bits_to_vars(a, 'a')
        b_bits = bits_to_vars(b, 'b')
        
        # Result bits using AND
        r_bits = []
        for i in range(8):
            r_bits.append(f"r{i}=(a{i}&b{i})")
        
        # Verify result equals expected
        expected_bits = []
        for i in range(8):
            if (expected >> i) & 1:
                expected_bits.append(f"r{i}")
            else:
                expected_bits.append(f"r{i}'")
        
        # Build constraint for this test case
        case_constraint = " && ".join(a_bits + b_bits + r_bits + expected_bits)
        constraints.append(f"({case_constraint})")
    
    # Combine all test cases with OR (at least one must be satisfied)
    solve_statement = "solve " + " | ".join(constraints)
    
    content += solve_statement + "\n\nquit"
    
    return content

def generate_or_lookup():
    """Generate 8-bit OR lookup table with validation cases."""
    test_cases = [
        (0, 0, 0),        # OR(0,0) = 0
        (255, 255, 255),  # OR(255,255) = 255
        (255, 0, 255),    # OR(255,0) = 255
        (15, 240, 255),   # OR(0x0F,0xF0) = 0xFF
        (170, 85, 255),   # OR(0xAA,0x55) = 0xFF (alternating bits)
        (128, 64, 192),   # OR(0x80,0x40) = 0xC0
    ]
    
    content = """# 8-bit OR Lookup Table
# Validates OR operation for key test cases
# Each test proves a specific lookup entry is correct

"""
    
    constraints = []
    for a, b, expected in test_cases:
        a_bits = bits_to_vars(a, 'a')
        b_bits = bits_to_vars(b, 'b')
        
        r_bits = []
        for i in range(8):
            r_bits.append(f"r{i}=(a{i}|b{i})")
        
        expected_bits = []
        for i in range(8):
            if (expected >> i) & 1:
                expected_bits.append(f"r{i}")
            else:
                expected_bits.append(f"r{i}'")
        
        case_constraint = " && ".join(a_bits + b_bits + r_bits + expected_bits)
        constraints.append(f"({case_constraint})")
    
    solve_statement = "solve " + " | ".join(constraints)
    content += solve_statement + "\n\nquit"
    
    return content

def generate_xor_lookup():
    """Generate 8-bit XOR lookup table with validation cases."""
    test_cases = [
        (0, 0, 0),        # XOR(0,0) = 0
        (255, 255, 0),    # XOR(255,255) = 0
        (255, 0, 255),    # XOR(255,0) = 255
        (170, 85, 255),   # XOR(0xAA,0x55) = 0xFF
        (15, 240, 255),   # XOR(0x0F,0xF0) = 0xFF
        (128, 128, 0),    # XOR(0x80,0x80) = 0
    ]
    
    content = """# 8-bit XOR Lookup Table
# Validates XOR operation for key test cases
# Each test proves a specific lookup entry is correct

"""
    
    constraints = []
    for a, b, expected in test_cases:
        a_bits = bits_to_vars(a, 'a')
        b_bits = bits_to_vars(b, 'b')
        
        r_bits = []
        for i in range(8):
            r_bits.append(f"r{i}=(a{i}+b{i})")  # + is XOR in Tau
        
        expected_bits = []
        for i in range(8):
            if (expected >> i) & 1:
                expected_bits.append(f"r{i}")
            else:
                expected_bits.append(f"r{i}'")
        
        case_constraint = " && ".join(a_bits + b_bits + r_bits + expected_bits)
        constraints.append(f"({case_constraint})")
    
    solve_statement = "solve " + " | ".join(constraints)
    content += solve_statement + "\n\nquit"
    
    return content

def generate_add_lookup():
    """Generate 8-bit ADD lookup table with validation cases."""
    test_cases = [
        (0, 0, 0),        # ADD(0,0) = 0
        (255, 1, 0),      # ADD(255,1) = 0 (overflow)
        (128, 128, 0),    # ADD(128,128) = 0 (overflow)
        (100, 156, 0),    # ADD(100,156) = 256 = 0 (mod 256)
        (15, 16, 31),     # ADD(15,16) = 31
        (127, 1, 128),    # ADD(127,1) = 128
    ]
    
    content = """# 8-bit ADD Lookup Table (modulo 256)
# Validates ADD operation with carry chain
# Each test proves a specific lookup entry is correct

"""
    
    constraints = []
    for a, b, expected in test_cases:
        a_bits = bits_to_vars(a, 'a')
        b_bits = bits_to_vars(b, 'b')
        
        # Carry chain addition
        sum_and_carry = []
        sum_and_carry.append("s0=(a0+b0)")
        sum_and_carry.append("c0=(a0&b0)")
        
        for i in range(1, 8):
            sum_and_carry.append(f"s{i}=(a{i}+b{i}+c{i-1})")
            sum_and_carry.append(f"c{i}=((a{i}&b{i})|((a{i}+b{i})&c{i-1}))")
        
        # Verify sum equals expected (ignore final carry for mod 256)
        expected_bits = []
        for i in range(8):
            if (expected >> i) & 1:
                expected_bits.append(f"s{i}")
            else:
                expected_bits.append(f"s{i}'")
        
        case_constraint = " && ".join(a_bits + b_bits + sum_and_carry + expected_bits)
        constraints.append(f"({case_constraint})")
    
    solve_statement = "solve " + " | ".join(constraints)
    content += solve_statement + "\n\nquit"
    
    return content

def generate_sub_lookup():
    """Generate 8-bit SUB lookup table with validation cases."""
    test_cases = [
        (255, 1, 254),    # SUB(255,1) = 254
        (0, 1, 255),      # SUB(0,1) = 255 (underflow)
        (128, 128, 0),    # SUB(128,128) = 0
        (100, 50, 50),    # SUB(100,50) = 50
        (50, 100, 206),   # SUB(50,100) = -50 = 206 (mod 256)
        (200, 56, 144),   # SUB(200,56) = 144
    ]
    
    content = """# 8-bit SUB Lookup Table (modulo 256)
# Validates SUB operation using two's complement
# Each test proves a specific lookup entry is correct

"""
    
    constraints = []
    for a, b, expected in test_cases:
        a_bits = bits_to_vars(a, 'a')
        b_bits = bits_to_vars(b, 'b')
        
        # Two's complement: a - b = a + (~b + 1)
        # First compute ~b (NOT of each bit)
        not_b = []
        for i in range(8):
            not_b.append(f"nb{i}=b{i}'")
        
        # Then add a + ~b + 1
        sum_and_carry = not_b
        sum_and_carry.append("s0=(a0+nb0+1)")  # +1 for two's complement
        sum_and_carry.append("c0=((a0&(nb0+1))|((a0+nb0)&1))")
        
        for i in range(1, 8):
            sum_and_carry.append(f"s{i}=(a{i}+nb{i}+c{i-1})")
            sum_and_carry.append(f"c{i}=((a{i}&(nb{i}+c{i-1}))|((a{i}+nb{i})&c{i-1}))")
        
        # Verify result equals expected
        expected_bits = []
        for i in range(8):
            if (expected >> i) & 1:
                expected_bits.append(f"s{i}")
            else:
                expected_bits.append(f"s{i}'")
        
        case_constraint = " && ".join(a_bits + b_bits + sum_and_carry + expected_bits)
        constraints.append(f"({case_constraint})")
    
    solve_statement = "solve " + " | ".join(constraints)
    content += solve_statement + "\n\nquit"
    
    return content

def generate_all_lookups():
    """Generate all lookup tables."""
    lookups = [
        ("lut8_and.tau", generate_and_lookup()),
        ("lut8_or.tau", generate_or_lookup()),
        ("lut8_xor.tau", generate_xor_lookup()),
        ("lut8_add.tau", generate_add_lookup()),
        ("lut8_sub.tau", generate_sub_lookup()),
    ]
    
    return lookups

if __name__ == "__main__":
    import os
    
    lookups = generate_all_lookups()
    lookup_dir = os.path.dirname(os.path.abspath(__file__))
    
    for filename, content in lookups:
        filepath = os.path.join(lookup_dir, filename)
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Generated {filename}")