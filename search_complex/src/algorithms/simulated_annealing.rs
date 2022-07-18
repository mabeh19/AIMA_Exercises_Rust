///
/// function Simulated-Annealing(problem, schedule) returns a solution state
///     current <- problem.initial
///     for t = 1 to infty do
///         T <- schedule(t)
///         if T = 0 then return current
///         next <- a randomly selected successor of current
///         del_E <- Value(current) - Value(next)
///         if del_E > 0 then current <- next
///         else current <- next only with probability e^(-del_E/T)
///

use std::fmt::{Debug, Display};

use rand::prelude::*;

use crate::algorithms::{
    node::Node,
    problem::*
};


pub fn simulated_annealing<P, S, A>(problem: &P) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{
    let mut rng = rand::thread_rng();
    let mut current = problem.get_initial_node();

    for t in 1..usize::MAX {
        let T = schedule(t);
        if T == 0. {
            return Ok(current);
        }
        let next = random_successor(problem, &current);
        let del_e: f64 = -value(problem, &current) + value(problem, &next);
        let boltzmann_dist = (del_e / T).exp();
        if del_e > 0. || rng.gen_bool(boltzmann_dist) {
            current = next;
        }
    }

    return Err(SearchError::Failure);
}


fn schedule(t: usize) -> f64 {
    if t % 1000 == 0 {
        return 0.
    } else {
        1. / t as f64
    }
}

fn random_successor<P, S, A>(problem: &P, node: &Node<S, A>) -> Node<S, A>
where
    P: Problem<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{
    let mut rng = rand::thread_rng();
    
    let candidates = expand(problem, node.clone());
    //println!("candidates: {}", candidates.len());
    let successor = candidates.get(rng.gen_range(0..candidates.len())).unwrap().clone();
    return successor;
}

fn value<P, S, A>(problem: &P, node: &Node<S, A>) -> f64
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    problem.get_heuristic_cost(&node.state)
}
