///
/// function Breadth-First-Search(problem) returns a solution node or failure
///     node <- Node(problem.Initial)
///     if problem.Is_Goal(node.State) then return node
///     frontier <- a FIFO queue, with node as an element
///     reached <- {problem.Initial}
///
///     while not Is_Empty(frontier) do
///         s <- child.State
///         if problem.Is_Goal(s) then return child
///         if s is not in reached then
///             add s to reached
///             add child to frontier
///     return failure
///

use std::collections::{
    VecDeque,
    HashMap
};
use std::hash::Hash;

use crate::algorithms::{node::Node, problem::*};


pub fn breadth_first_search<P, S, A>(problem: &P) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone + Eq + Ord + Hash,
    A: Clone + Eq + Ord + Hash
{
    
    let mut node = problem.get_initial_node();
    if problem.is_goal(&node.state) {
        return Ok(node);
    }
    
    let mut frontier: VecDeque<Node<S, A>> = VecDeque::new();
    frontier.push_back(node.clone());
    let mut reached: HashMap<S, Node<S, A>> = HashMap::new();
    reached.insert(node.state.clone(), node);


    while !frontier.is_empty() {
        let n = frontier.pop_front().unwrap();
        
        for child in expand(problem, n) {
            if problem.is_goal(&child.state) {
                return Ok(child);
            }
            if !reached.contains_key(&child.state) {
                reached.insert(child.state.clone(), child.clone());
                frontier.push_back(child);
            }
        }
    }

    Err(SearchError::Failure)
}


pub fn uniform_cost_search<P, S, A>(problem: &P) -> SearchResult<S, A> 
where
    P: Problem<S, A>,
    S: Clone + Eq + Ord + Hash,
    A: Clone + Eq + Ord + Hash
{
    breadth_first_search(problem)
}
