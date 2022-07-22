
use crate::algorithms::games::{
    chess
};


pub trait Game<S, A, P> 
where
    S: Clone,
    A: Clone,
    P: Player<S, A>
{
    fn create_game() -> Self;   
    fn get_initial_state(&self) -> &S;
    fn to_move(&self, state: &S) -> &P;
    fn actions(&self, state: &S) -> Vec<A>;
    fn result(&self, state: &S, action: &A) -> S;
    fn is_terminal(&self, state: &S) -> bool;
    fn utility(&self, state: &S, player: &P) -> f64;
}


pub trait Player<S, A>
where
    S: Clone,
    A: Clone
{

}

