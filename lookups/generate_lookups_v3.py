#!/usr/bin/env python3
"""
Generate Tau lookup tables v3 - with correct operator precedence.
Key learning: NOT operator needs careful handling with AND/OR.
"""

def bits_to_assignments(value, prefix, width=8):
    """Convert integer to bit variable assignments."""
    assignments = []
    for i in range(width):
        bit = (value >> i) & 1
        assignments.append(f"{prefix}{i}={bit}")
    return " && ".join(assignments)

def build_result_check(expected, prefix, width=8):
    """Build result check avoiding precedence issues."""
    # Instead of using NOT operator, we directly check the expected values
    checks = []
    for i in range(width):
        bit_val = (expected >> i) & 1
        if bit_val == 1:
            checks.append(f"{prefix}{i}")
        else:
            # For 0 bits, check that the XOR with 1 gives 1 (i.e., bit is 0)
            checks.append(f"({prefix}{i}+1)")
    
    # Use single-character variables to accumulate the check
    # This avoids complex expressions with NOT
    result = "check=(" + "&".join(checks) + ")"
    return result

def generate_lookup_table(operation, test_cases):
    """Generate lookup table for a specific operation."""
    tables = []
    
    for a, b, expected in test_cases:
        filename = f"lut_{operation}_{a}_{b}.tau"
        
        solve_parts = []
        
        # Input assignments
        solve_parts.append(bits_to_assignments(a, 'a'))
        solve_parts.append(bits_to_assignments(b, 'b'))
        
        # Operation-specific computation
        if operation == "and":
            for i in range(8):
                solve_parts.append(f"r{i}=(a{i}&b{i})")
        elif operation == "or":
            for i in range(8):
                solve_parts.append(f"r{i}=(a{i}|b{i})")
        elif operation == "xor":
            for i in range(8):
                solve_parts.append(f"r{i}=(a{i}+b{i})")
        
        # Result validation - match expected bit pattern
        for i in range(8):
            bit_val = (expected >> i) & 1
            if bit_val == 1:
                solve_parts.append(f"t{i}=r{i}")
            else:
                solve_parts.append(f"t{i}=(r{i}+1)")  # r{i} XOR 1 = 1 means r{i} = 0
        
        # Final check that all t bits are true
        solve_parts.append("result=(t0&t1&t2&t3&t4&t5&t6&t7)")
        
        solve_statement = "solve " + " && ".join(solve_parts)
        
        content = f"""# {operation.upper()} lookup validation: {a} {operation} {b} = {expected}
{solve_statement}

quit"""
        
        tables.append((filename, content))
    
    return tables

def generate_add_lookup(test_cases):
    """Generate ADD lookup with carry chain."""
    tables = []
    
    for a, b, expected in test_cases:
        filename = f"lut_add_{a}_{b}.tau"
        
        solve_parts = []
        
        # Input assignments
        solve_parts.append(bits_to_assignments(a, 'a'))
        solve_parts.append(bits_to_assignments(b, 'b'))
        
        # Carry chain addition
        solve_parts.append("s0=(a0+b0)")
        solve_parts.append("c0=(a0&b0)")
        
        for i in range(1, 8):
            solve_parts.append(f"s{i}=(a{i}+b{i}+c{i-1})")
            solve_parts.append(f"c{i}=((a{i}&b{i})|((a{i}+b{i})&c{i-1}))")
        
        # Result validation
        for i in range(8):
            bit_val = (expected >> i) & 1
            if bit_val == 1:
                solve_parts.append(f"t{i}=s{i}")
            else:
                solve_parts.append(f"t{i}=(s{i}+1)")
        
        solve_parts.append("result=(t0&t1&t2&t3&t4&t5&t6&t7)")
        
        solve_statement = "solve " + " && ".join(solve_parts)
        
        content = f"""# ADD lookup validation: {a} + {b} = {expected}
{solve_statement}

quit"""
        
        tables.append((filename, content))
    
    return tables

def generate_all_lookups():
    """Generate all lookup validation files."""
    all_tables = []
    
    # AND test cases
    and_cases = [
        (0, 0, 0),
        (255, 255, 255),
        (15, 240, 0),
        (170, 85, 0),
    ]
    all_tables.extend(generate_lookup_table("and", and_cases))
    
    # OR test cases
    or_cases = [
        (0, 0, 0),
        (255, 255, 255),
        (15, 240, 255),
        (170, 85, 255),
    ]
    all_tables.extend(generate_lookup_table("or", or_cases))
    
    # XOR test cases
    xor_cases = [
        (0, 0, 0),
        (255, 255, 0),
        (170, 85, 255),
        (15, 240, 255),
    ]
    all_tables.extend(generate_lookup_table("xor", xor_cases))
    
    # ADD test cases (8-bit wrapping)
    add_cases = [
        (0, 0, 0),
        (15, 16, 31),
        (255, 1, 0),    # Overflow
        (127, 128, 255),
    ]
    all_tables.extend(generate_add_lookup(add_cases))
    
    return all_tables

if __name__ == "__main__":
    import os
    
    tables = generate_all_lookups()
    
    for filename, content in tables:
        filepath = os.path.join(os.path.dirname(__file__), filename)
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Generated {filename}")