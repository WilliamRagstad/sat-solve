# SAT-Solve

[![Build](https://github.com/WilliamRagstad/sat-solve/actions/workflows/rust.yml/badge.svg)](https://github.com/WilliamRagstad/sat-solve/actions/workflows/rust.yml)

A *toy* Boolean SAT *(B-SAT)* solver written in Rust.

> [!NOTE]
>
> A SAT solver takes a Boolean expression and finds out if the variables can be replaced by true or false so that the formula evaluates to true.
> There are no known algorithms that can efficiently and correctly solve all possible input instances.
> It is unknown how long it will take to find a solution, but many problems will be solved in a short time.

## Solvers

There are multiple experimental implementations of SAT solvers in this project that uses different algorithms and methods:

- **DFS**: Depth-first search brute-force algorithm &nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(not recommended)*
- **DPPL**: Unit propagation algorithm (DPLL) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(under development)*
- **CDCL**: Conflict-Driven Clause Learning algorithm &nbsp;*(planned)*
- **WalkSAT**: WalkSAT algorithm &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(not planned yet)*
- **GSAT**: GSAT algorithm &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(not planned yet)*
- **Chaff**: Chaff algorithm &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(not planned yet)*
- **MiniSAT**: MiniSAT algorithm &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(not planned yet)*
- **Z3**: The algorithm used in the Z3 solver &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; *(not planned yet)*

## Usage

The SAT solver is built as an interactive shell parsing formulas using the following syntax:

- **Variables**: `x1`, `x2`, `x3`, ...
- **Negation**: `-x1`, `!x2`
- **Conjunction**: `x1 AND x2`, `x1 & x2`
- **Disjunction**: `x1 OR x2`, `x1 | x2`
- **Grouping**: `(x1 OR x2) AND x3`, `x1 OR (x2 AND x3)`

The shell can be started with `cargo run` and exited with the `exit` command.

### Examples

```pl
Welcome to the SAT Solver!

> (x2 or x4) and (x1 or x2) and (-x2)
Satisfiable: x1 = T, x2 = F, x4 = T

> (-x1 or x1) and (x2 or -x2)
Satisfiable (4 solutions):
  x1 = F, x2 = F
  x1 = F, x2 = T
  x1 = T, x2 = F
  x1 = T, x2 = T

> math
OK

> (¬x₁ ∨ x₁) ∧ (x₂ ∨ ¬x₂)
Satisfiable (4 solutions):
  x₁ = ⊥, x₂ = ⊥
  x₁ = ⊥, x₂ = ⊤
  x₁ = ⊤, x₂ = ⊥
  x₁ = ⊤, x₂ = ⊤

> prog
OK

> -x1 or x1 and x2 or -x2
  (!x1 | x1) & (x2 | !x2)
Satisfiable (4 solutions):
  x1 = 0, x2 = 0
  x1 = 0, x2 = 1
  x1 = 1, x2 = 0
  x1 = 1, x2 = 1

> exit
```

## References

- [SAT Competition](http://www.satcompetition.org/)
- [SATLIB](http://www.cs.ubc.ca/~hoos/SATLIB/)
- Wikipedia
  - [Boolean Satisfiability Problem](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem)
  - [SAT Solver](https://en.wikipedia.org/wiki/SAT_solver)
  - [DPLL Algorithm](https://en.wikipedia.org/wiki/DPLL_algorithm)
  - [CDCL Algorithm](https://en.wikipedia.org/wiki/Conflict-driven_clause_learning)
  - [WalkSAT Algorithm](https://en.wikipedia.org/wiki/WalkSAT)
  - [GSAT Algorithm](https://en.wikipedia.org/wiki/GSAT)
  - [Chaff Algorithm](https://en.wikipedia.org/wiki/Chaff_(SAT_solver))
  - [MiniSAT Algorithm](https://en.wikipedia.org/wiki/MiniSAT)
  - [Z3 Theorem Prover](https://en.wikipedia.org/wiki/Z3_Theorem_Prover)
  - Related Topics
	- [Satisfiability Modulo Theories](https://en.wikipedia.org/wiki/Satisfiability_modulo_theories)
	- [Quantified Boolean Formula](https://en.wikipedia.org/wiki/Quantified_boolean_formula)
	- [P vs NP Problem](https://en.wikipedia.org/wiki/P_versus_NP_problem)
	- [Boolean satisfiability problem](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem)
	- [Conjunctive Normal Form](https://en.wikipedia.org/wiki/Conjunctive_normal_form)
	- [Unit propagation](https://en.wikipedia.org/wiki/Unit_propagation)
- Talks
  - [Constraint Satisfaction: Introduction](https://www.youtube.com/watch?v=_e64FiDWvqs)
  - [A Peek Inside SAT Solvers](https://www.youtube.com/watch?v=d76e4hV1iJY)
  - [Z3 Explained - Satisfiability Modulo Theories & SMT Solvers](https://www.youtube.com/watch?v=EacYNe7moSs)
  - [Analyzing Programs with Z3](https://www.youtube.com/watch?v=ruNFcH-KibY)
  - [The Boolean Satisfiability Problem and Satisfiability Modulo Theories (SAT / SMT)](https://www.youtube.com/watch?v=zeQASMpuSGE)
  - [SAT solvers best explanation](https://www.youtube.com/playlist?list=PLqinEaadXCHYW_1Z3W05rNx0skQIxrmQB)
  - [SMT in reverse engineering, for dummies](https://www.youtube.com/watch?v=b92CW-NZ3l0)
- Papers
  - [Chaff: Engineering an Efficient SAT Solver](https://www.princeton.edu/~chaff/publication/DAC2001v56.pdf)
  - [Conflict-Driven Clause Learning SAT Solver](https://www.cs.princeton.edu/~zkincaid/courses/fall18/readings/SATHandbook-CDCL.pdf)
  - [CDCL SAT Solvers & SAT-Based Problem Solving](https://cse.usf.edu/~haozheng/teach/cda5416/slides/intro-sat.pdf)
  - [Lecture Notes on SAT Solvers & DPLL](https://www.cs.cmu.edu/~15414/f17/lectures/10-dpll.pdf)
  - [Lecture Notes on Solving SAT with DPLL](https://www.cs.cmu.edu/~15414/s22/lectures/14-sat-dpll.pdf)
  - [Learning to Select Branching Rules in the DPLL Procedure for Satisfiability](https://www.cs.ubc.ca/~hutter/earg/papers07/lagoudakis01learning.pdf)
