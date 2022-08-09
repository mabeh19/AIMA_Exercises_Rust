/// function Alpha-Beta-Search(game, state) returns an action
///     player <- game.To-Move(state)
///     value,move <- Max-Value(game, state, -infty, +infty)
///     return move
/// 
/// function Max-Value(game, state, alpha, beta) returns a (utlity, move) pair
///     if game.is_terminal(state) then return game.utility(state, player), null
///     v <- -infty
///     for each a in game.actions(state) do
///         v2, a2 <- Min-Value(game, game.result(state, a), alpha, beta)
///         if v2 > v then
///             v, move <- v2, a
///             alpha <- Max(alpha, v)
///         if v >= beta then return v, move
///     return v, move
///
/// function Min-Value(game, state, alpha, beta) returns a (utility, move) pair
///     if game.Is-Terminal(state) then return game.Utlity(state, player), null
///     v <- +infty
///     for each a in game.actions(state) do
///         v2, a2 <- Max-Value(game, game.result(state, a), alpha, beta)
///         if v2 < v then
///             v, move <- v2, a
///             beta <- Min(beta, v)
///         if v <= alpha then return v, mvoe
///     return v, move
///

use crate::algorithms::game::*;

pub fn alpha_beta_search<G, S, A, P>(game: &G, state: &S, depth: usize) -> Option<A>
where
    G: Game<S, A, P> + Clone,
    S: Clone,
    A: Clone,
    P: Player<S, A>
{
    let mut search_depth = depth;
    let mut game_copy = game.clone();
    let _player = G::to_move(state);
    let (_value, move_) = max_value(&mut game_copy, state, f64::NEG_INFINITY, f64::INFINITY, &mut search_depth);
    return move_;
}

fn max_value<G, S, A, P>(game: &mut G, state: &S, alpha: f64, beta: f64, depth: &mut usize) -> (f64, Option<A>)
where
    G: Game<S, A, P>,
    S: Clone,
    A: Clone,
    P: Player<S, A>
{
    *depth -= 1;
    let mut alpha = alpha;

    if *depth == 0 || game.is_terminal(state) {
        *depth += 1;
        return (game.utility(state, G::to_move(state)), None);
    }

    let mut v = f64::NEG_INFINITY;
    let mut move_: Option<A> = None;
    for a in game.actions(state) {
        let new_state = game.result(state, &a);
        let (v2, _a2) = min_value(game, &new_state, alpha, beta, depth);
        if v2 > v {
            (v, move_) = (v2, Some(a));
            alpha = alpha.max(v);
        }
        if v >= beta {
            *depth += 1;
            return (v, move_);
        }
    }

    *depth += 1;
    return (v, move_);
}

fn min_value<G, S, A, P>(game: &mut G, state: &S, alpha: f64, beta: f64, depth: &mut usize) -> (f64, Option<A>)
where
    G: Game<S, A, P>,
    S: Clone,
    A: Clone,
    P: Player<S, A>
{
    *depth -= 1;
    let mut beta = beta;

    if *depth == 0 ||game.is_terminal(state) {
        *depth += 1;
        return (game.utility(state, G::to_move(state)), None);
    }
    let mut v = f64::INFINITY;
    let mut move_: Option<A> = None;
    for a in game.actions(state) {
        let new_state = game.result(state, &a);
        let (v2, _a2) = max_value(game, &new_state, alpha, beta, depth);
        if v2 < v {
            (v, move_) = (v2, Some(a));
            beta = beta.min(v);
        }
        if v <= alpha {
            *depth += 1;
            return (v, move_);
        }
    }
    *depth += 1;
    return (v, move_);
}
