

use std::fmt::{Display, Debug};


mod algorithms;

use crate::algorithms::{
    node::Node,
    problem::*,
    hill_climbing,
    simulated_annealing,
    genetic_algorithm,
    or_search::{self, Plan},
};

type SearchAlgorithm<P, S, A> = fn(&P) -> SearchResult<S, A>;
type SearchAlgorithmHeuristic<P, S, A> = fn(&P, fn(&P, &Node<S, A>) -> f64) -> SearchResult<S, A>;

fn main() {
    try_algorithm(&GRAPH_PROBLEM, "hill_climbing", hill_climbing::hill_climbing);
    try_algorithm(&GRAPH_PROBLEM, "Simulated Annealing", simulated_annealing::simulated_annealing);
    try_genetic(&ARAD_TO_BUCHAREST_PROBLEM, "Genetic Algorithm", genetic_algorithm::genetic_algorithm);
    try_or_search(&ARAD_TO_BUCHAREST_PROBLEM, "And-Or Search", or_search::and_or_search);
}

fn try_algorithm<P, S, A>(problem: &P, name: &str, algorithm: SearchAlgorithm<P, S, A>) 
where
    P: Problem<S, A>,
    S: Clone + Display,
    A: Clone
{
    let res = algorithm(problem).expect("No path found");
    let mut node = Some(Box::new(res));
    println!("[{}] Optimal x:", name);
    //while node.is_some() {
        println!("      {}", node.as_ref().unwrap().state);
    //    node = node.unwrap().parent;
    //}
}

fn try_genetic<P, S, A>(problem: &P, name: &str, algorithm: SearchAlgorithm<P, String, A>) 
where
    P: Problem<S, A>,
    S: Clone + Display,
    A: Clone
{
    let res = algorithm(problem).expect("No path found");
    let mut node = Some(Box::new(res));
    println!("[{}] Optimal x:", name);
    //while node.is_some() {
        println!("      {}", node.as_ref().unwrap().state);
    //    node = node.unwrap().parent;
    //}
}

fn try_or_search<P, S, A>(problem: &P, name: &str, algorithm: fn(&P) -> Plan<A>)
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone + Debug
{
    let plan = algorithm(problem).expect("No path found");
    println!("[{}] Optimal plan:", name);
    for a in plan {
        println!("      {:?}", a);
    //    node = node.unwrap().parent;
    }
}

fn try_heuristic<P, S, A>(problem: &P, name: &str, algorithm: SearchAlgorithmHeuristic<P, S, A>, h: fn(&P, &Node<S, A>) -> f64)
where
    P: Problem<S, A>,
    S: Clone + Display,
    A: Clone
{
    let res = algorithm(problem, h).expect("No Path found");
    let mut node = Some(Box::new(res));
    println!("[{}] Optimal path from Arad to Bucharest:", name);
    while node.is_some() {
        println!("      {}", node.as_ref().unwrap().state);
        node = node.unwrap().parent;
    }
}

fn h<P, S, A>(problem: &P, node: &Node<S, A>) -> f64
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    problem.get_heuristic_cost(&node.state)
}
