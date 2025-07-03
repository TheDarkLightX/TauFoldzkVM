#!/usr/bin/env python3
"""
Execution Engine Generator Subagent for TauFoldZKVM
Implements VM execution state machine using compositional contracts.
Handles EXEC, STEP, TRACE operations for program execution.

Each component handles:
- Program Counter (PC) management
- Register state transitions  
- Flag updates (zero, carry, overflow, negative)
- Instruction fetch/decode/execute cycle
- Execution trace generation

Copyright (c) 2025 Dana Edwards. All rights reserved.
"""

from dataclasses import dataclass, field
from typing import List, Dict, Set, Tuple, Optional
from enum import IntEnum
import os

class ExecutionPhase(IntEnum):
    """Execution phases in the VM state machine."""
    FETCH = 0
    DECODE = 1
    EXECUTE = 2
    WRITEBACK = 3
    UPDATE_PC = 4

class FlagType(IntEnum):
    """CPU flags for condition tracking."""
    ZERO = 0
    CARRY = 1
    OVERFLOW = 2
    NEGATIVE = 3
    HALT = 4

@dataclass
class Contract:
    """Assume-Guarantee Contract for an execution component."""
    name: str
    assumptions: List[str]
    guarantees: List[str]
    variables: Set[str]
    constraints: List[str]
    max_chars: int = 700
    
    def to_tau_expression(self) -> str:
        """Convert contract to Tau expression."""
        all_parts = []
        all_parts.extend(self.assumptions)
        all_parts.extend(self.constraints)
        expr = " && ".join(all_parts)
        
        if len(expr) > self.max_chars:
            raise ValueError(f"Contract {self.name} too long: {len(expr)} chars")
        
        return expr

@dataclass
class ExecutionComponent:
    """A single component of execution state machine."""
    name: str
    contract: Contract
    phase: ExecutionPhase
    nibble_index: int = -1  # -1 for non-nibble operations
    cycle: int = 0
    
    def to_tau_file(self) -> str:
        """Generate Tau file content for this component."""
        expr = self.contract.to_tau_expression()
        header = f"# {self.name}\n"
        header += f"# Phase: {self.phase.name}, Cycle: {self.cycle}\n"
        if self.nibble_index >= 0:
            header += f"# Nibble: {self.nibble_index}\n"
        header += f"# Guarantees: {', '.join(self.contract.guarantees)}\n\n"
        return header + f"solve {expr}\n\nquit"

@dataclass
class ExecutionResult:
    """Result of generating execution components."""
    operation: str
    components_generated: List[ExecutionComponent]
    files: Dict[str, str]  # filename -> content
    contracts: List[Contract]
    total_constraints: int
    cycles_required: int

