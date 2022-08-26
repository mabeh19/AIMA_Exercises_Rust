/// function Backtracking-Search(csp) returns a solution or failure
///     return Backtrack(csp, {})
///
/// function Backtrack(csp, assignment) returns a solution or failure
///     if assignment is complete then return assignment
///     var <- Select-Unassigned-Variable(csp, assignment)
///     for each value in Order-Domain-Values(csp, var, assignment) do
///         if value is consistent with assignment then
///             add {var = value} to assignment
///             inferences <- Inference(csp, var, assignment)
///             if inferences != failure then
///                 add inferences to csp
///                 result <- Backtrack(csp, assignment)
///                 if result != failure then return result
///                 remove inferences from csp
///             remove {var = value} from assignment
///     return failure
///


