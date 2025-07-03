#!/usr/bin/env python3
"""
Control Flow Instruction Generator for TauFoldZKVM
Implements JMP, JZ, JNZ, CALL, and RET instructions using nibble decomposition
"""

from typing import List
from dataclasses import dataclass

@dataclass
class Component:
    name: str
    constraints: List[str]
    assumptions: List[str] = None
    guarantees: List[str] = None
    
    def to_tau(self) -> str:
        """Convert to Tau file content"""
        expr = " && ".join(self.constraints)
        if len(expr) > 700:
            raise ValueError(f"{self.name}: Expression too long ({len(expr)} chars)")
        
        content = f"# Component: {self.name}\n"
        if self.assumptions:
            content += f"# Assumptions: {', '.join(self.assumptions)}\n"
        if self.guarantees:
            content += f"# Guarantees: {', '.join(self.guarantees)}\n"
        content += f"\nsolve {expr}\n\nquit"
        return content

class ComponentGenerator:
    """Base class for component generators"""
    
    def generate_nibble_constraints(self, op: str, nibble: int, offset: int = 0) -> List[str]:
        """Generate constraints for a 4-bit operation"""
        constraints = []
        
        # Input bits (example values for testing)
        for i in range(4):
            bit = offset + i
            constraints.append(f"a{bit}={(i+nibble)%2}")
            constraints.append(f"b{bit}={(i+nibble+1)%2}")
        
        return constraints

