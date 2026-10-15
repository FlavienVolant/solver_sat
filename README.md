# solver_sat

Blazingly fast O(2^n) sat solver !!!!!!!!!!!

This project is a learning project. Its goal is to explore SAT solving, Boolean logic and Rust.

The solver works with formulas in Conjunctive Normal Form (CNF). It uses unit propagation to simplify formulas before performing an exhaustive search.

## Features

- Represent Boolean formulas in CNF.
- Create formulas with a simple DSL using Rust macros.
- Evaluate formulas with a variable assignment.
- Reduce formulas using known variable assignments.
- Find unit clauses and propagate their values.
- Solve SAT problems using exhaustive search.

## Getting started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo

### Run the project

Clone the repository and move into the project directory:

```bash
git clone <repository-url>
cd solver_sat
```

Run the project:

```bash
cargo run
```

Run the tests:

```bash
cargo test
```

## Usage

### Creating a formula

The solver provides macros to create literals, clauses and formulas.

For example:

```rust
let formula = formule!(
    1 | 2,
    -1 | 3,
    -2 | -3
);
```

This creates the following formula:

\[
(A \lor B) \land (\neg A \lor C) \land (\neg B \lor \neg C)
\]

Each integer represents a Boolean variable.

- A positive integer represents a positive literal.
- A negative integer represents a negated literal.
- The `|` operator separates literals in a clause.
- Commas separate clauses.

For example, `1` represents \(A\), while `-1` represents \(\neg A\).

The variable numbers have no predefined meaning. You can use them to represent any Boolean variables.

### Solving a formula

Use `solve()` to find a satisfying assignment:

```rust
let formula = formule!(
    1 | 2,
    -1 | 3,
    -2 | -3
);

match formula.solve() {
    Some(solution) => {
        println!("SAT");
        println!("{:?}", solution);
    }
    None => {
        println!("UNSAT");
    }
}
```

The solver returns:

- `Some(solution)` if the formula is satisfiable.
- `None` if the formula is unsatisfiable.

A solution is a mapping from variable numbers to Boolean values.

For example:

```text
{
    1: true,
    2: false,
    3: true
}
```

This means that variable `1` is true, variable `2` is false, and variable `3` is true.

### Evaluating a formula

You can evaluate a formula with a variable assignment:

```rust
let formula = formule!(
    1 | 2,
    -1 | 3
);

let mut assignment = HashMap::new();
assignment.insert(1, true);
assignment.insert(2, false);
assignment.insert(3, true);

assert!(formula.evaluate(&assignment));
```

The result is `true` because both clauses are satisfied.

## Why CNF?

The solver only supports formulas in Conjunctive Normal Form (CNF).

CNF is a way to represent Boolean formulas using two operations:

- OR (`∨`) inside clauses.
- AND (`∧`) between clauses.

For example:

\[
(A \lor B) \land (\neg A \lor C)
\]

is in CNF.

CNF is widely used in SAT solving because it provides a simple and structured representation of Boolean constraints.

Restricting the solver to CNF also keeps the implementation small and makes it easier to explore SAT-solving algorithms.

Formulas that are not in CNF must be converted before they can be used by this solver.

## How does it work?

The solver uses the following steps:

1. **Unit propagation**: find clauses containing a single literal and assign the corresponding variable.
2. **Formula reduction**: remove satisfied clauses and remove false literals from the remaining clauses.
3. **Exhaustive search**: enumerate the possible assignments for the remaining variables.
4. **Evaluation**: return a satisfying assignment if one is found.

If no assignment satisfies the formula, the solver returns `None`.

### Complexity

The current solving algorithm uses exhaustive search.

For a formula with `n` unassigned variables, it may need to explore up to:

\[
2^n
\]

possible assignments.

Unit propagation can reduce the number of variables that need to be explored, but the worst-case complexity remains exponential.
