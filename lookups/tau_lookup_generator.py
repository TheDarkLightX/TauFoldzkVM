#!/usr/bin/env python3
"""
Tau Lookup Table Generator - Production Version
Generates validated Tau specifications for zkVM lookup tables.
Incorporates all lessons learned about Tau's quirks.
"""

import os
from typing import List, Tuple, Dict, Callable
from dataclasses import dataclass
from enum import Enum

class Operation(Enum):
    """Supported 8-bit operations."""
    AND = "and"
    OR = "or"
    XOR = "xor"
    ADD = "add"
    SUB = "sub"
    MUL = "mul"
    SHL = "shl"
    SHR = "shr"

@dataclass
class TestCase:
    """Single test case for validation."""
    a: int
    b: int
    expected: int
    description: str = ""

class TauLookupGenerator:
    """Main generator class for Tau lookup tables."""
    
    MAX_EXPR_LENGTH = 700  # Safe limit for expression length
    BIT_WIDTH = 8
    
    def __init__(self, output_dir: str = "."):
        self.output_dir = output_dir
        os.makedirs(output_dir, exist_ok=True)
    
    def bits_to_assignments(self, value: int, prefix: str, width: int = 8) -> List[str]:
        """Convert integer to bit variable assignments."""
        assignments = []
        for i in range(width):
            bit = (value >> i) & 1
            assignments.append(f"{prefix}{i}={bit}")
        return assignments
    
    def check_bit_value(self, bit_var: str, expected: int) -> str:
        """Generate constraint to check if bit has expected value."""
        if expected == 1:
            return bit_var
        else:
            # Use XOR with 1 to check for 0 (avoids NOT operator issues)
            return f"({bit_var}+1)"
    
    def check_bits_equal(self, value: int, prefix: str, width: int = 8) -> List[str]:
        """Generate constraints to check if bits equal expected value."""
        checks = []
        for i in range(width):
            bit_val = (value >> i) & 1
            checks.append(self.check_bit_value(f"{prefix}{i}", bit_val))
        return checks
    
    def generate_bitwise_op(self, op: Operation, a: int, b: int, expected: int) -> str:
        """Generate constraints for bitwise operations."""
        parts = []
        
        # Input assignments
        parts.extend(self.bits_to_assignments(a, 'a'))
        parts.extend(self.bits_to_assignments(b, 'b'))
        
        # Compute result based on operation
        for i in range(self.BIT_WIDTH):
            if op == Operation.AND:
                parts.append(f"r{i}=(a{i}&b{i})")
            elif op == Operation.OR:
                parts.append(f"r{i}=(a{i}|b{i})")
            elif op == Operation.XOR:
                parts.append(f"r{i}=(a{i}+b{i})")
        
        # Check result
        result_checks = self.check_bits_equal(expected, 'r')
        for i, check in enumerate(result_checks):
            parts.append(f"t{i}={check}")
        
        # Final verification
        parts.append("result=(" + "&".join([f"t{i}" for i in range(self.BIT_WIDTH)]) + ")")
        
        return " && ".join(parts)
    
    def generate_add(self, a: int, b: int, expected: int) -> str:
        """Generate constraints for 8-bit addition with carry chain."""
        parts = []
        
        # Input assignments
        parts.extend(self.bits_to_assignments(a, 'a'))
        parts.extend(self.bits_to_assignments(b, 'b'))
        
        # Carry chain addition
        parts.append("s0=(a0+b0)")
        parts.append("c0=(a0&b0)")
        
        for i in range(1, self.BIT_WIDTH):
            parts.append(f"s{i}=(a{i}+b{i}+c{i-1})")
            parts.append(f"c{i}=((a{i}&b{i})|((a{i}+b{i})&c{i-1}))")
        
        # Check result (ignore carry out for 8-bit result)
        result_checks = self.check_bits_equal(expected, 's')
        for i, check in enumerate(result_checks):
            parts.append(f"t{i}={check}")
        
        parts.append("result=(" + "&".join([f"t{i}" for i in range(self.BIT_WIDTH)]) + ")")
        
        return " && ".join(parts)
    
    def generate_sub(self, a: int, b: int, expected: int) -> str:
        """Generate constraints for 8-bit subtraction using two's complement."""
        parts = []
        
        # Input assignments
        parts.extend(self.bits_to_assignments(a, 'a'))
        parts.extend(self.bits_to_assignments(b, 'b'))
        
        # Two's complement: a - b = a + (~b + 1)
        # First compute ~b using XOR with 1
        for i in range(self.BIT_WIDTH):
            parts.append(f"nb{i}=(b{i}+1)")  # NOT b[i]
        
        # Add a + ~b + 1
        parts.append("s0=(a0+nb0+1)")  # +1 for two's complement
        parts.append("c0=((a0&(nb0+1))|((a0+nb0)&1))")
        
        for i in range(1, self.BIT_WIDTH):
            parts.append(f"s{i}=(a{i}+nb{i}+c{i-1})")
            parts.append(f"c{i}=((a{i}&(nb{i}+c{i-1}))|((a{i}+nb{i})&c{i-1}))")
        
        # Check result
        result_checks = self.check_bits_equal(expected, 's')
        for i, check in enumerate(result_checks):
            parts.append(f"t{i}={check}")
        
        parts.append("result=(" + "&".join([f"t{i}" for i in range(self.BIT_WIDTH)]) + ")")
        
        return " && ".join(parts)
    
    def generate_mul(self, a: int, b: int, expected: int) -> str:
        """Generate constraints for 8-bit multiplication (truncated to 8 bits)."""
        parts = []
        
        # Input assignments
        parts.extend(self.bits_to_assignments(a, 'a'))
        parts.extend(self.bits_to_assignments(b, 'b'))
        
        # Shift-and-add multiplication
        # Initialize partial products
        for j in range(self.BIT_WIDTH):
            for i in range(self.BIT_WIDTH):
                if i + j < self.BIT_WIDTH:
                    parts.append(f"p{j}{i}=(a{i}&b{j})")
        
        # Sum partial products (simplified for 8-bit result)
        # This is a simplified version - full multiplication would need more carries
        parts.append("m0=p00")
        parts.append("m1=(p01+p10)")
        parts.append("m2=(p02+p11+p20)")
        # ... simplified for demonstration
        
        # For now, use a test pattern that validates the concept
        result_checks = self.check_bits_equal(expected & 0xFF, 'm')
        
        # Simplified result check for demonstration
        parts.append("result=1")  # Placeholder - full implementation needed
        
        return " && ".join(parts)
    
    def generate_shl(self, a: int, shift: int, expected: int) -> str:
        """Generate constraints for left shift."""
        parts = []
        
        # Input assignments
        parts.extend(self.bits_to_assignments(a, 'a'))
        parts.extend(self.bits_to_assignments(shift, 'sh'))
        
        # For simplicity, handle fixed shift amounts
        # Full implementation would use multiplexers
        for i in range(self.BIT_WIDTH):
            if i >= shift:
                parts.append(f"r{i}=a{i-shift}")
            else:
                parts.append(f"r{i}=0")
        
        # Check result
        result_checks = self.check_bits_equal(expected, 'r')
        for i, check in enumerate(result_checks):
            parts.append(f"t{i}={check}")
        
        parts.append("result=(" + "&".join([f"t{i}" for i in range(self.BIT_WIDTH)]) + ")")
        
        return " && ".join(parts)
    
    def generate_shr(self, a: int, shift: int, expected: int) -> str:
        """Generate constraints for logical right shift."""
        parts = []
        
        # Input assignments
        parts.extend(self.bits_to_assignments(a, 'a'))
        parts.extend(self.bits_to_assignments(shift, 'sh'))
        
        # Fixed shift for simplicity
        for i in range(self.BIT_WIDTH):
            if i + shift < self.BIT_WIDTH:
                parts.append(f"r{i}=a{i+shift}")
            else:
                parts.append(f"r{i}=0")
        
        # Check result
        result_checks = self.check_bits_equal(expected, 'r')
        for i, check in enumerate(result_checks):
            parts.append(f"t{i}={check}")
        
        parts.append("result=(" + "&".join([f"t{i}" for i in range(self.BIT_WIDTH)]) + ")")
        
        return " && ".join(parts)
    
    def generate_test_file(self, op: Operation, test: TestCase) -> Tuple[str, str]:
        """Generate a complete test file for an operation."""
        # Generate constraints based on operation
        if op in [Operation.AND, Operation.OR, Operation.XOR]:
            constraints = self.generate_bitwise_op(op, test.a, test.b, test.expected)
        elif op == Operation.ADD:
            constraints = self.generate_add(test.a, test.b, test.expected)
        elif op == Operation.SUB:
            constraints = self.generate_sub(test.a, test.b, test.expected)
        elif op == Operation.MUL:
            constraints = self.generate_mul(test.a, test.b, test.expected)
        elif op == Operation.SHL:
            constraints = self.generate_shl(test.a, test.b, test.expected)
        elif op == Operation.SHR:
            constraints = self.generate_shr(test.a, test.b, test.expected)
        else:
            raise ValueError(f"Unknown operation: {op}")
        
        # Check expression length
        if len(constraints) > self.MAX_EXPR_LENGTH:
            raise ValueError(f"Expression too long ({len(constraints)} chars)")
        
        # Generate file content
        content = f"""# {op.value.upper()} lookup validation: {test.a} {op.value} {test.b} = {test.expected}
{test.description and f'# {test.description}' or ''}
solve {constraints}

quit"""
        
        filename = f"lut_{op.value}_{test.a}_{test.b}.tau"
        return filename, content
    
    def generate_all_lookups(self) -> Dict[Operation, List[TestCase]]:
        """Define all test cases for each operation."""
        test_cases = {
            Operation.AND: [
                TestCase(0, 0, 0, "Zero AND"),
                TestCase(255, 255, 255, "All ones"),
                TestCase(15, 240, 0, "No overlap"),
                TestCase(170, 85, 0, "Alternating bits"),
            ],
            Operation.OR: [
                TestCase(0, 0, 0, "Zero OR"),
                TestCase(255, 255, 255, "All ones"),
                TestCase(15, 240, 255, "Complete coverage"),
                TestCase(170, 85, 255, "Alternating bits"),
            ],
            Operation.XOR: [
                TestCase(0, 0, 0, "Zero XOR"),
                TestCase(255, 255, 0, "Same value"),
                TestCase(170, 85, 255, "Alternating bits"),
                TestCase(15, 240, 255, "Complementary"),
            ],
            Operation.ADD: [
                TestCase(0, 0, 0, "Zero addition"),
                TestCase(15, 16, 31, "Simple add"),
                TestCase(255, 1, 0, "Overflow"),
                TestCase(127, 128, 255, "Half overflow"),
            ],
            Operation.SUB: [
                TestCase(255, 1, 254, "Simple sub"),
                TestCase(0, 1, 255, "Underflow"),
                TestCase(128, 128, 0, "Equal values"),
                TestCase(100, 50, 50, "Normal sub"),
            ],
            Operation.SHL: [
                TestCase(1, 0, 1, "No shift"),
                TestCase(1, 1, 2, "Shift left 1"),
                TestCase(1, 7, 128, "Shift to MSB"),
                TestCase(255, 1, 254, "Shift with loss"),
            ],
            Operation.SHR: [
                TestCase(128, 0, 128, "No shift"),
                TestCase(128, 1, 64, "Shift right 1"),
                TestCase(128, 7, 1, "Shift to LSB"),
                TestCase(255, 1, 127, "Shift all bits"),
            ],
        }
        return test_cases
    
    def generate_and_save_all(self):
        """Generate and save all lookup tables."""
        test_cases = self.generate_all_lookups()
        generated_files = []
        
        for op, tests in test_cases.items():
            if op == Operation.MUL:
                # Skip MUL for now - needs more complex implementation
                continue
                
            for test in tests:
                try:
                    filename, content = self.generate_test_file(op, test)
                    filepath = os.path.join(self.output_dir, filename)
                    
                    with open(filepath, 'w') as f:
                        f.write(content)
                    
                    generated_files.append(filename)
                    print(f"Generated {filename}")
                    
                except ValueError as e:
                    print(f"Skipped {op.value} {test.a},{test.b}: {e}")
        
        return generated_files

if __name__ == "__main__":
    generator = TauLookupGenerator(".")
    files = generator.generate_and_save_all()
    print(f"\nGenerated {len(files)} lookup validation files")