class ControlFlowGenerator(ComponentGenerator):
    """Generate control flow instruction components"""
    
    def generate_jmp_nibble(self, nibble: int) -> Component:
        """Generate JMP nibble - unconditional jump to address"""
        offset = nibble * 4
        constraints = []
        
        # Target address nibble (from instruction immediate)
        for i in range(4):
            bit = offset + i
            constraints.append(f"target{bit}={(i+nibble)%2}")  # Example target address
        
        # Current PC nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"pc{bit}={(i+nibble+1)%2}")  # Example current PC
        
        # New PC nibble = target address nibble (unconditional copy)
        for i in range(4):
            bit = offset + i
            constraints.append(f"newpc{bit}=target{bit}")
        
        return Component(
            name=f"jmp_nibble_{nibble}",
            constraints=constraints,
            guarantees=[f"newpc{offset}", f"newpc{offset+1}", f"newpc{offset+2}", f"newpc{offset+3}"]
        )
    
    def generate_jz_nibble(self, nibble: int) -> Component:
        """Generate JZ nibble - jump if zero flag is set"""
        offset = nibble * 4
        constraints = []
        
        # Zero flag (shared across all nibbles)
        constraints.append("zflag=1")  # Example: zero flag is set
        
        # Target address nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"target{bit}={(i+nibble)%2}")
        
        # Current PC nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"pc{bit}={(i+nibble+1)%2}")
        
        # PC increment for sequential execution (PC + 4)
        if nibble == 0:
            # LSB nibble - add 4
            constraints.append("s0=(pc0+0)")  # bit 0
            constraints.append("s1=(pc1+0)")  # bit 1
            constraints.append("s2=(pc2+1)")  # bit 2 (add 4 = 0100 binary)
            constraints.append("c2=pc2")      # carry from bit 2
            constraints.append("s3=(pc3+c2)")  # bit 3 with carry
            constraints.append("cout0=(pc3&c2)")  # carry out
        else:
            # Other nibbles - handle carry in
            constraints.append("cin=0")  # Will be linked via composition
            if nibble == 1:
                # Continue incrementing with carry
                constraints.append(f"s{offset}=(pc{offset}+cin)")
                constraints.append(f"c{offset}=(pc{offset}&cin)")
            else:
                # Just propagate carry
                constraints.append(f"s{offset}=(pc{offset}+cin)")
                constraints.append(f"c{offset}=(pc{offset}&cin)")
            
            for i in range(1, 4):
                bit = offset + i
                constraints.append(f"s{bit}=(pc{bit}+c{bit-1})")
                constraints.append(f"c{bit}=(pc{bit}&c{bit-1})")
            
            if nibble < 7:
                constraints.append(f"cout{nibble}=c{offset+3}")
        
        # Conditional: if zflag=1, use target; else use incremented PC
        for i in range(4):
            bit = offset + i
            if nibble == 0 or i > 0:
                # Use the sum bits for incremented PC
                idx = bit if nibble > 0 else i
                constraints.append(f"newpc{bit}=((zflag&target{bit})|((zflag+1)&s{idx}))")
            else:
                # For nibble 0, bit 0, we computed s0 directly
                constraints.append(f"newpc{bit}=((zflag&target{bit})|((zflag+1)&s{i}))")
        
        assumptions = ["zflag=0|zflag=1"]
        if nibble > 0:
            assumptions.append("cin=0|cin=1")
        
        guarantees = [f"newpc{offset}", f"newpc{offset+1}", f"newpc{offset+2}", f"newpc{offset+3}"]
        if nibble < 7:
            guarantees.append(f"cout{nibble}")
        
        return Component(
            name=f"jz_nibble_{nibble}",
            constraints=constraints,
            assumptions=assumptions,
            guarantees=guarantees
        )
    
    def generate_jnz_nibble(self, nibble: int) -> Component:
        """Generate JNZ nibble - jump if zero flag is not set"""
        offset = nibble * 4
        constraints = []
        
        # Zero flag
        constraints.append("zflag=0")  # Example: zero flag is not set
        
        # Target address nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"target{bit}={(i+nibble)%2}")
        
        # Current PC nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"pc{bit}={(i+nibble+1)%2}")
        
        # PC increment logic (same as JZ)
        if nibble == 0:
            constraints.append("s0=(pc0+0)")
            constraints.append("s1=(pc1+0)")
            constraints.append("s2=(pc2+1)")
            constraints.append("c2=pc2")
            constraints.append("s3=(pc3+c2)")
            constraints.append("cout0=(pc3&c2)")
        else:
            constraints.append("cin=0")
            if nibble == 1:
                constraints.append(f"s{offset}=(pc{offset}+cin)")
                constraints.append(f"c{offset}=(pc{offset}&cin)")
            else:
                constraints.append(f"s{offset}=(pc{offset}+cin)")
                constraints.append(f"c{offset}=(pc{offset}&cin)")
            
            for i in range(1, 4):
                bit = offset + i
                constraints.append(f"s{bit}=(pc{bit}+c{bit-1})")
                constraints.append(f"c{bit}=(pc{bit}&c{bit-1})")
            
            if nibble < 7:
                constraints.append(f"cout{nibble}=c{offset+3}")
        
        # Conditional: if zflag=0 (NOT zero), use target; else use incremented PC
        for i in range(4):
            bit = offset + i
            if nibble == 0 or i > 0:
                idx = bit if nibble > 0 else i
                constraints.append(f"newpc{bit}=(((zflag+1)&target{bit})|(zflag&s{idx}))")
            else:
                constraints.append(f"newpc{bit}=(((zflag+1)&target{bit})|(zflag&s{i}))")
        
        assumptions = ["zflag=0|zflag=1"]
        if nibble > 0:
            assumptions.append("cin=0|cin=1")
        
        guarantees = [f"newpc{offset}", f"newpc{offset+1}", f"newpc{offset+2}", f"newpc{offset+3}"]
        if nibble < 7:
            guarantees.append(f"cout{nibble}")
        
        return Component(
            name=f"jnz_nibble_{nibble}",
            constraints=constraints,
            assumptions=assumptions,
            guarantees=guarantees
        )
    
    def generate_call_nibble(self, nibble: int) -> Component:
        """Generate CALL nibble - save return address and jump"""
        offset = nibble * 4
        constraints = []
        
        # Target address nibble (call destination)
        for i in range(4):
            bit = offset + i
            constraints.append(f"target{bit}={(i+nibble)%2}")
        
        # Current PC nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"pc{bit}={(i+nibble+1)%2}")
        
        # Stack pointer nibble (for saving return address)
        for i in range(4):
            bit = offset + i
            constraints.append(f"sp{bit}={(i+nibble+2)%2}")
        
        # Calculate return address (PC + 4)
        if nibble == 0:
            constraints.append("r0=(pc0+0)")
            constraints.append("r1=(pc1+0)")
            constraints.append("r2=(pc2+1)")
            constraints.append("c2=pc2")
            constraints.append("r3=(pc3+c2)")
            constraints.append("cout0=(pc3&c2)")
        else:
            constraints.append("cin=0")
            if nibble == 1:
                constraints.append(f"r{offset}=(pc{offset}+cin)")
                constraints.append(f"c{offset}=(pc{offset}&cin)")
            else:
                constraints.append(f"r{offset}=(pc{offset}+cin)")
                constraints.append(f"c{offset}=(pc{offset}&cin)")
            
            for i in range(1, 4):
                bit = offset + i
                constraints.append(f"r{bit}=(pc{bit}+c{bit-1})")
                constraints.append(f"c{bit}=(pc{bit}&c{bit-1})")
            
            if nibble < 7:
                constraints.append(f"cout{nibble}=c{offset+3}")
        
        # Save return address nibble to stack
        for i in range(4):
            bit = offset + i
            if nibble == 0:
                constraints.append(f"stack{bit}=r{i}")
            else:
                constraints.append(f"stack{bit}=r{bit}")
        
        # New PC = target address
        for i in range(4):
            bit = offset + i
            constraints.append(f"newpc{bit}=target{bit}")
        
        # New SP = SP - 4 (decrement for stack push)
        # For simplicity, we just track that SP changes
        constraints.append(f"spchanged{nibble}=1")
        
        assumptions = []
        if nibble > 0:
            assumptions.append("cin=0|cin=1")
        
        guarantees = [f"newpc{offset}", f"newpc{offset+1}", f"newpc{offset+2}", f"newpc{offset+3}",
                     f"stack{offset}", f"stack{offset+1}", f"stack{offset+2}", f"stack{offset+3}",
                     f"spchanged{nibble}"]
        if nibble < 7:
            guarantees.append(f"cout{nibble}")
        
        return Component(
            name=f"call_nibble_{nibble}",
            constraints=constraints,
            assumptions=assumptions,
            guarantees=guarantees
        )
    
    def generate_ret_nibble(self, nibble: int) -> Component:
        """Generate RET nibble - restore PC from return address stack"""
        offset = nibble * 4
        constraints = []
        
        # Stack pointer nibble
        for i in range(4):
            bit = offset + i
            constraints.append(f"sp{bit}={(i+nibble)%2}")
        
        # Return address from stack
        for i in range(4):
            bit = offset + i
            constraints.append(f"retaddr{bit}={(i+nibble+3)%2}")  # Example return address
        
        # Current PC nibble (not used in RET)
        for i in range(4):
            bit = offset + i
            constraints.append(f"pc{bit}={(i+nibble+1)%2}")
        
        # New PC = return address from stack
        for i in range(4):
            bit = offset + i
            constraints.append(f"newpc{bit}=retaddr{bit}")
        
        # New SP = SP + 4 (increment after stack pop)
        # For simplicity, we just track that SP changes
        constraints.append(f"spchanged{nibble}=1")
        
        guarantees = [f"newpc{offset}", f"newpc{offset+1}", f"newpc{offset+2}", f"newpc{offset+3}",
                     f"spchanged{nibble}"]
        
        return Component(
            name=f"ret_nibble_{nibble}",
            constraints=constraints,
            guarantees=guarantees
        )
    
    def generate_zero_flag_aggregator(self, num_nibbles: int = 8) -> Component:
        """Generate component to check if all nibbles of result are zero"""
        constraints = []
        
        # Check each nibble for all zeros
        for i in range(num_nibbles):
            offset = i * 4
            # For each nibble, check if all 4 bits are zero
            constraints.append(f"r{offset}=0")
            constraints.append(f"r{offset+1}=0")
            constraints.append(f"r{offset+2}=0")
            constraints.append(f"r{offset+3}=0")
            
            # Nibble is zero if all bits are zero
            constraints.append(f"nz{i}=((r{offset}+1)&(r{offset+1}+1)&(r{offset+2}+1)&(r{offset+3}+1))")
        
        # Zero flag is set if all nibbles are zero
        zflag_expr = "&".join([f"nz{i}" for i in range(num_nibbles)])
        constraints.append(f"zflag=({zflag_expr})")
        
        return Component(
            name="zero_flag_aggregator",
            constraints=constraints,
            guarantees=["zflag"]
        )
    
    def generate_pc_carry_link(self, from_nibble: int, to_nibble: int) -> Component:
        """Generate carry propagation for PC increment"""
        return Component(
            name=f"pc_carry_{from_nibble}_to_{to_nibble}",
            constraints=[f"cout{from_nibble}=0", f"cin=cout{from_nibble}"],
            assumptions=[f"cout{from_nibble}=0|cout{from_nibble}=1"],
            guarantees=[f"cin=cout{from_nibble}"]
        )

