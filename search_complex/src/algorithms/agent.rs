
use crate::algorithms::problem::*;


pub trait Agent<P, S, A> 
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    fn new() -> Self;
    fn step(&mut self, problem: &P, s_star: S) -> A;
}

