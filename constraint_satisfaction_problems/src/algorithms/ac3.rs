/// function AC-3(csp) returns false if an inconsistency is found and true otherwise
///     queue <- a queue of arcs, initially all the arcs in csp
///
///     while queue is not empty do
///         (X_i, X_j) <- Pop(queue)
///         if Revise(csp, X_i, X_j) then
///             if size of D_i = 0 then return false
///             for each X_k in X_i.neighbors - {X_j} do
///                 add (X_k, X_i) to queue
///     return true
///
/// function Revise(csp, X_i, X_j) returns true iff we revise the domain of X_i
///     revised <- false
///     for each x in D_i do
///         if no value y in D_j allows (x,y) to satisfy the constraint between X_i and X_j then
///             delete x from D_i
///             revised <- true
///     return revised
///

use crate::algorithms::constraint::*;

pub fn ac3<T>(csp: &CSP<T>) -> Option<CSP<T>>
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut local_csp = csp.clone();
    let mut queue = csp.get_arcs();
    let mut new_arc: (String, String);
    while !queue.is_empty() {
        let (x_i, x_j) = queue.pop_front().unwrap();
        let (x_j, mut x_i) = (local_csp.get_variable(&x_j).clone(), local_csp.get_variable_as_mut(&x_i));
        if revise(csp, &mut x_i, &x_j) {
            if x_i.get_domain().len() == 0 {
                return None;
            }
            for x_k in x_i.get_neighbors() {
                if csp.get_variable(&x_k) != &x_j {
                    new_arc = (x_k, x_i.get_name());
                    queue.push_back(new_arc);
                }
            }
        }
    }
    
    return Some(local_csp);
}

fn revise<T>(csp: &CSP<T>, x_i: &mut Variable<T>, x_j: &Variable<T>) -> bool 
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq + std::fmt::Debug
{
    let mut revised = false;
    let mut to_remove = Vec::new();
    for x in &x_i.get_domain() {
        let mut a_constraint_satisfied = false;
        for y in &x_j.get_domain() {
            if csp.satisfies_constraint(&x_i.get_name(), &x_j.get_name(), x, y) {
                a_constraint_satisfied = true;
            }
        }
        if !a_constraint_satisfied {
            to_remove.push(x.clone());
            revised = true;
        }
    }
    
    for x in to_remove {
        let mut index = 0;
        for d in &x_i.get_domain() {
            if d == &x {
                x_i.get_domain_as_mut().remove(index);
                break;
            }
            index += 1;
        }
    }

    return revised;
}
