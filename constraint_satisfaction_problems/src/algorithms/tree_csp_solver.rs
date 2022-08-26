/// function Tree-CSP-Solver(csp) returns a solution or failure
///     inputs: csp, a CSP with components X, D, C
///
///     n <- number of variables in X
///     assignment <- an empty assignment
///     root <- any variable in X
///     X <- TopologicalSort(X, root)
///     for j = n down to 2 do
///         Make-Arc-Consistent(Parent(X_j),X_j)
///         if it cannot be made consistent then return failure
///     for i = 1 to n do
///         assignment[X_i] <- any consistent value from D_i
///         if there is no consisten value then return failure
///     return assignment
///

