
use std::fmt::Display;

mod algorithms;

use algorithms::{
    node::Node,
    problem::{Problem, SearchResult, ARAD_TO_BUCHAREST_PROBLEM},
    best_first_search,
    breadth_first_search,
    iterative_deepening_search,
    bibf_search,
};

type SearchAlgorithm<P: Problem<S, A>, S, A> = fn(&P) -> SearchResult<S, A>;

fn main() {
    try_algorithm(&ARAD_TO_BUCHAREST_PROBLEM, "Best First Search", best_first_search::best_first_search);
    try_algorithm(&ARAD_TO_BUCHAREST_PROBLEM, "Breadth First Search", breadth_first_search::breadth_first_search);
    try_algorithm(&ARAD_TO_BUCHAREST_PROBLEM, "Iterative Deepening Search", iterative_deepening_search::iterative_deepening_search);
    try_algorithm(&ARAD_TO_BUCHAREST_PROBLEM, "Bidirectional Best First Search", bibf_search::bibf_search);
}

fn try_algorithm<P, S, A>(problem: &P, name: &str, algorithm: SearchAlgorithm<P, S, A>) 
where
    P: Problem<S, A>,
    S: Clone + Display,
    A: Clone
{
    let res = algorithm(problem).expect("No path found");
    let mut node = Some(Box::new(res));
    println!("[{}] Optimal path from Arad to Bucharest:", name);
    while node.is_some() {
        println!("      {}", node.as_ref().unwrap().state);
        node = node.unwrap().parent;
    }
}
