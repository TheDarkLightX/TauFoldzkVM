#!/usr/bin/env python3
"""
Generate Tau lookup tables v2 - using correct syntax patterns.
Based on analysis of working Tau files.
"""

def bits_to_assignments(value, prefix, width=8):
    """Convert integer to bit variable assignments."""
    assignments = []
    for i in range(width):
        bit = (value >> i) & 1
        assignments.append(f"{prefix}{i}={bit}")
    return " && ".join(assignments)

def bits_to_checks(value, prefix, width=8):
    """Convert integer to bit checks (using ' for NOT)."""
    checks = []
    for i in range(width):
        if (value >> i) & 1:
            checks.append(f"{prefix}{i}")
        else:
            checks.append(f"{prefix}{i}'")
    return " && ".join(checks)

def generate_and_validation():
    """Generate 8-bit AND validation with multiple test cases."""
    # Test cases: (a, b, expected)
    test_cases = [
        (0, 0, 0),        # 0 & 0 = 0
        (255, 255, 255),  # 255 & 255 = 255
        (15, 240, 0),     # 0x0F & 0xF0 = 0
        (170, 85, 0),     # 0xAA & 0x55 = 0
    ]
    
    content = """# 8-bit AND Lookup Validation
# Tests multiple cases of AND operation

"""
    
    for i, (a, b, expected) in enumerate(test_cases):
        test_name = f"test_and_{a}_{b}"
        
        # Build solve statement
        solve_parts = []
        
        # Input assignments
        solve_parts.append(bits_to_assignments(a, 'a'))
        solve_parts.append(bits_to_assignments(b, 'b'))
        
        # Result computation
        for j in range(8):
            solve_parts.append(f"r{j}=(a{j}&b{j})")
        
        # Result validation
        solve_parts.append(f"result=({bits_to_checks(expected, 'r')})")
        
        solve_statement = "solve " + " && ".join(solve_parts)
        
        # Write individual test file
        with open(f"{test_name}.tau", 'w') as f:
            f.write(f"# AND test: {a} & {b} = {expected}\n")
            f.write(solve_statement + "\n\nquit")
        
        content += f"# Test {i+1}: {a} & {b} = {expected}\n"
    
    return content

def generate_add_validation():
    """Generate 8-bit ADD validation with carry chain."""
    # Test case: 15 + 16 = 31
    a, b, expected = 15, 16, 31
    
    content = f"""# 8-bit ADD validation: {a} + {b} = {expected}
# Uses carry chain addition

solve {bits_to_assignments(a, 'a')} && {bits_to_assignments(b, 'b')} && """
    
    # Build carry chain
    carry_parts = []
    carry_parts.append("s0=(a0+b0)")
    carry_parts.append("c0=(a0&b0)")
    
    for i in range(1, 8):
        carry_parts.append(f"s{i}=(a{i}+b{i}+c{i-1})")
        carry_parts.append(f"c{i}=((a{i}&b{i})|((a{i}+b{i})&c{i-1}))")
    
    content += " && ".join(carry_parts)
    
    # Verify result
    content += f" && result=({bits_to_checks(expected, 's')})"
    content += "\n\nquit"
    
    return content

def generate_xor_validation():
    """Generate 8-bit XOR validation."""
    # Test case: 170 ^ 85 = 255 (0xAA ^ 0x55 = 0xFF)
    a, b, expected = 170, 85, 255
    
    content = f"""# 8-bit XOR validation: {a} ^ {b} = {expected}

solve {bits_to_assignments(a, 'a')} && {bits_to_assignments(b, 'b')} && """
    
    # XOR computation
    xor_parts = []
    for i in range(8):
        xor_parts.append(f"r{i}=(a{i}+b{i})")
    
    content += " && ".join(xor_parts)
    content += f" && result=({bits_to_checks(expected, 'r')})"
    content += "\n\nquit"
    
    return content

def generate_or_validation():
    """Generate 8-bit OR validation."""
    # Test case: 15 | 240 = 255
    a, b, expected = 15, 240, 255
    
    content = f"""# 8-bit OR validation: {a} | {b} = {expected}

solve {bits_to_assignments(a, 'a')} && {bits_to_assignments(b, 'b')} && """
    
    # OR computation
    or_parts = []
    for i in range(8):
        or_parts.append(f"r{i}=(a{i}|b{i})")
    
    content += " && ".join(or_parts)
    content += f" && result=({bits_to_checks(expected, 'r')})"
    content += "\n\nquit"
    
    return content

def generate_all():
    """Generate all validation files."""
    # Generate AND test cases
    generate_and_validation()
    
    # Single file validations
    validations = [
        ("validate_add.tau", generate_add_validation()),
        ("validate_xor.tau", generate_xor_validation()),
        ("validate_or.tau", generate_or_validation()),
    ]
    
    for filename, content in validations:
        with open(filename, 'w') as f:
            f.write(content)
        print(f"Generated {filename}")

if __name__ == "__main__":
    generate_all()