///
/// function Recursive-Best-First-Search(problem) returns a solution or failure
///     solution,fvalue <- RBFS(problem, Node(problem.initial), infty)
/// return solution
///
///
/// function RBFS(problem, node, f_limit) returns a solution or failure, and a new f-cost limit
///     if problem.Is-Goal(node.state) then return node
///     successors <- List(Expand(node))
///     if successors is empty then return failure, infty
///     for each s in successors do     // update f with value from previous search
///         s,f <- max(s.Path-Cost + h(s), node.f)
///     while true do
///         best <- the node in successors with lowest f-value
///         if best.f > f_limit then return failure, best.f
///         alternative <- the second-lowest f-value among successors
///         result,best.f <- RBFS(problem, best, min(f_limit,alternative))
///         if result != then return result, best.f
///
///

use std::{
    fmt::Debug
};

use crate::algorithms::{
    problem::*,
    node::*,
};

pub fn recursive_best_first_search<P, S, A>(problem: &P, h: fn(&P, &Node<S, A>) -> u32) -> SearchResult<S, A>
where
    P: Problem<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{
    let (solution, fvalue) = rbfs(problem, h, problem.get_initial_node(), u32::MAX);
    return solution;
}

fn rbfs<P, S, A>(problem: &P, h: fn(&P, &Node<S, A>) -> u32, node: Node<S, A>, f_limit: u32) -> (SearchResult<S, A>, u32)
where
    P: Problem<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{
    if problem.is_goal(&node.state) {
        return (Ok(node), f_limit);
    }
    let mut successors = expand(problem, node.clone());
    successors.reverse();

    if successors.is_empty() {
        return (Err(SearchError::Failure), u32::MAX);
    }

    for mut s in &mut successors {
        s.f = (s.path_cost + h(problem, &s)).max(node.clone().f);
    }
    
    loop {
        //println!("{:#?}", successors);           
        let mut best = successors.pop().unwrap();
        if best.f >= f_limit {
            return (Err(SearchError::Failure), best.f);
        }
        
        let alternative = successors.pop().unwrap();
        let result: SearchResult<S, A>;
        (result, best.f) = rbfs(problem, h, best.clone(), f_limit.min(alternative.f));
        match result {
            Ok(_) => {
                return (result, best.f);
            },
            _ => {}
        }
    }
}


