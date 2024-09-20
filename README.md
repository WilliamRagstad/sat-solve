# SAT-Solve

A simple boolean SAT solver written in Rust.

```pl
> x2 OR x4 AND x1 OR x2 AND -x2
Formula: (x2 OR x4) AND (x1 OR x2) AND (-x2)
Satisfiable: x1 = T, x2 = F, x4 = T
```

```pl
> x2 OR x4 AND x1 OR x2
Formula: (x2 OR x4) AND (x1 OR x2)
Satisfiable (2 solutions):
  x1 = F, x2 = T, x4 = F
  x1 = T, x2 = F, x4 = T
```
