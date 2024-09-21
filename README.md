# SAT-Solve

A *toy* Boolean SAT *(B-SAT)* solver written in Rust.

> [!NOTE]
>
> A SAT solver takes a Boolean expression and finds out if the variables can be replaced by true or false so that the formula evaluates to true.
> There are no known algorithms that can efficiently and correctly solve all possible input instances.
> It is unknown how long it will take to find a solution, but many problems will be solved in a short time.

## Example

Start the interactive shell with `cargo run` and enter formulas to solve.

```pl
> x2 OR x4 AND x1 OR x2 AND -x2
Formula: (x2 OR x4) AND (x1 OR x2) AND (-x2)
Satisfiable: x1 = T, x2 = F, x4 = T

> x2 OR x4 AND x1 OR x2
Formula: (x2 OR x4) AND (x1 OR x2)
Satisfiable (2 solutions):
  x1 = F, x2 = T, x4 = F
  x1 = T, x2 = F, x4 = T
```
