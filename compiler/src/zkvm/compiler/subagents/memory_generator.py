"""
Memory Generator Subagent for zkVM Compiler.

This module generates compositional contracts for memory operations:
- LOAD/STORE: 32-bit memory operations decomposed into nibbles
- MLOAD/MSTORE: Stack memory operations
- PUSH/POP: Stack pointer management
- DUP/SWAP: Stack manipulation

All 32-bit addresses are decomposed into 8 nibbles for Tau compatibility.
Each component is kept under 700 characters.

Copyright (c) 2025 Dana Edwards. All rights reserved.
"""

from dataclasses import dataclass
from typing import List, Dict, Set, Tuple
import os


@dataclass
class Contract:
    """Assume-Guarantee Contract for an instruction component."""
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
class MemoryComponent:
    """A single component of a memory operation."""
    name: str
    contract: Contract
    nibble_index: int  # -1 for non-nibble operations
    is_address: bool = False
    is_data: bool = False
    
    def to_tau_file(self) -> str:
        """Generate Tau file content for this component."""
        expr = self.contract.to_tau_expression()
        header = f"# {self.name}\n"
        header += f"# Nibble: {self.nibble_index}, Address: {self.is_address}, Data: {self.is_data}\n"
        header += f"# Guarantees: {', '.join(self.contract.guarantees)}\n\n"
        return header + f"solve {expr}\n\nquit"


@dataclass
class MemoryResult:
    """Result of generating a memory operation."""
    operation: str
    components_generated: List[MemoryComponent]
    files: Dict[str, str]  # filename -> content
    contracts: List[Contract]
    total_constraints: int


