"""ProtoStar folding generator subagent for zkVM compiler.

This module implements the ProtoStar folding scheme with noise vector tracking,
handling FOLD, UNFOLD, and COMMIT operations for recursive proof composition.

Copyright (c) 2025 Dana Edwards. All rights reserved.
"""

from typing import Dict, List, Optional, Tuple, Set, Any
from dataclasses import dataclass, field
from enum import Enum
import hashlib
import json


class FoldingOp(Enum):
    """Folding operation types."""
    FOLD = "FOLD"
    UNFOLD = "UNFOLD"
    COMMIT = "COMMIT"
    ACCUMULATE = "ACCUMULATE"
    VERIFY = "VERIFY"


@dataclass
class NoiseVector:
    """Noise vector for ProtoStar folding."""
    dimension: int
    elements: List[int]
    bound: int
    
    def __post_init__(self):
        """Validate noise vector."""
        if len(self.elements) != self.dimension:
            raise ValueError(f"Noise vector dimension mismatch")
        if any(abs(e) > self.bound for e in self.elements):
            raise ValueError(f"Noise element exceeds bound")


@dataclass
class FoldedInstance:
    """Folded instance with accumulator state."""
    witness: List[int]
    statement: List[int]
    error: List[int]
    noise: NoiseVector
    round: int
    commitment: Optional[str] = None


@dataclass
class Accumulator:
    """ProtoStar accumulator state."""
    instances: List[FoldedInstance]
    error_bound: int
    noise_bound: int
    current_round: int
    commitments: List[str] = field(default_factory=list)


@dataclass
class FoldingCircuit:
    """Circuit for folding operations."""
    operation: FoldingOp
    inputs: List[str]
    outputs: List[str]
    constraints: List[str]
    noise_params: Dict[str, Any]


@dataclass
class FoldingResult:
    """Result of folding generation."""
    success: bool
    circuits: List[FoldingCircuit]
    accumulator: Optional[Accumulator] = None
    commitments: Dict[str, str] = field(default_factory=dict)
    errors: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)


class NoiseManager:
    """Manages noise vectors for ProtoStar folding."""
    
    def __init__(self, dimension: int, bound: int):
        """Initialize noise manager.
        
        Args:
            dimension: Noise vector dimension
            bound: Maximum noise element value
        """
        self.dimension = dimension
        self.bound = bound
        self.noise_history: List[NoiseVector] = []
    
    def generate_noise(self, seed: int) -> NoiseVector:
        """Generate deterministic noise vector.
        
        Args:
            seed: Random seed for noise generation
            
        Returns:
            Generated noise vector
        """
        # Simple deterministic noise generation
        elements = []
        for i in range(self.dimension):
            val = (seed * (i + 1) + i * i) % (2 * self.bound + 1)
            elements.append(val - self.bound)
        
        noise = NoiseVector(self.dimension, elements, self.bound)
        self.noise_history.append(noise)
        return noise
    
    def combine_noise(self, n1: NoiseVector, n2: NoiseVector, 
                     challenge: int) -> NoiseVector:
        """Combine two noise vectors.
        
        Args:
            n1: First noise vector
            n2: Second noise vector
            challenge: Folding challenge
            
        Returns:
            Combined noise vector
        """
        if n1.dimension != n2.dimension:
            raise ValueError("Noise dimension mismatch")
        
        combined = []
        for i in range(n1.dimension):
            # Linear combination with challenge
            val = (n1.elements[i] + challenge * n2.elements[i]) % (2 * self.bound + 1)
            if val > self.bound:
                val -= 2 * self.bound + 1
            combined.append(val)
        
        return NoiseVector(n1.dimension, combined, self.bound)