def generate_control_flow_instruction(instruction: str) -> List[Component]:
    """Generate all components for a control flow instruction"""
    components = []
    cf_gen = ControlFlowGenerator()
    
    if instruction == "JMP":
        # Unconditional jump - simple copy of target to PC
        for i in range(8):
            components.append(cf_gen.generate_jmp_nibble(i))
    
    elif instruction == "JZ":
        # Jump if zero - need zero flag check
        # First generate the zero flag aggregator
        components.append(cf_gen.generate_zero_flag_aggregator())
        
        # Then generate conditional jump nibbles
        for i in range(8):
            components.append(cf_gen.generate_jz_nibble(i))
        
        # Add PC increment carry links
        for i in range(7):
            components.append(cf_gen.generate_pc_carry_link(i, i+1))
    
    elif instruction == "JNZ":
        # Jump if not zero - inverse of JZ
        components.append(cf_gen.generate_zero_flag_aggregator())
        
        for i in range(8):
            components.append(cf_gen.generate_jnz_nibble(i))
        
        for i in range(7):
            components.append(cf_gen.generate_pc_carry_link(i, i+1))
    
    elif instruction == "CALL":
        # Function call - save return address and jump
        for i in range(8):
            components.append(cf_gen.generate_call_nibble(i))
        
        # PC increment carry links for return address calculation
        for i in range(7):
            components.append(cf_gen.generate_pc_carry_link(i, i+1))
    
    elif instruction == "RET":
        # Return from function - restore PC from stack
        for i in range(8):
            components.append(cf_gen.generate_ret_nibble(i))
    
    return components