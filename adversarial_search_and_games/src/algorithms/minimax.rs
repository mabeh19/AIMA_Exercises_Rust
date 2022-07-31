/// function Minimax-Search(game, state) returns an action
///     player <- game.To-Move(state)
///     value, move <- Max-Value(game, state)
///     return move
///
/// function Max-Value(game, state) returns a (utility, move) pair
///     if game.Is-Terminal(state) then return game.utility(state, player), null
///     v <- -infty
///     for each a in game.Actions(state) do
///         v2, a2 <- Min-Value(game, game.Result(state, a))
///     if v2 > v then
///         v, move <- v2.a
///     return v, move
///
/// function Min-Value(game, state) returns a (utility, move) pair
///     if game.Is-Terminal(state) then return game.Utility(state, player), null
///     v <- +infty
///     for each a in game.Actions(state) do
///         v2, a2 <- Max-Value(game, game.Result(state, a))
///         if v2 < v then
///             v, move <- v2, a
///     return v, move

use std::fmt::Debug;

use crate::algorithms::game::*;

pub fn minimax_search<G, S, A, P>(game: &G, state: &S, depth: usize) -> Option<A>
where
    G: Game<S, A, P> + Clone,
    P: Player<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{
    let mut loc_game = game.clone(); // get local clone we can manipulate
    let mut cur_depth = depth; 
    let (_value, move_) = max_value(&mut loc_game, state, &mut cur_depth);
    return move_;
}

fn max_value<G, S, A, P>(game: &mut G, state: &S, cur_depth: &mut usize) -> (f64, Option<A>)
where
    G: Game<S, A, P> + Clone,
    P: Player<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{

    *cur_depth -= 1;

    if *cur_depth == 0 || game.is_terminal(state) {
        *cur_depth += 1;
        return (game.utility(state, G::to_move(state)), None);
    }
    
    let mut v = f64::NEG_INFINITY;
    let mut move_: Option<A> = None;
    for a in game.actions(state) {
        let state2 = game.result(state, &a);
        let (v2, _a2) = min_value(game, &state2, cur_depth);
        if v2 > v {
            (v, move_) = (v2, Some(a));
        }
    }
    
    *cur_depth += 1;

    return (v, move_);
}

fn min_value<G, S, A, P>(game: &mut G, state: &S, cur_depth: &mut usize) -> (f64, Option<A>)
where 
    G: Game<S, A, P> + Clone,
    P: Player<S, A>,
    S: Clone + Debug,
    A: Clone + Debug
{

    *cur_depth -= 1;
    if *cur_depth == 0 || game.is_terminal(state) {
        *cur_depth += 1;
        return (game.utility(state, G::to_move(state)), None);
    }
    
    let mut v = f64::INFINITY;
    let mut move_: Option<A> = None;
    for a in game.actions(state) {
        let state2 = game.result(state, &a);
        let (v2, _a2) = max_value(game, &state2, cur_depth);
        if v2 < v {
            (v, move_) = (v2, Some(a));
        }
    }

    *cur_depth += 1;

    return (v, move_);
}