class FoldingGenerator:
    """ProtoStar folding generator subagent."""
    
    def __init__(self):
        """Initialize folding generator."""
        self.noise_managers: Dict[str, NoiseManager] = {}
        self.accumulators: Dict[str, Accumulator] = {}
        self.folding_round = 0
    
    def generate(self, ir_nodes: List[Dict[str, Any]], 
                config: Dict[str, Any]) -> FoldingResult:
        """Generate folding circuits from IR nodes.
        
        Args:
            ir_nodes: List of IR nodes to process
            config: Folding configuration
            
        Returns:
            FoldingResult with generated circuits
        """
        try:
            # Extract folding parameters
            noise_dim = config.get("noise_dimension", 16)
            noise_bound = config.get("noise_bound", 100)
            error_bound = config.get("error_bound", 1000)
            
            circuits = []
            errors = []
            
            # Process each IR node
            for node in ir_nodes:
                if node.get("type") == "folding":
                    circuit = self._process_folding_node(
                        node, noise_dim, noise_bound, error_bound
                    )
                    if circuit:
                        circuits.append(circuit)
                    else:
                        errors.append(f"Failed to process node: {node.get('id')}")
            
            # Generate accumulator if needed
            accumulator = None
            if config.get("create_accumulator", False):
                accumulator = self._create_accumulator(
                    circuits, error_bound, noise_bound
                )
            
            # Generate commitments
            commitments = self._generate_commitments(circuits)
            
            return FoldingResult(
                success=len(errors) == 0,
                circuits=circuits,
                accumulator=accumulator,
                commitments=commitments,
                errors=errors,
                metadata={
                    "folding_round": self.folding_round,
                    "noise_dimension": noise_dim,
                    "total_circuits": len(circuits)
                }
            )
            
        except Exception as e:
            return FoldingResult(
                success=False,
                circuits=[],
                errors=[f"Generation failed: {str(e)}"]
            )
    
    def _process_folding_node(self, node: Dict[str, Any], 
                             noise_dim: int, noise_bound: int,
                             error_bound: int) -> Optional[FoldingCircuit]:
        """Process a single folding IR node.
        
        Args:
            node: IR node to process
            noise_dim: Noise vector dimension
            noise_bound: Noise element bound
            error_bound: Error term bound
            
        Returns:
            Generated folding circuit or None
        """
        op_type = node.get("operation", "").upper()
        
        if op_type not in [op.value for op in FoldingOp]:
            return None
        
        operation = FoldingOp(op_type)
        
        # Get or create noise manager
        instance_id = node.get("instance_id", "default")
        if instance_id not in self.noise_managers:
            self.noise_managers[instance_id] = NoiseManager(noise_dim, noise_bound)
        
        noise_mgr = self.noise_managers[instance_id]
        
        # Generate circuit based on operation
        if operation == FoldingOp.FOLD:
            return self._generate_fold_circuit(node, noise_mgr)
        elif operation == FoldingOp.UNFOLD:
            return self._generate_unfold_circuit(node, noise_mgr)
        elif operation == FoldingOp.COMMIT:
            return self._generate_commit_circuit(node)
        elif operation == FoldingOp.ACCUMULATE:
            return self._generate_accumulate_circuit(node, noise_mgr, error_bound)
        elif operation == FoldingOp.VERIFY:
            return self._generate_verify_circuit(node)
        
        return None
    
    def _generate_fold_circuit(self, node: Dict[str, Any], 
                              noise_mgr: NoiseManager) -> FoldingCircuit:
        """Generate FOLD operation circuit.
        
        Args:
            node: IR node for fold operation
            noise_mgr: Noise manager instance
            
        Returns:
            Folding circuit for FOLD
        """
        inputs = node.get("inputs", ["w1", "w2", "x1", "x2"])
        outputs = node.get("outputs", ["w_folded", "x_folded", "e_folded"])
        
        # Generate noise for this folding
        seed = node.get("seed", self.folding_round)
        noise = noise_mgr.generate_noise(seed)
        
        # Generate folding constraints
        constraints = [
            f"w_folded = w1 + r * w2",  # Witness folding
            f"x_folded = x1 + r * x2",  # Statement folding
            f"e_folded = e1 + r * e2 + r * (1 - r) * T",  # Error folding
        ]
        
        # Add noise constraints
        for i in range(min(4, noise.dimension)):
            constraints.append(f"n{i} = {noise.elements[i]}")
        
        self.folding_round += 1
        
        return FoldingCircuit(
            operation=FoldingOp.FOLD,
            inputs=inputs,
            outputs=outputs,
            constraints=constraints,
            noise_params={
                "dimension": noise.dimension,
                "bound": noise.bound,
                "seed": seed
            }
        )
    
    def _generate_unfold_circuit(self, node: Dict[str, Any],
                                noise_mgr: NoiseManager) -> FoldingCircuit:
        """Generate UNFOLD operation circuit.
        
        Args:
            node: IR node for unfold operation
            noise_mgr: Noise manager instance
            
        Returns:
            Folding circuit for UNFOLD
        """
        inputs = node.get("inputs", ["w_folded", "x_folded", "e_folded"])
        outputs = node.get("outputs", ["w", "x", "valid"])
        
        constraints = [
            f"valid = (e_folded < error_threshold)",
            f"w = extract_witness(w_folded)",
            f"x = extract_statement(x_folded)"
        ]
        
        return FoldingCircuit(
            operation=FoldingOp.UNFOLD,
            inputs=inputs,
            outputs=outputs,
            constraints=constraints,
            noise_params={
                "dimension": noise_mgr.dimension,
                "bound": noise_mgr.bound
            }
        )
    
    def _generate_commit_circuit(self, node: Dict[str, Any]) -> FoldingCircuit:
        """Generate COMMIT operation circuit.
        
        Args:
            node: IR node for commit operation
            
        Returns:
            Folding circuit for COMMIT
        """
        inputs = node.get("inputs", ["instance", "randomness"])
        outputs = node.get("outputs", ["commitment"])
        
        constraints = [
            f"commitment = hash(instance || randomness)",
            f"verify_commitment_binding(commitment, instance)"
        ]
        
        return FoldingCircuit(
            operation=FoldingOp.COMMIT,
            inputs=inputs,
            outputs=outputs,
            constraints=constraints,
            noise_params={}
        )
    
    def _generate_accumulate_circuit(self, node: Dict[str, Any],
                                   noise_mgr: NoiseManager,
                                   error_bound: int) -> FoldingCircuit:
        """Generate ACCUMULATE operation circuit.
        
        Args:
            node: IR node for accumulate operation
            noise_mgr: Noise manager instance
            error_bound: Maximum error bound
            
        Returns:
            Folding circuit for ACCUMULATE
        """
        inputs = node.get("inputs", ["acc", "instance"])
        outputs = node.get("outputs", ["acc_new", "proof"])
        
        # Generate fresh noise
        noise = noise_mgr.generate_noise(self.folding_round)
        
        constraints = [
            f"acc_new.error = acc.error + instance.error",
            f"acc_new.error < {error_bound}",
            f"acc_new.round = acc.round + 1",
            f"proof = generate_accumulation_proof(acc, instance)"
        ]
        
        # Add noise accumulation
        for i in range(min(4, noise.dimension)):
            constraints.append(
                f"acc_new.noise[{i}] = acc.noise[{i}] + instance.noise[{i}]"
            )
        
        return FoldingCircuit(
            operation=FoldingOp.ACCUMULATE,
            inputs=inputs,
            outputs=outputs,
            constraints=constraints,
            noise_params={
                "dimension": noise.dimension,
                "bound": noise.bound,
                "error_bound": error_bound
            }
        )
    
    def _generate_verify_circuit(self, node: Dict[str, Any]) -> FoldingCircuit:
        """Generate VERIFY operation circuit.
        
        Args:
            node: IR node for verify operation
            
        Returns:
            Folding circuit for VERIFY
        """
        inputs = node.get("inputs", ["instance", "proof", "commitment"])
        outputs = node.get("outputs", ["valid"])
        
        constraints = [
            f"valid = verify_folding_proof(instance, proof)",
            f"valid = valid AND verify_commitment(instance, commitment)",
            f"valid = valid AND check_error_bounds(instance)"
        ]
        
        return FoldingCircuit(
            operation=FoldingOp.VERIFY,
            inputs=inputs,
            outputs=outputs,
            constraints=constraints,
            noise_params={}
        )
    
    def _create_accumulator(self, circuits: List[FoldingCircuit],
                           error_bound: int, noise_bound: int) -> Accumulator:
        """Create accumulator from folding circuits.
        
        Args:
            circuits: List of folding circuits
            error_bound: Maximum error bound
            noise_bound: Maximum noise bound
            
        Returns:
            Created accumulator
        """
        instances = []
        commitments = []
        
        for circuit in circuits:
            if circuit.operation in [FoldingOp.FOLD, FoldingOp.ACCUMULATE]:
                # Create folded instance
                instance = FoldedInstance(
                    witness=[0] * 10,  # Placeholder
                    statement=[0] * 5,  # Placeholder
                    error=[0] * 3,  # Placeholder
                    noise=NoiseVector(
                        circuit.noise_params.get("dimension", 16),
                        [0] * circuit.noise_params.get("dimension", 16),
                        circuit.noise_params.get("bound", noise_bound)
                    ),
                    round=self.folding_round
                )
                instances.append(instance)
            
            if circuit.operation == FoldingOp.COMMIT:
                # Generate commitment
                commitment = self._hash_data(circuit.constraints)
                commitments.append(commitment)
        
        return Accumulator(
            instances=instances,
            error_bound=error_bound,
            noise_bound=noise_bound,
            current_round=self.folding_round,
            commitments=commitments
        )
    
    def _generate_commitments(self, circuits: List[FoldingCircuit]) -> Dict[str, str]:
        """Generate commitments for folding circuits.
        
        Args:
            circuits: List of folding circuits
            
        Returns:
            Dictionary of commitments
        """
        commitments = {}
        
        for i, circuit in enumerate(circuits):
            if circuit.operation == FoldingOp.COMMIT:
                key = f"commit_{i}"
                value = self._hash_data(
                    circuit.inputs + circuit.outputs + circuit.constraints
                )
                commitments[key] = value
            elif circuit.operation == FoldingOp.FOLD:
                # Commit to folded instance
                key = f"fold_{i}"
                value = self._hash_data([
                    str(circuit.noise_params),
                    str(circuit.constraints)
                ])
                commitments[key] = value
        
        return commitments
    
    def _hash_data(self, data: List[str]) -> str:
        """Hash data for commitment.
        
        Args:
            data: Data to hash
            
        Returns:
            Hex hash string
        """
        hasher = hashlib.sha256()
        for item in data:
            hasher.update(item.encode('utf-8'))
        return hasher.hexdigest()[:16]  # Truncate for readability


