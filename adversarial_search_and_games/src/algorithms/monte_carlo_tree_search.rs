/// function Monte-Carlo-Tree-Search(state) returns an action
///     tree <- Node(state)
///     while Is-Time-Remainig() do
///         leaf <- Select(tree)
///         child <- Expand(leaf)
///         result <- Simulate(child)
///         Back-Propagate(result, child)
///     return the move in Actions(state) whose node as highest number of playouts
///

use crate::algorithms::game::*;

const MAX_TIME: std::time::Duration = std::time::Duration::from_secs(5);

pub fn monte_carlo_search<G, P, S, A>(game: &G, state: &S) -> A
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
    
}

fn expand<G, P, S, A>(game: &G, state: &S) -> S
where
    G: Game<S, A, P>,
    P: Player<S, A>,
    S: Clone,
    A: Clone
{

}
