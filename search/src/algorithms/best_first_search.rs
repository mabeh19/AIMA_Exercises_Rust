///
///
/// Best first search algorithm based on the following pseudo-code from AIMA
///
/// function Best-First-Search(problem, f) returns a solution node or failure
///     node <- Node(State=problem.INITIAL)
///     frontier <- a priority queue ordered by f, with node as an element
///     reached <- a lookup table, with one entry with key problem.INITIAL and value node
///
///     while not Is_Empty(frontier) do
///         node <- Pop(frontier)
///         if problem.Is_Goal(node.STATE) then return node
///         for each child in Expand(node.STATE) do
///             s <- child.STATE
///             if s is not in reached or child.PATH_COST < reached[s].PATH_COST then
///                 reached[s] <- child
///                 add child to frontier
///     return failure
///
/// function Expand(problem, node) yields nodes
///     s <- node.STATE
///     for each action in problem.ACTIONS(s) do
///         s' <- problem.RESULT(s,action)
///         cost <- node.PATH_COST + problem.ACTION_COST(s,actions,s')
///         yield Node(State=s', Parent=node, Action=action, Path_Cost=cost)
///

/* Std library */
use std::collections::HashMap;
use std::hash::Hash;

/* Internal crates */
use crate::algorithms::node::Node;
use crate::algorithms::problem::*;



pub fn best_first_search<P, S, A>(problem: &P) -> SearchResult<S, A> 
where
    P: Problem<S, A>,
    S: Clone + Eq + Ord + Hash,
    A: Clone + Eq + Ord + Hash
{
    let mut node = problem.get_initial_node();
    let mut frontier: Vec<Node<S, A>> = vec![node.clone()];
    let mut reached: HashMap<S, Node<S, A>> = HashMap::new(); 
    reached.insert(node.state.clone(), node.clone());
    
    while !frontier.is_empty() {
        frontier.sort();
        node = frontier.pop().unwrap();
        if problem.is_goal(&node.state) {
            return Ok(node);
        }

        for child in expand(problem, node) {
            let s = child.state.clone();
            if !reached.contains_key(&s) || child.path_cost < reached.get(&s).unwrap().path_cost {
                reached.insert(s, child.clone());
                frontier.push(child.clone());
            }
        }
    }

    Err(SearchError::Failure)
}

