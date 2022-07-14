///
/// function Bibf_Search(problem_F, f_F, problem_B, f_B) returns a solution node or failure
///     node_F <- Node(problem_F.initial)   // Node for a start gate
///     node_B <- Node(problem_B.initial)   // Node for a goal state
///     frontier_F <- a priority queue ordered by f_F, with node_F as an element
///     frontier_B <- a priority queue ordered by f_B, with node_B as an element
///     reached_F <- a lookup table, with one key node_F.State and value node_F
///     reached_B <- a lookup table, with one key node_B.State and value node_B
///     solution <- failure
///
///     while not Terminated(solution, frontier_F, frontier_B) do
///         if f_F(Top(frontier_F)) < f_B(Top(frontier_B)) then
///             solution <- Proceed(F, problem_F, frontier_F, reached_F, reached_B, solution)
///         else solution <- Proceed(B, problem_B, frontier_B, reached_B, reached_F, solution)
///     return solution
///
///
/// function Proceed(dir, problem, frontier, reached, reached_2, solution) returns a solution
///         // Expand node on frontier; check against the other frontier in reached_2.
///         // The variable "dir" is the direction: either F for forward or B for backward
///     node <- Pop(frontier)
///     for each child in Expand(problem, node) do
///         s <- child.State
///         if s not in reached or Path-Cost(child) < Path-Cost(reached[s]) then
///             reached[s] <- child
///             add child to frontier
///             if s is in reached_2 then
///                 solution_2 <- Join-Nodes(dir, child, reached_2[s]))
///                 if Path-Cost(solution_2) < Path-Cost(solution) then
///                     solution <- solution_2
///     return solution
///

use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    fmt::Debug
};

use crate::algorithms::{
    problem::*,
    node::*
};

enum Direction {
    Forward,
    Backward
}

pub fn bibf_search<P, S, A>(problem: &P) -> SearchResult<S, A> 
where
    P: Problem<S, A>,
    S: Clone + Eq + Ord + Hash + Debug,
    A: Clone + Eq + Ord + Hash + Debug
{
    let node_f = problem.get_initial_node();
    let node_b = problem.get_goal_node();
    let mut frontier_f: Vec<Node<S, A>> = vec![node_f.clone()];
    let mut frontier_b: Vec<Node<S, A>> = vec![node_b.clone()];
    let mut reached_f: HashMap<S, Node<S, A>> = HashMap::new();
    let mut reached_b: HashMap<S, Node<S, A>> = HashMap::new();
    reached_f.insert(node_f.state.clone(), node_f);
    reached_b.insert(node_b.state.clone(), node_b);
    let mut solution = Err(SearchError::Failure);

    while !terminated(&solution, &frontier_f, &frontier_b) {
        frontier_f.sort();
        frontier_b.sort();
        if frontier_f.first().unwrap() < frontier_b.first().unwrap() {
            solution = proceed(problem, Direction::Forward, &mut frontier_f, &mut reached_f, &mut reached_b, solution.clone());
        } else {
            solution = proceed(problem, Direction::Backward, &mut frontier_b, &mut reached_b, &mut reached_f, solution.clone());
        }
    }

    solution
}



fn proceed<P, S, A>(problem: &P, dir: Direction, frontier: &mut Vec<Node<S, A>>, reached: &mut HashMap<S, Node<S, A>>, reached_2: &mut HashMap<S, Node<S, A>>, solution: SearchResult<S, A>) -> SearchResult<S, A> 
where 
    P: Problem<S, A>,
    S: Clone + Eq + Ord + Hash + Debug, 
    A: Clone + Eq + Ord + Hash + Debug
{
    if frontier.is_empty() {
        return Err(SearchError::Failure);
    }

    let mut new_solution = solution;
    let node = frontier.pop().unwrap();
    
    for child in expand(problem, node.clone()) {
        let s = child.state.clone();

        if !reached.contains_key(&s) || child.path_cost < reached.get(&s).unwrap().path_cost {
            reached.insert(s.clone(), child.clone());
            frontier.push(child.clone());
            if reached_2.contains_key(&s) {
                let solution_2 = join_nodes(&dir, child, reached_2.get(&s).unwrap().clone());
                if new_solution.is_err() || solution_2.path_cost < new_solution.as_ref().unwrap().path_cost {
                    new_solution = Ok(solution_2);
                }
            }
        }
    }

    return new_solution;
}

fn join_nodes<S, A>(dir: &Direction, node: Node<S, A>, reached: Node<S, A>) -> Node<S, A>
where
    S: Clone,
    A: Clone
{
    // Move all from other chain into a vector
    let mut prev: Option<Box<Node<S, A>>> = Some(Box::new(node.clone()));
    let mut tmp_vec: VecDeque<Box<Node<S, A>>> = VecDeque::from([Box::new(reached.clone())]);
    let mut n = reached.parent;

    while n.is_some() {
        tmp_vec.push_back(n.as_ref().unwrap().clone());
        n = n.unwrap().parent;
    }

    tmp_vec.pop_front();

    while let Some(mut current) = tmp_vec.pop_front() {
          current.parent = prev;
          prev = Some(current);
    }
/*
    while current.is_some() {
        next = current.clone().unwrap().parent;
        current.clone().unwrap().parent = prev;
        prev = current.clone();
        current = next;
    }
*/
    *prev.unwrap()
}

fn terminated<S, A>(solution: &SearchResult<S, A>, frontier_F: &Vec<Node<S, A>>, frontier_B: &Vec<Node<S, A>>) -> bool 
where
    S: Clone,
    A: Clone
{
    if frontier_F.is_empty() || frontier_B.is_empty() {
        true
    } else {
        false
    }
}
