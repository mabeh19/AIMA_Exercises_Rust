///
/// function Simulated-Annealing(problem, schedule) returns a solution state
///     current <- problem.initial
///     for t = 1 to infty do
///         T <- schedule(t)
///         if T = 0 then return current
///         next <- a randomly selected successor of current
///         del_E <- Value(current) - Value(current)
///         if del_E > 0 then current <- next
///         else current <- next only with probability e^(-del_E/T)
///

use rand::prelude::*;

use crate::algorithms::{
    node::Node,
    problem::*
};


pub fn simulated_annealing<P, S, A>(problem: &P) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    let mut rng = rand::thread_rng();
    let mut current = problem.get_initial_node();

    for t in 1..usize::MAX {
        let T = schedule(t);
        if T == 0 {
            return current;
        }
        let next = random_successor(&current);
        let del_E = Value(current) - Value(current);
        if del_E > 0 || rng.gen_ratio(-del_E, T) {
            current = next;
        }
    }

    return Err(SearchError::Failure);
}
