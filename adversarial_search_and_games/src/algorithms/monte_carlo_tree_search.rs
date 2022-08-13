/// function Monte-Carlo-Tree-Search(state) returns an action
///     tree <- Node(state)
///     while Is-Time-Remainig() do
///         leaf <- Select(tree)
///         child <- Expand(leaf)
///         result <- Simulate(child)
///         Back-Propagate(result, child)
///     return the move in Actions(state) whose node as highest number of playouts
///
use rand::prelude::*;

use crate::algorithms::game::*;

const MAX_TIME: std::time::Duration = std::time::Duration::from_secs(5);
/*
pub fn monte_carlo_tree_search<G, P, S, A>(game: &G, state: &S) -> A
where
    G: Game<S, A, P>,
    P: Player<S, A>,
    S: Clone,
    A: Clone
{
    let tree = state.clone();
    let start = std::time::Instant::now();
    while start.elapsed() < MAX_TIME {

    }

    
}

fn select<G, P, S, A>(game: &G, state: &S) -> S
where
    G: Game<S, A, P>,
    P: Player<S, A>,
    S: Clone,
    A: Clone
{
    state.clone()
}

fn expand<G, P, S, A>(game: &G, state: &S) -> S
where
    G: Game<S, A, P>,
    P: Player<S, A>,
    S: Clone,
    A: Clone
{

    state.clone()
}

fn simulate<G, P, S, A>(game: &G, state: &S) -> bool
where
    G: Game<S, A, P>,
    P: Player<S, A>,
    S: Clone,
    A: Clone
{
    let rng = rand::thread_rng();
    if rng.gen_ratio((50. + game.utility(state, G::to_move(state))) as u32, 100) {
        true
    } else {
        false 
    }
}
*/