# High-degree gate decomposition utilities
class GateDecomposer:
    """Decomposes high-degree gates for folding."""
    
    @staticmethod
    def decompose_multiplication(inputs: List[str], 
                               degree: int) -> List[Tuple[str, str, str]]:
        """Decompose high-degree multiplication.
        
        Args:
            inputs: Input variables
            degree: Multiplication degree
            
        Returns:
            List of binary multiplication gates
        """
        if degree <= 2:
            return [(inputs[0], inputs[1], "mul_out")]
        
        gates = []
        intermediates = []
        
        # Binary tree decomposition
        current = inputs[:degree]
        level = 0
        
        while len(current) > 1:
            next_level = []
            for i in range(0, len(current), 2):
                if i + 1 < len(current):
                    out = f"t_{level}_{i//2}"
                    gates.append((current[i], current[i+1], out))
                    next_level.append(out)
                else:
                    next_level.append(current[i])
            current = next_level
            level += 1
        
        return gates
    
    @staticmethod
    def decompose_polynomial(coeffs: List[int], var: str,
                            max_degree: int = 2) -> List[str]:
        """Decompose polynomial evaluation.
        
        Args:
            coeffs: Polynomial coefficients
            var: Variable name
            max_degree: Maximum gate degree
            
        Returns:
            List of constraints
        """
        constraints = []
        
        # Horner's method decomposition
        if len(coeffs) <= max_degree + 1:
            # Direct evaluation
            terms = []
            for i, coeff in enumerate(coeffs):
                if i == 0:
                    terms.append(str(coeff))
                else:
                    terms.append(f"{coeff} * {var}^{i}")
            constraints.append(f"poly = {' + '.join(terms)}")
        else:
            # Recursive decomposition
            result = f"c{len(coeffs)-1}"
            for i in range(len(coeffs)-2, -1, -1):
                prev = result
                result = f"t{i}"
                constraints.append(f"{result} = {coeffs[i]} + {var} * {prev}")
            constraints.append(f"poly = {result}")
        
        return constraints


# Export main class
__all__ = ['FoldingGenerator', 'FoldingResult', 'FoldingOp', 
           'NoiseVector', 'FoldedInstance', 'Accumulator']