# TauFold-zkVM

A formally verified microlibrary of zkVM components built using the [Tau Language's](https://github.com/IDNI/tau-lang) reactive synthesis engine.

This project is not a complete, runnable zkVM. It is a collection of provably correct hardware and logic components—**"sacred bricks"**—that can be used to construct high-assurance zero-knowledge virtual machines.

## The Tau Paradigm: Reactive Synthesis

Tau is not a conventional programming language; it is a tool for **reactive synthesis**. This concept can be framed as a game between a controller and its environment:

> *For every move the environment (inputs) can make, can I (the controller) always produce a move (outputs) **now** so the system's rules are never broken—forever?*

Tau's satisfiability check directly encodes this game. When a Tau specification is run, the runtime repeatedly:

1.  **Requests input values** for the current time-step.
2.  **Invokes an internal solver** to synthesize concrete output values that satisfy the specification's constraints.
3.  **Emits those outputs** and advances to the next time-step.

This makes a Tau spec an **executable strategy** that is guaranteed to be correct for any possible sequence of inputs.

## Can a whole zkVM fit inside Tau?

-   **Theoretically, yes.** One could describe an entire ISA, micro-architectural state, and memory system as a set of Tau constraints. Satisfiability would guarantee a valid state transition for any program.
-   **Practically, this is challenging.** The state-space explosion and the complexity of bit-vector arithmetic make this approach intractable for all but the simplest machines.

### A Pragmatic Approach

This project takes a more practical, hierarchical approach:

1.  **Specify & Verify Micro-components:** We use Tau for what it excels at: formally verifying small, critical hardware components like adders, multiplexers, registers, and program counters.
2.  **Export Proofs:** These verified "sacred bricks" can then be used with confidence by higher-level tools or conventional code to construct an optimized zkVM circuit.
3.  **Ambient Monitors:** Tau can still be used to specify and monitor high-level properties of the larger system, such as memory isolation or protocol correctness.

## Repository Structure

The project is organized around the verification workflow:

-   `microlibrary/`: Contains the Tau definitions for hardware components.
    -   `passed/`: **Sacred Bricks.** These components are formally verified and considered immutable.
    -   `pending/`: Components that are new, being debugged, or awaiting verification.
-   `proofs/`: Contains the equivalence proofs for the components in the microlibrary.
    -   `passed/`: Proofs that have successfully run, verifying a component.
    -   `pending/`: Proofs that are under development.

## Verification Workflow

Our core verification strategy is the **UNSAT Discrepancy Proof**. To prove a structural implementation (`*_s.tau`) is equivalent to its mathematical specification (`*_b.tau`), we create a proof that asserts there exists a set of inputs for which their outputs differ. The Tau solver must prove this assertion is **unsatisfiable**.

### Running a Proof

To run a proof, you need a local build of the [Tau Language](https://github.com/IDNI/tau-lang). The canonical way to execute a proof is to pipe it directly into the `tau` executable:

```bash
# Example: Verifying the 2-bit program counter
cat proofs/pending/prove_pc2_equivalence.tau | ./path/to/tau-lang/build-Release/tau
```

A successful proof will output `%1: T`, indicating the solver proved the discrepancy was unsatisfiable.

### Best Practices

1.  **Contracts First, Then Structure:** Always start with a clear mathematical contract (`_b.tau`) before building the gate-level implementation (`_s.tau`).
2.  **Slice and Decompose:** Prove components at the smallest possible scale (1-bit, one mode, one cycle) and compose them hierarchically.
3.  **Guard All Inputs:** Use helper predicates like `bit(x)` to constrain the domain of variables, preventing the solver from exploring impossible states.

## Project Status

This project is an active research and development effort. We are focused on building out the foundational layers of a zkVM, including:

-   Arithmetic Logic Unit (ALU) components
-   Register files and memory blocks
-   Program counters and instruction decoders

Our primary goal is to create a robust, open-source library of formally verified components, establishing a foundation of trust for future zkVM development.