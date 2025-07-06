# TauFold ZKVM Microlibrary

## 1. Overview

This directory contains a set of formally verified, primitive hardware components specified in the Tau language. Each component serves as a fundamental building block for constructing more complex arithmetic and logical circuits within the TauFold ZKVM.

The primary goal of this microlibrary is to provide a foundation of **sound, budget-conscious, and modular** proofs. Every component herein has been rigorously verified to be a tautology, meaning it is correct for all possible inputs.

## 2. Core Principles

- **Formal Verification**: We do not merely test for specific cases. Each component is proven correct using the `unsat(specification ∧ ¬guarantee)` pattern. This demonstrates that no counterexample exists, providing the strongest possible guarantee of correctness.
- **Modularity and Composition**: Components are designed to be small, independent, and composable. Complex systems are built by chaining these verified primitives together, ensuring that the complexity of any single proof remains within the Tau solver's clause budget.
- **REPL-First Workflow**: All specifications are defined and verified using the Tau REPL via the `tau-cli.py` automation tool. This provides access to the full feature set of the Tau language, including the `define` keyword, which is essential for building a modular library.
- **No Disjunctions in Assembly**: When composing primitives into larger circuits, only conjunction (`&&`) is used to chain components. This enforces a strict, linear data flow that is easier to reason about and verify.

## 3. Verified Components

This microlibrary currently contains the following verified components:

- `ha.tau`: **Half Adder**
  - **Inputs**: `a`, `b`
  - **Outputs**: `s` (sum), `c` (carry)
  - **Logic**: `s = a XOR b`, `c = a AND b`

- `fa.tau`: **Full Adder**
  - **Inputs**: `a`, `b`, `cin` (carry-in)
  - **Outputs**: `s` (sum), `cout` (carry-out)
  - **Logic**: `s = a XOR b XOR cin`, `cout = majority(a, b, cin)`

- `cmp.tau`: **1-Bit Comparator**
  - **Inputs**: `a`, `b`
  - **Outputs**: `lt` (less than), `eq` (equal), `gt` (greater than)
  - **Logic**: `lt = !a AND b`, `eq = !(a XOR b)`, `gt = a AND !b`

- `cond_sub.tau`: **Conditional Subtractor**
  - **Inputs**: `a`, `b`, `cond` (condition)
  - **Outputs**: `sub` (result)
  - **Logic**: `sub = cond ? (a - b) : a`

## 4. Usage

Components can be defined, tested, and verified using the `tau-cli.py` tool.

### Example: Verifying the Half Adder

```bash
# Navigate to the automation tools directory
cd ../tools/tau_repl_automation

# The tool has a built-in command to verify the 'ha' spec
./tau_cli.py verify ha
```

### Example: Verifying a Custom Component (Comparator)

Custom components are verified using a dedicated script that runs in the REPL.

1.  **Define the component**:
    ```bash
    ./tau_cli.py define cmp "a,b,lt,eq,gt" "lt=(a'&b) && eq=(a+b)' && gt=(a&b')" --save ../../microlibrary/cmp.tau
    ```

2.  **Create a verification script** (`verify_cmp.tau`):
    ```tau
    # Define the function in the REPL session
    define cmp(a,b,lt,eq,gt) = (lt=(a'&b) & eq=(a+b)' & gt=(a&b'))

    # Prove its correctness by showing no counterexample exists
    unsat exists a,b,lt,eq,gt. (cmp(a,b,lt,eq,gt) & !(lt=(a'&b) & eq=(a+b)' & gt=(a&b')))

    quit
    ```

3.  **Run the verification script**:
    ```bash
    ./tau_cli.py run ../../microlibrary/verify_cmp.tau
    ```