class ExecutionGenerator:
    """
    Execution Engine Generator that creates compositional contracts for VM execution.
    Each instruction execution is decomposed into fetch/decode/execute/writeback phases.
    32-bit PC and registers use 8 nibbles each.
    """
    
    def __init__(self, output_dir: str = "build/execution"):
        self.output_dir = output_dir
        self.nibble_bits = 4
        self.word_size = 32
        self.num_nibbles = self.word_size // self.nibble_bits
        self.num_registers = 16  # R0-R15
        
    def generate(self, operations: List[str]) -> Dict[str, ExecutionResult]:
        """
        Generate components for execution operations.
        Supported operations: EXEC, STEP, TRACE
        """
        results = {}
        
        for op in operations:
            if op.upper() == "EXEC":
                results[op] = self._generate_exec_cycle()
            elif op.upper() == "STEP":
                results[op] = self._generate_single_step()
            elif op.upper() == "TRACE":
                results[op] = self._generate_trace_capture()
            else:
                raise ValueError(f"Unknown execution operation: {op}")
        
        return results
    
    def _generate_exec_cycle(self) -> ExecutionResult:
        """Generate full instruction execution cycle."""
        components = []
        contracts = []
        files = {}
        
        # Generate fetch phase components
        for i in range(self.num_nibbles):
            component, contract = self._generate_pc_nibble_fetch(i)
            components.append(component)
            contracts.append(contract)
            filename = f"exec_fetch_pc_nibble_{i}.tau"
            files[filename] = component.to_tau_file()
        
        # Generate instruction memory read
        component, contract = self._generate_instruction_fetch()
        components.append(component)
        contracts.append(contract)
        files["exec_fetch_instruction.tau"] = component.to_tau_file()
        
        # Generate decode phase
        component, contract = self._generate_decode_phase()
        components.append(component)
        contracts.append(contract)
        files["exec_decode.tau"] = component.to_tau_file()
        
        # Generate execute phase for each register nibble
        for reg in range(4):  # First 4 registers for demo
            for nibble in range(self.num_nibbles):
                component, contract = self._generate_register_update(reg, nibble)
                components.append(component)
                contracts.append(contract)
                filename = f"exec_reg_{reg}_nibble_{nibble}.tau"
                files[filename] = component.to_tau_file()
        
        # Generate flag updates
        for flag in FlagType:
            component, contract = self._generate_flag_update(flag)
            components.append(component)
            contracts.append(contract)
            filename = f"exec_flag_{flag.name.lower()}.tau"
            files[filename] = component.to_tau_file()
        
        # Generate PC update
        for i in range(self.num_nibbles):
            component, contract = self._generate_pc_update(i)
            components.append(component)
            contracts.append(contract)
            filename = f"exec_pc_update_nibble_{i}.tau"
            files[filename] = component.to_tau_file()
        
        return ExecutionResult(
            operation="EXEC",
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=len(components),
            cycles_required=5  # Fetch, Decode, Execute, Writeback, PC Update
        )
    
    def _generate_single_step(self) -> ExecutionResult:
        """Generate single instruction step components."""
        components = []
        contracts = []
        files = {}
        
        # State capture before step
        component, contract = self._generate_state_capture("pre")
        components.append(component)
        contracts.append(contract)
        files["step_state_pre.tau"] = component.to_tau_file()
        
        # Execute one instruction
        exec_result = self._generate_exec_cycle()
        components.extend(exec_result.components_generated)
        contracts.extend(exec_result.contracts)
        files.update({f"step_{k}": v for k, v in exec_result.files.items()})
        
        # State capture after step
        component, contract = self._generate_state_capture("post")
        components.append(component)
        contracts.append(contract)
        files["step_state_post.tau"] = component.to_tau_file()
        
        # State transition validation
        component, contract = self._generate_state_transition()
        components.append(component)
        contracts.append(contract)
        files["step_transition.tau"] = component.to_tau_file()
        
        return ExecutionResult(
            operation="STEP",
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=len(components),
            cycles_required=1
        )
    
    def _generate_trace_capture(self) -> ExecutionResult:
        """Generate execution trace components."""
        components = []
        contracts = []
        files = {}
        
        # Trace buffer initialization
        component, contract = self._generate_trace_init()
        components.append(component)
        contracts.append(contract)
        files["trace_init.tau"] = component.to_tau_file()
        
        # Generate trace entry for each execution phase
        for phase in ExecutionPhase:
            component, contract = self._generate_trace_entry(phase)
            components.append(component)
            contracts.append(contract)
            filename = f"trace_phase_{phase.name.lower()}.tau"
            files[filename] = component.to_tau_file()
        
        # Trace consistency checks
        component, contract = self._generate_trace_consistency()
        components.append(component)
        contracts.append(contract)
        files["trace_consistency.tau"] = component.to_tau_file()
        
        return ExecutionResult(
            operation="TRACE",
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=len(components),
            cycles_required=1  # Trace capture in parallel
        )
    
    def _generate_pc_nibble_fetch(self, nibble_idx: int) -> Tuple[ExecutionComponent, Contract]:
        """Generate PC nibble fetch component."""
        shift = nibble_idx * 4
        mask = 0xF << shift
        
        variables = {f"pc{nibble_idx}", f"pc_bit{shift}", f"pc_bit{shift+1}", 
                    f"pc_bit{shift+2}", f"pc_bit{shift+3}"}
        
        assumptions = [f"pc{nibble_idx} = pc{nibble_idx}"]
        
        constraints = [
            f"pc{nibble_idx} = (pc_bit{shift} + pc_bit{shift+1}*2 + "
            f"pc_bit{shift+2}*4 + pc_bit{shift+3}*8)"
        ]
        
        guarantees = [f"pc_nibble_{nibble_idx}_fetched"]
        
        contract = Contract(
            name=f"pc_nibble_fetch_{nibble_idx}",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name=f"PC Nibble {nibble_idx} Fetch",
            contract=contract,
            phase=ExecutionPhase.FETCH,
            nibble_index=nibble_idx,
            cycle=0
        )
        
        return component, contract
    
    def _generate_instruction_fetch(self) -> Tuple[ExecutionComponent, Contract]:
        """Generate instruction memory fetch."""
        variables = {"pc_full", "inst", "mem_addr", "mem_data"}
        
        assumptions = ["pc_full = pc_full", "mem_addr = pc_full"]
        
        constraints = [
            "inst = mem_data",
            "mem_addr = pc_full"
        ]
        
        guarantees = ["instruction_fetched"]
        
        contract = Contract(
            name="instruction_fetch",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name="Instruction Fetch",
            contract=contract,
            phase=ExecutionPhase.FETCH,
            cycle=0
        )
        
        return component, contract
    
    def _generate_decode_phase(self) -> Tuple[ExecutionComponent, Contract]:
        """Generate instruction decode component."""
        variables = {"inst", "opcode", "rd", "rs1", "rs2", "imm"}
        
        assumptions = ["inst = inst"]
        
        # Decode fields from instruction
        constraints = [
            "opcode = (inst & 127)",  # Lower 7 bits
            "rd = ((inst & 3968) / 128)",  # Bits 7-11
            "rs1 = ((inst & 1015808) / 4096)",  # Bits 12-19
            "rs2 = ((inst & 31457280) / 1048576)"  # Bits 20-24
        ]
        
        guarantees = ["instruction_decoded"]
        
        contract = Contract(
            name="instruction_decode",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name="Instruction Decode",
            contract=contract,
            phase=ExecutionPhase.DECODE,
            cycle=1
        )
        
        return component, contract
    
    def _generate_register_update(self, reg_idx: int, nibble_idx: int) -> Tuple[ExecutionComponent, Contract]:
        """Generate register nibble update component."""
        shift = nibble_idx * 4
        
        variables = {f"r{reg_idx}_n{nibble_idx}", f"r{reg_idx}_n{nibble_idx}_new",
                    f"alu_out_n{nibble_idx}", "rd", "write_enable"}
        
        assumptions = [
            f"r{reg_idx}_n{nibble_idx} = r{reg_idx}_n{nibble_idx}",
            "write_enable = write_enable"
        ]
        
        # Conditional update based on destination register
        constraints = [
            f"r{reg_idx}_n{nibble_idx}_new = "
            f"((rd = {reg_idx}) & write_enable) * alu_out_n{nibble_idx} + "
            f"((rd' | {reg_idx}) | write_enable') * r{reg_idx}_n{nibble_idx}"
        ]
        
        guarantees = [f"register_{reg_idx}_nibble_{nibble_idx}_updated"]
        
        contract = Contract(
            name=f"register_{reg_idx}_nibble_{nibble_idx}_update",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name=f"Register R{reg_idx} Nibble {nibble_idx} Update",
            contract=contract,
            phase=ExecutionPhase.WRITEBACK,
            nibble_index=nibble_idx,
            cycle=3
        )
        
        return component, contract
    
    def _generate_flag_update(self, flag: FlagType) -> Tuple[ExecutionComponent, Contract]:
        """Generate flag update component."""
        flag_name = flag.name.lower()
        
        variables = {f"flag_{flag_name}", f"flag_{flag_name}_new", 
                    "alu_result", "opcode"}
        
        assumptions = [f"flag_{flag_name} = flag_{flag_name}"]
        
        if flag == FlagType.ZERO:
            constraints = [f"flag_{flag_name}_new = (alu_result = 0)"]
        elif flag == FlagType.CARRY:
            constraints = [f"flag_{flag_name}_new = alu_carry_out"]
        elif flag == FlagType.OVERFLOW:
            constraints = [f"flag_{flag_name}_new = alu_overflow"]
        elif flag == FlagType.NEGATIVE:
            constraints = [f"flag_{flag_name}_new = alu_result_sign"]
        elif flag == FlagType.HALT:
            constraints = [f"flag_{flag_name}_new = (opcode = 255)"]  # HALT opcode
        
        guarantees = [f"flag_{flag_name}_updated"]
        
        contract = Contract(
            name=f"flag_{flag_name}_update",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name=f"Flag {flag_name.upper()} Update",
            contract=contract,
            phase=ExecutionPhase.EXECUTE,
            cycle=2
        )
        
        return component, contract
    
    def _generate_pc_update(self, nibble_idx: int) -> Tuple[ExecutionComponent, Contract]:
        """Generate PC nibble update component."""
        variables = {f"pc_n{nibble_idx}", f"pc_n{nibble_idx}_new",
                    f"pc_inc_n{nibble_idx}", f"branch_target_n{nibble_idx}",
                    "branch_taken"}
        
        assumptions = [f"pc_n{nibble_idx} = pc_n{nibble_idx}"]
        
        # PC = branch_taken ? branch_target : PC + 4
        constraints = [
            f"pc_inc_n{nibble_idx} = pc_n{nibble_idx} + {1 if nibble_idx == 0 else 0}",
            f"pc_n{nibble_idx}_new = branch_taken * branch_target_n{nibble_idx} + "
            f"branch_taken' * pc_inc_n{nibble_idx}"
        ]
        
        guarantees = [f"pc_nibble_{nibble_idx}_updated"]
        
        contract = Contract(
            name=f"pc_nibble_{nibble_idx}_update",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name=f"PC Nibble {nibble_idx} Update",
            contract=contract,
            phase=ExecutionPhase.UPDATE_PC,
            nibble_index=nibble_idx,
            cycle=4
        )
        
        return component, contract
    
    def _generate_state_capture(self, phase: str) -> Tuple[ExecutionComponent, Contract]:
        """Generate state capture component."""
        variables = {"pc_snapshot", "flags_snapshot"}
        
        for i in range(4):  # First 4 registers
            variables.add(f"r{i}_snapshot")
        
        assumptions = ["capture_enable = 1"]
        
        constraints = [
            "pc_snapshot = pc_full",
            "flags_snapshot = (flag_zero + flag_carry*2 + flag_overflow*4 + flag_negative*8)"
        ]
        
        for i in range(4):
            constraints.append(f"r{i}_snapshot = r{i}_full")
        
        guarantees = [f"state_{phase}_captured"]
        
        contract = Contract(
            name=f"state_capture_{phase}",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name=f"State Capture {phase.upper()}",
            contract=contract,
            phase=ExecutionPhase.FETCH if phase == "pre" else ExecutionPhase.UPDATE_PC,
            cycle=0 if phase == "pre" else 5
        )
        
        return component, contract
    
    def _generate_state_transition(self) -> Tuple[ExecutionComponent, Contract]:
        """Generate state transition validation."""
        variables = {"pc_pre", "pc_post", "transition_valid"}
        
        assumptions = ["pc_pre = pc_pre", "pc_post = pc_post"]
        
        # Validate PC advanced or branched correctly
        constraints = [
            "transition_valid = ((pc_post = pc_pre + 4) | branch_taken)"
        ]
        
        guarantees = ["state_transition_valid"]
        
        contract = Contract(
            name="state_transition",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name="State Transition Validation",
            contract=contract,
            phase=ExecutionPhase.UPDATE_PC,
            cycle=5
        )
        
        return component, contract
    
    def _generate_trace_init(self) -> Tuple[ExecutionComponent, Contract]:
        """Generate trace buffer initialization."""
        variables = {"trace_ptr", "trace_size", "trace_enable"}
        
        assumptions = ["trace_enable = 1"]
        
        constraints = [
            "trace_ptr = 0",
            "trace_size = 0"
        ]
        
        guarantees = ["trace_initialized"]
        
        contract = Contract(
            name="trace_init",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name="Trace Initialization",
            contract=contract,
            phase=ExecutionPhase.FETCH,
            cycle=0
        )
        
        return component, contract
    
    def _generate_trace_entry(self, phase: ExecutionPhase) -> Tuple[ExecutionComponent, Contract]:
        """Generate trace entry for execution phase."""
        phase_name = phase.name.lower()
        
        variables = {"trace_ptr", f"trace_{phase_name}_pc", 
                    f"trace_{phase_name}_data", "cycle_count"}
        
        assumptions = ["trace_enable = 1"]
        
        constraints = [
            f"trace_{phase_name}_pc = pc_full",
            f"trace_{phase_name}_data = phase_data_{phase.value}",
            "trace_ptr = trace_ptr + 1"
        ]
        
        guarantees = [f"trace_{phase_name}_recorded"]
        
        contract = Contract(
            name=f"trace_{phase_name}",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name=f"Trace {phase.name} Entry",
            contract=contract,
            phase=phase,
            cycle=phase.value
        )
        
        return component, contract
    
    def _generate_trace_consistency(self) -> Tuple[ExecutionComponent, Contract]:
        """Generate trace consistency check."""
        variables = {"trace_start", "trace_end", "trace_valid"}
        
        assumptions = ["trace_start = trace_start", "trace_end = trace_end"]
        
        # Ensure trace entries are sequential
        constraints = [
            "trace_valid = (trace_end >= trace_start)",
            "trace_valid = trace_valid & ((trace_end + trace_start + 1) = trace_size * 2)"
        ]
        
        guarantees = ["trace_consistent"]
        
        contract = Contract(
            name="trace_consistency",
            assumptions=assumptions,
            guarantees=guarantees,
            variables=variables,
            constraints=constraints
        )
        
        component = ExecutionComponent(
            name="Trace Consistency Check",
            contract=contract,
            phase=ExecutionPhase.UPDATE_PC,
            cycle=5
        )
        
        return component, contract
    
    def save_components(self, results: Dict[str, ExecutionResult]):
        """Save all generated components to files."""
        os.makedirs(self.output_dir, exist_ok=True)
        
        for op, result in results.items():
            op_dir = os.path.join(self.output_dir, op.lower())
            os.makedirs(op_dir, exist_ok=True)
            
            # Save each component file
            for filename, content in result.files.items():
                filepath = os.path.join(op_dir, filename)
                with open(filepath, 'w') as f:
                    f.write(content)
            
            # Save summary
            summary_path = os.path.join(op_dir, "summary.txt")
            with open(summary_path, 'w') as f:
                f.write(f"Operation: {result.operation}\n")
                f.write(f"Components: {len(result.components_generated)}\n")
                f.write(f"Contracts: {len(result.contracts)}\n")
                f.write(f"Total Constraints: {result.total_constraints}\n")
                f.write(f"Cycles Required: {result.cycles_required}\n\n")
                
                f.write("Components:\n")
                for comp in result.components_generated:
                    f.write(f"  - {comp.name} (Phase: {comp.phase.name}, Cycle: {comp.cycle})\n")


if __name__ == "__main__":
    # Example usage
    generator = ExecutionGenerator()
    
    # Generate execution components
    operations = ["EXEC", "STEP", "TRACE"]
    results = generator.generate(operations)
    
    # Save to files
    generator.save_components(results)
    
    # Print summary
    for op, result in results.items():
        print(f"\n{op}:")
        print(f"  Components: {len(result.components_generated)}")
        print(f"  Files: {len(result.files)}")
        print(f"  Constraints: {result.total_constraints}")
        print(f"  Cycles: {result.cycles_required}")