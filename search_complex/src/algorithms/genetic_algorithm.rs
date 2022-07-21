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

use rand::prelude::*;

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

    let mut rng = rand::thread_rng();

    let mut population: Vec<Node<String, A>> = instantiate_population(problem, 100);

    loop {
        let weights = weighted_by(problem, &population, genetic_fitness);
        let mut population2: Vec<Node<String, A>> = Vec::new();
        for _ in 0..population.len() {
            let (parent1, parent2) = weighted_random_choices(problem, &population, &weights, 2);
            let mut child = reproduce(problem, &parent1, &parent2);
            if rng.gen_range::<i32, _>(0..100) < 50 {
                child = mutate(problem, &child);
            }
            population2.push(child);
        }
        population.append(&mut population2.clone());
        
        // Filter out any NAN values
        population = population.iter().filter_map(|f| if f.path_cost.is_nan() { None } else {Some(f.clone()) }).collect();
        population.sort_by(|a, b| a.path_cost.partial_cmp(&b.path_cost).unwrap());
        
        for p in &population {
            is_fit_enough(problem, p);
        }

        /* Remove worst half of population */
        for _ in 0..population.len() / 2 {
            population.pop();
        }
        
        if is_fit_enough(problem, population.first().unwrap()) {
            break;
        }
    }

    Ok(population.first().unwrap().clone())
}

fn weighted_random_choices<P, A>(_problem: &P, population: &Vec<Individual<A>>, weights: &Vec<GeneticWeights>, num_parents: usize) -> (Individual<A>, Individual<A>)
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut rng = rand::thread_rng();
    let mut i: usize = 0;
    let mut parents: Vec<Individual<A>> = Vec::new();
    let pop_len = population.len();

    while parents.len() < num_parents {
        let mut weight_sum: f64 = 0.;

        let vec = weights.get(i % pop_len).unwrap();
        for w in vec {
            weight_sum += w;
        }

        if rng.gen_bool(weight_sum) {
            parents.push(population.get(i % pop_len).unwrap().clone());
        }
        i += 1;
    }

    (parents.get(0).unwrap().clone(), parents.get(1).unwrap().clone())
}

fn weighted_by<P, A>(problem: &P, population: &Vec<Node<String, A>>, fitness: fn(&P, &Individual<A>) -> GeneticWeights) -> Vec<GeneticWeights>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut weights: Vec<GeneticWeights> = Vec::new();
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
    let c = rand::thread_rng().gen_range(0..n-1);
    let mut node: Node<String, A> = Node {
        state: parent1.state.get(0..c).unwrap().to_string(),
        parent: None,
        action: None,
        path_cost: 0.,
        f: 0.
    };
    node.state.push_str(parent2.state.get(c..n).unwrap());
    node.path_cost = get_path_cost(problem, &node);
    return node;
}

fn is_fit_enough<P, A>(problem: &P, individual: &Individual<A>) -> bool 
where
    P: Problem<&'static str, A>,
    A: Clone
{
    const FITNESS: f64 = 1500.;

    let total_cost = get_path_cost(problem, individual);
    
    total_cost < FITNESS
}

fn mutate<P, A>(problem: &P, child: &Individual<A>) -> Individual<A>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut rng = rand::thread_rng();
    // Mutate a single part of the individuals dna
    let index: usize = rng.gen_range(0..child.state.len() / 2);
    let mut mutated_child = child.clone();

    // Move temporary state to the strand that is going to be changed
    let mut node = problem.get_initial_node();
    let mut n = 0;
    for c in child.state.chars() {
        if let Some(i) = c.to_digit(10) {
            let nodes = expand(problem, node.clone());
            if let Some(n) = nodes.get(i.clamp(0, nodes.len() as u32) as usize) {
                node = n.clone();
            }
        }
        n += 1;
        if n == index {
            break;
        }
    }

    let actions_size = problem.actions(&node.state).len();
    let mut new_val = rng.gen_range(0..actions_size);
    
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
                if problem.is_goal(&tmp.state) {
                    break;
                }
            }
        }
    }

    return weights;
}

fn instantiate_population<P, A>(problem: &P, size: usize) -> Vec<Individual<A>>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut pop = Vec::new();

    for _ in 0..size {
        pop.push(rand_individual(problem)); 
    }

    pop
}

fn rand_individual<P, A>(problem: &P) -> Individual<A>
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut rng = rand::thread_rng();
    const MAX_MOVES: usize = 10;
    let mut dna = String::new();
    let mut node = problem.get_initial_node();

    for _ in 0..MAX_MOVES {
        
        let nodes = expand(problem, node.clone());
        
        let choice = rng.gen_range(0..nodes.len()) as u8;
        dna.push_str(format!("{}", choice).as_str());
        node = nodes.get(choice as usize).unwrap().clone();
    }

    let mut new_individual = Individual::new(dna, None, None, 0., 0.);
    new_individual.path_cost = get_path_cost(problem, &new_individual);

    new_individual
}

fn get_path_cost<P, A>(problem: &P, individual: &Individual<A>) -> f64 
where
    P: Problem<&'static str, A>,
    A: Clone
{
    let mut node = problem.get_initial_node();
    let mut total_cost = 0.;

    for c in individual.state.chars() {
        if let Some(index) = c.to_digit(10) {
            let choices = expand(problem, node.clone());
            if let Some(tmp) = choices.get(index as usize) {
                node = tmp.clone();
                total_cost += node.path_cost + problem.get_heuristic_cost(&node.state);
            }
        }
    }
    
    total_cost
}

pub fn iterate_over_dna<P, S, A, T>(problem: &P, individual: &Individual<A>, callback: fn(Node<S, A>, &mut T), param: &mut T)
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    let mut node = problem.get_initial_node();

    for c in individual.state.chars() {
        if let Some(index) = c.to_digit(10) {
            let choices = expand(problem, node.clone());
            if let Some(tmp) = choices.get(index as usize) {
                (callback)(tmp.clone(), param);
                node = tmp.clone();
            }
        }
    }
}
