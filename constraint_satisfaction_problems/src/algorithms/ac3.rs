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

pub fn ac3<T, C>(csp: &CSP<T, C>) -> bool 
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    let mut queue = csp.get_arcs();

    while !queue.is_empty() {
        let (x_i, x_j) = queue.pop().unwrap();
        if revise(csp, x_i, x_j) {
            if d_i.len() == 0 {
                return false
            }
            for x_k in x_i.get_neighbors() {
                if x_k != x_J {
                    queue.push((x_k, x_i));
                }
            }
        }
    }
    
    return true;
}

fn revise<T, C>(csp: &CSP<T, C>, x_i: &Variable<T, C>, x_j: &Variable<T, C>) -> bool 
where
    T: Clone + PartialEq + PartialOrd + std::hash::Hash + Eq,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    let mut revised = false;
    let mut to_remove = Vec::new();
    for x in &d_i {
        for y in &d_j {
            if csp.satisfies_constraint(x_i, x_j, y) {
                to_remove.push(x);
                revised = true;
            }
        }
    }
    return revised;
}
