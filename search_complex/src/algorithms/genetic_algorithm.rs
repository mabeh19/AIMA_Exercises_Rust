use std::cmp::Ordering;

///
/// function Genetic-Algorithm(population, fitness) returns an individual
///     repeat
///         weights <- Weighted-By(population, fitness)
///         population2 <- empty list
///         for i = 1 to Size(population) do
///             parent1, parent2 <- Weighted-Random-Choices(population, weights, 2)
///             child <- reproduce(parent1, parent2)
///             if (small random probability) then child <- Mutate(child)
///             add child to population2
///         population <- population2
///     until some individual is fit enough, or enough time has elapsed
///     return the best individual in population, according to fitness
///
/// function Reproduce(parent1, parent2) returns an individual
///     n <- Length(parent1)
///     c <- random number from 1 to n
///     return Append(Substring(parent1, 1, c), Substring(parent2, c+1, n))
///

use std::cmp::Ordering;

use rand::prelude::*;
use lazy_static::lazy_static;

use crate::algorithms::{
    node::Node,
    problem::*,
};

type GeneticWeights = Vec<f64>;
type Individual<A> = Node<String, A>;

pub fn genetic_algorithm<P, A>(problem: &P) -> SearchResult<String, A> 
where
    P: Problem<&'static str, A>,
    A: Clone
{

    let rng = rand::thread_rng();

    let mut population: Vec<Node<String, A>> = Vec::new();

    while {
        let weights = weighted_by(problem, &population, genetic_fitness);
        let mut population2: Vec<Node<String, A>> = Vec::new();
        for i in 1..population.len() {
            let (parent1, parent2) = weighted_random_choices(problem, &population, &weights, 2);
            let mut child = reproduce(problem, &parent1, &parent2);
            if rng.gen_range::<i32, _>(1..1000) == 1 {
                child = mutate(problem, &child);
            }
        }
        population.append(&mut population2);


        // Filter out any NAN values
        population = population.iter().filter_map(|f| if f.path_cost.is_nan() { None } else {Some(*f) }).collect();
        population.sort_by(cmp_f64);

        is_fit_enough(problem, population.first().unwrap())
    }{}

    Err(SearchError::Failure)
}

fn weighted_random_choices<P, A>(problem: &P, population: &Vec<Individual<A>>, weights: &Vec<GeneticWeights>, num_parents: usize) -> (Individual<A>, Individual<A>)
where
    P: Problem<&'static str, A>,
    A: Clone
{

}

fn cmp_f64<S, A>(a: &Node<S, A>, b: &Node<S, A>) -> Ordering {
    if a.path_cost.is_nan() {
        return Ordering::Greater;
    }
    if b.path_cost.is_nan() {
        return Ordering::Less;
    }
    if a.path_cost < b.path_cost {
        return Ordering::Less;
    } else if a.path_cost > b.path_cost {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn weighted_by<P, A>(problem: &P, population: &Vec<Node<String, A>>, fitness: fn(&P, &Individual<A>) -> GeneticWeights) -> Vec<GeneticWeights>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let weights: Vec<GeneticWeights> = Vec::new();
    for individual in population {
        weights.push(fitness(problem, &individual));
    }

    return weights;
}

fn reproduce<P, A>(problem: &P, parent1: &Node<String, A>, parent2: &Node<String, A>) -> Node<String, A>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let n = parent1.state.len();
    let c = rand::thread_rng().gen_range(0..n);
    let node: Node<String, A> = Node {
        state: parent1.state.get(0..c).unwrap().to_string(),
        parent: None,
        action: None,
        path_cost: 0.,
        f: 0.
    };
    node.state.push_str(parent2.state.get(c+1..n).unwrap());

    return node;
}

fn is_fit_enough<P, A>(problem: &P, individual: &Individual<A>) -> bool 
where
    P: Problem<&'static str, A>,
    A: Clone
{
    const FITNESS: u32 = 450;

    let mut node = problem.get_initial_node();
    let mut total_cost = 0.;
    for c in individual.state.chars() {
        if let Some(index) = c.to_digit(10) {
            let choices = expand(problem, node.clone());
            if let Some(tmp) = choices.get(index as usize) {
                node = tmp.clone();
                total_cost += node.path_cost;
            }
        }
    }

    total_cost < FITNESS as f64
}

fn mutate<P, A>(problem: &P, child: &Individual<A>) -> Individual<A>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let rng = rand::thread_rng();
    // Mutate a single part of the individuals dna
    let index: usize = rng.gen_range(0..child.state.len());

    let mut mutated_child = child.clone();

    // Move temporary state to the strand that is going to be changed
    let node = problem.get_initial_node();
    let mut n = 0;
    for c in child.state.chars() {
        if let Some(i) = c.to_digit(10) {
            node = expand(problem, node.clone()).get(i as usize).unwrap().clone();
        }
        n += 1;
        if n == index {
            break;
        }
    }

    let actions_size = problem.actions(&node.state).len();
    let new_val = rng.gen_range(0..actions_size);
    
    while new_val == index {
        new_val = rng.gen_range(0..actions_size);
    }

    mutated_child.state.replace_range(index as usize..index as usize + 1, new_val.to_string().as_str());

    mutated_child
}

fn genetic_fitness<P, A>(problem: &P, individual: &Individual<A>) -> GeneticWeights 
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut weights: GeneticWeights = GeneticWeights::new();
    
    let mut node = problem.get_initial_node();

    /*
     * Our DNA string described a choice at each node
     */
    for c in individual.state.chars() {
        if let Some(index) = c.to_digit(10) {
            let choices = expand(problem, node.clone());
            if let Some(tmp) = choices.get(index as usize) {
                node = tmp.clone();
                weights.push(1. / node.path_cost);
            }
        }
    }

    return weights;
}

const STATES: [&'static str; 20] = [
    "Arad",
    "Bucharest",
    "Craoiva",
    "Drobeta",
    "Eforie",
    "Fagaras",
    "Giurgiu",
    "Hirsova",
    "Iasi",
    "Lugoj",
    "Mehadia",
    "Neamt",
    "Oradea",
    "Pitesti",
    "Rimnicu Vilcea",
    "Sibiu",
    "Timisoara",
    "Urziceni",
    "Vaslui",
    "Zerind"
];
