///
/// function Hill-Climbing(problem) returns a state that is a local maximum
///     current <- problem.Initial
///     while true do
///         neighbor <- a highest-valued successor state of current
///         if Value(neighbor) <= Value(current) then return current
///         current <- neighbor
///

use crate::algorithms::{
    node::Node,
    problem::*
};

pub fn hill_climbing<P, S, A>(problem: &P) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    let mut current = problem.get_goal_node();

    loop {
        let neighbor = highest_valued_successor(problem, current.clone())?;
        if neighbor.path_cost <= current.path_cost {
            return Ok(current);
        }
        current = neighbor;
    }
}


fn highest_valued_successor<P, S, A>(problem: &P, node: Node<S, A>) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    let mut neighbors = expand(problem, node.clone());
    
    /* Sort descending */
    neighbors.reverse();
      
    let retval: Node<S, A> = neighbors.get(0).unwrap().clone();

    return Ok(retval);
}
