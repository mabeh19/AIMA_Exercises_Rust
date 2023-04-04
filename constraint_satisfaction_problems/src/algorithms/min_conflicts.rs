/// function Min-Conflicts(csp, max_steps) returns a solution or failure
///     inputs: csp, a constraint satisfaction problem
///     max_steps, the number of steps allowed before giving up
///
///     current <- an initial complete assignment for csp
///     for i = 1 to max_steps do
///         if current is a solution for csp then return current
///         var <- a randomly chosen conflicted variable from csp.Variables
///         value <- the value v for var that minimizes Conflicts(csp, var, v, current)
///         set var = value in current
///     return failure
///

use rand::prelude::*;
use crate::algorithms::constraint::*;

pub fn min_conflicts<T>(csp: &CSP<T>, max_steps: usize) -> Option<CSP<T>>
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut current = choose_random_solution(csp);
    let mut rng = rand::thread_rng();
    let n_vars = current.get_variables().len();

    for _ in 0..max_steps {
        
        if current.assignment_complete() {
            return Some(current);
        }
        
        let rand_var = current.get_variables().keys().nth(rng.gen_range(0..n_vars)).unwrap().clone();
        let val = var_minimizing_conflicts(csp, &rand_var, &current).unwrap();
        current.set_domain(&rand_var, vec![val]);
    }

    return None;
}

fn choose_random_solution<T>(csp: &CSP<T>) -> CSP<T>
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut solution = csp.clone();
    let mut rng = rand::thread_rng();

    for var in solution.get_variables().clone() {
        let domain = var.1.get_domain();
        let rand_value = domain[rng.gen_range(0..domain.len())].clone();
        solution.set_domain(&var.0, vec![rand_value]);
    }

    return solution;
}

fn var_minimizing_conflicts<T>(csp: &CSP<T>, rand_var: &str, current: &CSP<T>) -> Option<T>
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut curr_copy = current.clone();
    let mut min_num_conflicts = 0;
    let mut min_num_conflicts_val: Option<T> = None;

    for v in csp.get_variable(rand_var).get_domain() {
        curr_copy.get_variable_as_mut(rand_var).set_value(&v);
        let num_conflicts = curr_copy.get_num_conflicts(rand_var);

        if num_conflicts > min_num_conflicts {
            min_num_conflicts = num_conflicts;
            min_num_conflicts_val = Some(v);
        }
    }

    min_num_conflicts_val
}