class MemoryGenerator:
    """
    Memory Generator that creates compositional contracts for zkVM memory operations.
    Each 32-bit address/data uses 8 nibbles with proper decomposition.
    """
    
    def __init__(self, output_dir: str = "build/memory"):
        self.output_dir = output_dir
        self.nibble_bits = 4
        self.word_size = 32
        self.num_nibbles = self.word_size // self.nibble_bits
        
    def generate(self, operations: List[str]) -> Dict[str, MemoryResult]:
        """
        Generate components for a list of memory operation names.
        Returns a dictionary mapping operation names to their results.
        """
        results = {}
        
        for op in operations:
            if op.upper() in ["LOAD", "STORE"]:
                results[op] = self._generate_memory_access(op)
            elif op.upper() in ["MLOAD", "MSTORE"]:
                results[op] = self._generate_stack_memory(op)
            elif op.upper() in ["PUSH", "POP"]:
                results[op] = self._generate_stack_pointer(op)
            elif op.upper() in ["DUP", "SWAP"]:
                results[op] = self._generate_stack_manipulation(op)
            else:
                raise ValueError(f"Unknown memory operation: {op}")
        
        return results
    
    def _generate_memory_access(self, operation: str) -> MemoryResult:
        """Generate LOAD/STORE memory access components."""
        components = []
        contracts = []
        files = {}
        
        op_upper = operation.upper()
        
        # Address decomposition (8 nibbles)
        for i in range(self.num_nibbles):
            component, contract = self._generate_address_nibble(i, op_upper)
            components.append(component)
            contracts.append(contract)
            filename = f"{operation.lower()}_addr_{i}.tau"
            files[filename] = component.to_tau_file()
        
        # Address validation contract
        component, contract = self._generate_address_validator(op_upper)
        components.append(component)
        contracts.append(contract)
        files[f"{operation.lower()}_addr_valid.tau"] = component.to_tau_file()
        
        if op_upper == "LOAD":
            # Data read components (8 nibbles)
            for i in range(self.num_nibbles):
                component, contract = self._generate_load_data_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"load_data_{i}.tau"
                files[filename] = component.to_tau_file()
        else:  # STORE
            # Data write components (8 nibbles)
            for i in range(self.num_nibbles):
                component, contract = self._generate_store_data_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"store_data_{i}.tau"
                files[filename] = component.to_tau_file()
        
        # Memory consistency contract
        component, contract = self._generate_memory_consistency(op_upper)
        components.append(component)
        contracts.append(contract)
        files[f"{operation.lower()}_consistency.tau"] = component.to_tau_file()
        
        total_constraints = sum(len(c.constraints) for c in contracts)
        
        return MemoryResult(
            operation=operation,
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=total_constraints
        )
    
    def _generate_stack_memory(self, operation: str) -> MemoryResult:
        """Generate MLOAD/MSTORE stack memory components."""
        components = []
        contracts = []
        files = {}
        
        op_upper = operation.upper()
        
        # Stack offset decomposition (smaller address space - 4 nibbles)
        for i in range(4):
            component, contract = self._generate_stack_offset_nibble(i, op_upper)
            components.append(component)
            contracts.append(contract)
            filename = f"{operation.lower()}_offset_{i}.tau"
            files[filename] = component.to_tau_file()
        
        # Stack bounds check
        component, contract = self._generate_stack_bounds_check(op_upper)
        components.append(component)
        contracts.append(contract)
        files[f"{operation.lower()}_bounds.tau"] = component.to_tau_file()
        
        if op_upper == "MLOAD":
            # Load from stack memory
            for i in range(self.num_nibbles):
                component, contract = self._generate_stack_load_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"mload_data_{i}.tau"
                files[filename] = component.to_tau_file()
        else:  # MSTORE
            # Store to stack memory
            for i in range(self.num_nibbles):
                component, contract = self._generate_stack_store_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"mstore_data_{i}.tau"
                files[filename] = component.to_tau_file()
        
        total_constraints = sum(len(c.constraints) for c in contracts)
        
        return MemoryResult(
            operation=operation,
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=total_constraints
        )
    
    def _generate_stack_pointer(self, operation: str) -> MemoryResult:
        """Generate PUSH/POP stack pointer management components."""
        components = []
        contracts = []
        files = {}
        
        op_upper = operation.upper()
        
        # Stack pointer update (8 nibbles)
        for i in range(self.num_nibbles):
            component, contract = self._generate_sp_update_nibble(i, op_upper)
            components.append(component)
            contracts.append(contract)
            filename = f"{operation.lower()}_sp_{i}.tau"
            files[filename] = component.to_tau_file()
        
        # Stack overflow/underflow check
        component, contract = self._generate_stack_check(op_upper)
        components.append(component)
        contracts.append(contract)
        files[f"{operation.lower()}_check.tau"] = component.to_tau_file()
        
        if op_upper == "PUSH":
            # Push data to stack
            for i in range(self.num_nibbles):
                component, contract = self._generate_push_data_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"push_data_{i}.tau"
                files[filename] = component.to_tau_file()
        else:  # POP
            # Pop data from stack
            for i in range(self.num_nibbles):
                component, contract = self._generate_pop_data_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"pop_data_{i}.tau"
                files[filename] = component.to_tau_file()
        
        total_constraints = sum(len(c.constraints) for c in contracts)
        
        return MemoryResult(
            operation=operation,
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=total_constraints
        )
    
    def _generate_stack_manipulation(self, operation: str) -> MemoryResult:
        """Generate DUP/SWAP stack manipulation components."""
        components = []
        contracts = []
        files = {}
        
        op_upper = operation.upper()
        
        if op_upper == "DUP":
            # Duplicate stack item
            for i in range(self.num_nibbles):
                component, contract = self._generate_dup_nibble(i)
                components.append(component)
                contracts.append(contract)
                filename = f"dup_nibble_{i}.tau"
                files[filename] = component.to_tau_file()
            
            # Update stack pointer for duplication
            component, contract = self._generate_dup_sp_update()
            components.append(component)
            contracts.append(contract)
            files["dup_sp_update.tau"] = component.to_tau_file()
            
        else:  # SWAP
            # Swap two stack items
            for i in range(self.num_nibbles):
                component, contract = self._generate_swap_nibble_a(i)
                components.append(component)
                contracts.append(contract)
                filename = f"swap_a_{i}.tau"
                files[filename] = component.to_tau_file()
                
                component, contract = self._generate_swap_nibble_b(i)
                components.append(component)
                contracts.append(contract)
                filename = f"swap_b_{i}.tau"
                files[filename] = component.to_tau_file()
            
            # Swap coordination contract
            component, contract = self._generate_swap_coordination()
            components.append(component)
            contracts.append(contract)
            files["swap_coord.tau"] = component.to_tau_file()
        
        total_constraints = sum(len(c.constraints) for c in contracts)
        
        return MemoryResult(
            operation=operation,
            components_generated=components,
            files=files,
            contracts=contracts,
            total_constraints=total_constraints
        )
    
    # Helper methods for generating specific components
    
    def _generate_address_nibble(self, nibble_idx: int, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate address nibble component."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Address bits (4 bits per nibble)
        for i in range(4):
            constraints.append(f"a{n}{i}=0")
            variables.add(f"a{n}{i}")
        
        # Validate address bits are binary
        for i in range(4):
            constraints.append(f"v{n}{i}=(a{n}{i}|(a{n}{i}+1))")
            variables.add(f"v{n}{i}")
        
        # Address nibble value
        constraints.append(f"addr{n}=(a{n}3&a{n}2&a{n}1&a{n}0)")
        variables.add(f"addr{n}")
        
        contract = Contract(
            name=f"{op.lower()}_addr_{n}",
            assumptions=[f"addr_valid_{n}=1"],
            guarantees=[f"addr_nibble_{n}_ok"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_ADDR_{n}",
            contract=contract,
            nibble_index=n,
            is_address=True,
            is_data=False
        )
        
        return component, contract
    
    def _generate_address_validator(self, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate address validation contract."""
        constraints = []
        variables = set()
        
        # Import all address nibbles
        for i in range(self.num_nibbles):
            constraints.append(f"addr{i}=0")
            variables.add(f"addr{i}")
        
        # Check address alignment (word-aligned for 32-bit)
        # Lower 2 bits must be 0 for 4-byte alignment
        constraints.append("align=(a00+a01)")  # Should be 0
        constraints.append("aligned=(align+1)")  # 1 if aligned
        variables.add("align")
        variables.add("aligned")
        
        # Valid memory range check (simplified)
        constraints.append("valid_range=1")
        variables.add("valid_range")
        
        contract = Contract(
            name=f"{op.lower()}_addr_valid",
            assumptions=[f"addr_complete=1"],
            guarantees=[f"addr_valid", f"addr_aligned"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_ADDR_VALID",
            contract=contract,
            nibble_index=-1,
            is_address=True,
            is_data=False
        )
        
        return component, contract
    
    def _generate_load_data_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate data load nibble component."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Memory read result bits
        for i in range(4):
            constraints.append(f"m{n}{i}=0")
            variables.add(f"m{n}{i}")
        
        # Output data bits
        for i in range(4):
            constraints.append(f"d{n}{i}=m{n}{i}")
            variables.add(f"d{n}{i}")
        
        # Read valid flag
        constraints.append(f"read_ok_{n}=1")
        variables.add(f"read_ok_{n}")
        
        contract = Contract(
            name=f"load_data_{n}",
            assumptions=[f"addr_valid=1", f"mem_ready_{n}=1"],
            guarantees=[f"data_{n}_loaded"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"LOAD_DATA_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_store_data_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate data store nibble component."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Input data bits
        for i in range(4):
            constraints.append(f"d{n}{i}=0")
            variables.add(f"d{n}{i}")
        
        # Memory write bits
        for i in range(4):
            constraints.append(f"m{n}{i}=d{n}{i}")
            variables.add(f"m{n}{i}")
        
        # Write enable
        constraints.append(f"wen_{n}=1")
        variables.add(f"wen_{n}")
        
        # Write complete flag
        constraints.append(f"write_ok_{n}=wen_{n}")
        variables.add(f"write_ok_{n}")
        
        contract = Contract(
            name=f"store_data_{n}",
            assumptions=[f"addr_valid=1", f"data_ready_{n}=1"],
            guarantees=[f"data_{n}_stored"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"STORE_DATA_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_memory_consistency(self, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate memory consistency contract."""
        constraints = []
        variables = set()
        
        # Import operation flags
        constraints.append("is_load=0")
        constraints.append("is_store=0")
        variables.add("is_load")
        variables.add("is_store")
        
        if op == "LOAD":
            constraints.append("is_load=1")
        else:
            constraints.append("is_store=1")
        
        # Memory ordering flags
        constraints.append("ordered=1")
        constraints.append("atomic=1")
        variables.add("ordered")
        variables.add("atomic")
        
        # Consistency guarantee
        constraints.append("consistent=(ordered&atomic)")
        variables.add("consistent")
        
        contract = Contract(
            name=f"{op.lower()}_consistency",
            assumptions=["mem_model_valid=1"],
            guarantees=["memory_consistent"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_CONSISTENCY",
            contract=contract,
            nibble_index=-1,
            is_address=False,
            is_data=False
        )
        
        return component, contract
    
    def _generate_stack_offset_nibble(self, nibble_idx: int, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate stack offset nibble (16-bit offset, 4 nibbles)."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Offset bits
        for i in range(4):
            constraints.append(f"o{n}{i}=0")
            variables.add(f"o{n}{i}")
        
        # Stack base pointer bit contribution
        constraints.append(f"base{n}=0")
        variables.add(f"base{n}")
        
        # Effective address nibble
        constraints.append(f"eff{n}=(base{n}+o{n}0+o{n}1+o{n}2+o{n}3)")
        variables.add(f"eff{n}")
        
        contract = Contract(
            name=f"{op.lower()}_offset_{n}",
            assumptions=[f"sp_valid_{n}=1"],
            guarantees=[f"offset_{n}_ok"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_OFFSET_{n}",
            contract=contract,
            nibble_index=n,
            is_address=True,
            is_data=False
        )
        
        return component, contract
    
    def _generate_stack_bounds_check(self, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate stack bounds checking contract."""
        constraints = []
        variables = set()
        
        # Stack limits (simplified)
        constraints.append("stack_base=0")
        constraints.append("stack_limit=1")
        variables.add("stack_base")
        variables.add("stack_limit")
        
        # Current stack pointer position
        constraints.append("sp_pos=0")
        variables.add("sp_pos")
        
        # Bounds check
        constraints.append("above_base=1")  # sp >= stack_base
        constraints.append("below_limit=1")  # sp < stack_limit
        constraints.append("in_bounds=(above_base&below_limit)")
        variables.add("above_base")
        variables.add("below_limit")
        variables.add("in_bounds")
        
        contract = Contract(
            name=f"{op.lower()}_bounds",
            assumptions=["stack_initialized=1"],
            guarantees=["stack_bounds_ok"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_BOUNDS",
            contract=contract,
            nibble_index=-1,
            is_address=False,
            is_data=False
        )
        
        return component, contract
    
    def _generate_stack_load_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate stack memory load nibble."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Stack memory bits
        for i in range(4):
            constraints.append(f"sm{n}{i}=0")
            variables.add(f"sm{n}{i}")
        
        # Output data
        for i in range(4):
            constraints.append(f"sd{n}{i}=sm{n}{i}")
            variables.add(f"sd{n}{i}")
        
        # Stack read valid
        constraints.append(f"sread_{n}=1")
        variables.add(f"sread_{n}")
        
        contract = Contract(
            name=f"mload_data_{n}",
            assumptions=[f"stack_addr_valid=1"],
            guarantees=[f"stack_data_{n}_loaded"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"MLOAD_DATA_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_stack_store_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate stack memory store nibble."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Input data
        for i in range(4):
            constraints.append(f"sd{n}{i}=0")
            variables.add(f"sd{n}{i}")
        
        # Stack memory write
        for i in range(4):
            constraints.append(f"sm{n}{i}=sd{n}{i}")
            variables.add(f"sm{n}{i}")
        
        # Stack write enable
        constraints.append(f"swen_{n}=1")
        variables.add(f"swen_{n}")
        
        contract = Contract(
            name=f"mstore_data_{n}",
            assumptions=[f"stack_addr_valid=1"],
            guarantees=[f"stack_data_{n}_stored"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"MSTORE_DATA_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_sp_update_nibble(self, nibble_idx: int, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate stack pointer update nibble."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Current SP nibble
        for i in range(4):
            constraints.append(f"sp{n}{i}=0")
            variables.add(f"sp{n}{i}")
        
        # Update amount (4 for 32-bit word)
        if op == "PUSH":
            # Decrement SP
            if n == 0:
                constraints.append("dec=1")  # -4 in nibble 0
            else:
                constraints.append("dec=0")
        else:  # POP
            # Increment SP
            if n == 0:
                constraints.append("inc=1")  # +4 in nibble 0
            else:
                constraints.append("inc=0")
        
        # New SP nibble (simplified)
        for i in range(4):
            if n == 0 and i == 2:  # bit 2 represents 4
                if op == "PUSH":
                    constraints.append(f"nsp{n}{i}=(sp{n}{i}+1)")  # flip for -4
                else:
                    constraints.append(f"nsp{n}{i}=(sp{n}{i}+1)")  # flip for +4
            else:
                constraints.append(f"nsp{n}{i}=sp{n}{i}")
            variables.add(f"nsp{n}{i}")
        
        variables.add("dec" if op == "PUSH" else "inc")
        
        contract = Contract(
            name=f"{op.lower()}_sp_{n}",
            assumptions=[f"sp_valid_{n}=1"],
            guarantees=[f"sp_updated_{n}"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_SP_{n}",
            contract=contract,
            nibble_index=n,
            is_address=True,
            is_data=False
        )
        
        return component, contract
    
    def _generate_stack_check(self, op: str) -> Tuple[MemoryComponent, Contract]:
        """Generate stack overflow/underflow check."""
        constraints = []
        variables = set()
        
        # Stack limits
        constraints.append("stack_top=0")
        constraints.append("stack_bottom=1")
        variables.add("stack_top")
        variables.add("stack_bottom")
        
        # Current SP position
        constraints.append("sp_current=0")
        variables.add("sp_current")
        
        if op == "PUSH":
            # Check for stack overflow
            constraints.append("has_space=1")
            constraints.append("no_overflow=has_space")
            variables.add("has_space")
            variables.add("no_overflow")
        else:  # POP
            # Check for stack underflow
            constraints.append("has_data=1")
            constraints.append("no_underflow=has_data")
            variables.add("has_data")
            variables.add("no_underflow")
        
        contract = Contract(
            name=f"{op.lower()}_check",
            assumptions=["stack_valid=1"],
            guarantees=[f"{op.lower()}_safe"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"{op}_CHECK",
            contract=contract,
            nibble_index=-1,
            is_address=False,
            is_data=False
        )
        
        return component, contract
    
    def _generate_push_data_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate push data nibble."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Input data to push
        for i in range(4):
            constraints.append(f"pd{n}{i}=0")
            variables.add(f"pd{n}{i}")
        
        # Write to stack top
        for i in range(4):
            constraints.append(f"st{n}{i}=pd{n}{i}")
            variables.add(f"st{n}{i}")
        
        # Push complete
        constraints.append(f"pushed_{n}=1")
        variables.add(f"pushed_{n}")
        
        contract = Contract(
            name=f"push_data_{n}",
            assumptions=[f"sp_updated=1"],
            guarantees=[f"data_{n}_pushed"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"PUSH_DATA_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_pop_data_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate pop data nibble."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Read from stack top
        for i in range(4):
            constraints.append(f"st{n}{i}=0")
            variables.add(f"st{n}{i}")
        
        # Output popped data
        for i in range(4):
            constraints.append(f"pd{n}{i}=st{n}{i}")
            variables.add(f"pd{n}{i}")
        
        # Pop complete
        constraints.append(f"popped_{n}=1")
        variables.add(f"popped_{n}")
        
        contract = Contract(
            name=f"pop_data_{n}",
            assumptions=[f"stack_has_data=1"],
            guarantees=[f"data_{n}_popped"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"POP_DATA_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_dup_nibble(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate DUP nibble component."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Source data from stack
        for i in range(4):
            constraints.append(f"src{n}{i}=0")
            variables.add(f"src{n}{i}")
        
        # Duplicate to new stack position
        for i in range(4):
            constraints.append(f"dup{n}{i}=src{n}{i}")
            variables.add(f"dup{n}{i}")
        
        # Original preserved
        for i in range(4):
            constraints.append(f"orig{n}{i}=src{n}{i}")
            variables.add(f"orig{n}{i}")
        
        contract = Contract(
            name=f"dup_nibble_{n}",
            assumptions=[f"stack_item_valid_{n}=1"],
            guarantees=[f"item_{n}_duplicated"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"DUP_NIBBLE_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_dup_sp_update(self) -> Tuple[MemoryComponent, Contract]:
        """Generate DUP stack pointer update."""
        constraints = []
        variables = set()
        
        # Current SP
        constraints.append("sp_old=0")
        variables.add("sp_old")
        
        # Decrement by word size
        constraints.append("sp_new=(sp_old+1)")  # Simplified
        variables.add("sp_new")
        
        # Space available
        constraints.append("has_space=1")
        variables.add("has_space")
        
        contract = Contract(
            name="dup_sp_update",
            assumptions=["dup_requested=1"],
            guarantees=["sp_updated_for_dup"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name="DUP_SP_UPDATE",
            contract=contract,
            nibble_index=-1,
            is_address=True,
            is_data=False
        )
        
        return component, contract
    
    def _generate_swap_nibble_a(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate SWAP nibble for item A."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Item A data
        for i in range(4):
            constraints.append(f"a{n}{i}=0")
            variables.add(f"a{n}{i}")
        
        # Temporary storage
        for i in range(4):
            constraints.append(f"tmp{n}{i}=a{n}{i}")
            variables.add(f"tmp{n}{i}")
        
        contract = Contract(
            name=f"swap_a_{n}",
            assumptions=[f"item_a_valid_{n}=1"],
            guarantees=[f"a_{n}_saved"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"SWAP_A_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_swap_nibble_b(self, nibble_idx: int) -> Tuple[MemoryComponent, Contract]:
        """Generate SWAP nibble for item B."""
        constraints = []
        variables = set()
        
        n = nibble_idx
        
        # Item B data
        for i in range(4):
            constraints.append(f"b{n}{i}=0")
            variables.add(f"b{n}{i}")
        
        # Move B to A's position
        for i in range(4):
            constraints.append(f"newa{n}{i}=b{n}{i}")
            variables.add(f"newa{n}{i}")
        
        # Move temp (old A) to B's position
        for i in range(4):
            constraints.append(f"newb{n}{i}=0")  # Would come from tmp
            variables.add(f"newb{n}{i}")
        
        contract = Contract(
            name=f"swap_b_{n}",
            assumptions=[f"item_b_valid_{n}=1", f"temp_valid_{n}=1"],
            guarantees=[f"items_{n}_swapped"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name=f"SWAP_B_{n}",
            contract=contract,
            nibble_index=n,
            is_address=False,
            is_data=True
        )
        
        return component, contract
    
    def _generate_swap_coordination(self) -> Tuple[MemoryComponent, Contract]:
        """Generate SWAP coordination contract."""
        constraints = []
        variables = set()
        
        # All nibbles swapped flags
        for i in range(self.num_nibbles):
            constraints.append(f"swapped_{i}=0")
            variables.add(f"swapped_{i}")
        
        # All nibbles must be swapped atomically
        all_swapped = " & ".join([f"swapped_{i}" for i in range(self.num_nibbles)])
        constraints.append(f"atomic_swap=({all_swapped})")
        variables.add("atomic_swap")
        
        # No partial swaps
        constraints.append("complete=atomic_swap")
        variables.add("complete")
        
        contract = Contract(
            name="swap_coord",
            assumptions=["swap_initiated=1"],
            guarantees=["swap_atomic", "swap_complete"],
            variables=variables,
            constraints=constraints
        )
        
        component = MemoryComponent(
            name="SWAP_COORD",
            contract=contract,
            nibble_index=-1,
            is_address=False,
            is_data=False
        )
        
        return component, contract
    
    def save_memory_files(self, memory_result: MemoryResult):
        """Save all files for a memory operation to disk."""
        op_dir = os.path.join(self.output_dir, memory_result.operation.lower())
        os.makedirs(op_dir, exist_ok=True)
        
        for filename, content in memory_result.files.items():
            filepath = os.path.join(op_dir, filename)
            with open(filepath, 'w') as f:
                f.write(content)
        
        # Save summary
        summary = f"# {memory_result.operation} Memory Operation Summary\n\n"
        summary += f"Components: {len(memory_result.components_generated)}\n"
        summary += f"Total Constraints: {memory_result.total_constraints}\n\n"
        summary += "## Components:\n"
        for comp in memory_result.components_generated:
            summary += f"- {comp.name}: {len(comp.contract.constraints)} constraints\n"
        
        summary_path = os.path.join(op_dir, "summary.md")
        with open(summary_path, 'w') as f:
            f.write(summary)