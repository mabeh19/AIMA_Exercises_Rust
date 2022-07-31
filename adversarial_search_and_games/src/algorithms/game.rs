pub trait Game<S, A, P> 
where
    S: Clone,
    A: Clone,
    P: Player<S, A>
{
    fn create_game() -> Self;   
    fn get_initial_state(&self) -> &S;
    fn to_move(state: &S) -> &P;
    fn actions(&self, state: &S) -> Vec<A>;
    fn result(&mut self, state: &S, action: &A) -> S;
    fn is_terminal(&self, state: &S) -> bool;
    fn utility(&self, state: &S, player: &P) -> f64;
    fn take_action(&mut self, state: &S, action: &A) -> &S;
}


pub trait Player<S, A>
where
    S: Clone,
    A: Clone
{

}

