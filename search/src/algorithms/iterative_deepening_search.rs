///
/// function Iterative_Deepening_Search(problem) returns a solution node or failure
///     for depth = 0 to infty do
///         result <- Depth_Limited_Search(problem, depth)
///         if result != cutoff then return result
///
/// function Depth_Limited_Search(problem, l) returns a node or failure or cutoff
///     frontier <- a LIFO queue (stack) with Node(problem.initial) as an element
///     result <- failure
///     while not Is_Empty(frontier) do
///         node <- Pop(frontier)
///         if problem.Is_Goal(node.state) then return node
///         if depth(node) > l then
///             result <- cutoff
///         else if not Is_Cycle(node) do
///             for each child in expand(problem, node) do
///                 add child to frontier
///     return result

use crate::algorithms::{
    problem::*,
    node::{Node, depth, is_cycle},
};

const MAX_LIMIT: usize = 1_000;

pub fn iterative_deepening_search<P, S, A>(problem: &P) -> SearchResult<S, A> 
where
    P: Problem<S, A>,
    S: Clone + Eq,
    A: Clone + Eq
{
    
    for depth in 0..MAX_LIMIT {
        match depth_limited_search(problem, depth) {
            Ok(result) => {
                return Ok(result);
            },
            _ => {}
        }
         
    }
    
    Err(SearchError::Failure)
}


fn depth_limited_search<P, S, A>(problem: &P, l: usize) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone + Eq,
    A: Clone + Eq
{
    let mut frontier: Vec<Node<S, A>> = vec![problem.get_initial_node()];
    let mut result: SearchResult<S, A> = Err(SearchError::Failure);    
    
    while !frontier.is_empty() {
        let node = frontier.pop().unwrap();
        if problem.is_goal(&node.state) {
            return Ok(node);
        }
        if depth(node.clone()) == l {
            result = Err(SearchError::CutOffReached);
        }
        else if !is_cycle(node.clone()) {
            for child in expand(problem, node.clone()) {
                frontier.push(child);
            }
        }
    }

    return result;
}
