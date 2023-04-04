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

use crate::algorithms::{
    constraint::*,
    ac3
};

pub fn backtracking_search<T>(csp: &CSP<T>) -> Option<CSP<T>>
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut assignment = csp.clone();
    return backtrack(csp, &mut assignment);
}

fn backtrack<T>(csp: &CSP<T>, assignment: &mut CSP<T>) -> Option<CSP<T>>
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    if assignment_complete(csp) {
        return Some(csp.clone());
    }

    let var = select_unassigned_variable(csp, assignment);

    for value in assignment.get_variable(&var).get_domain() {
        let current_domain = assignment.get_variable(&var).get_domain().clone();
        assignment.set_domain(&var, vec![value.clone()]);
        let inference = ac3::ac3(assignment);
        if inference.is_some() {
            let mut new_assignment = inference.as_ref().unwrap().clone();
            let result = backtrack(&inference.unwrap(), &mut new_assignment);

            if result.is_some() {
                return result;
            }
        }
        let mut updated_domain = current_domain;
        if let Some(index) = updated_domain.iter().position(|v| *v == value) {
            updated_domain.remove(index);
        }
        assignment.set_domain(&var, updated_domain);
    }
   
    return None;
}

fn assignment_complete<T>(csp: &CSP<T>) -> bool
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut is_assignment_complete: bool = true;
    
    for var in csp.get_variables() {
        if var.1.get_domain().len() != 1 {
            is_assignment_complete = false;
            break;
        }
    }

    return is_assignment_complete;
}

fn select_unassigned_variable<'a, T>(csp: &'a CSP<T>, assignment: &'a CSP<T>) -> String
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut ret_val: String = String::new();

    for var in assignment.get_variables() {
        if var.1.get_domain().len() > 1 {
            ret_val = var.0.to_string();
        }
    }

    return ret_val;
}

