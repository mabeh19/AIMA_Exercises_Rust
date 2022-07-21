/// function LRTA*-Agent(problem, s', h) returns an action
///             s, a, the previous state and action, initially null
///     persistent: result, a table mapping (s, a) to s', initially empty
///                 H, a table mapping s to a cost estimate, initially empty
///
///     if Is-Goal(s') then return stop
///     if s' is a new state (not in H) then H[s'] <- h(s')
///     if s is not null then
///         result[s,a] <- s'
///         H[s] <- min_{b in Actions(s)} LRTA*-Cost(s,b,result[s,b],H)
///
///     a <- argmin_{b in Actions(s)} LRTA*-Cost(problem,s',b,result[s',b],H)
///     s <- s'
///
///     return a
///
/// function LRTA*-Cost(problem,s,a,s',H) returns a cost estimate
///     if s' is undefined then return h(s)
///     else return problem.Action-Cost(s,a,s') + H[s']
///

use std::{
    collections::HashMap,
    hash::Hash
};

use crate::algorithms::{
    problem::*,
    agent::Agent
};

pub struct LrtaAgent<P, S, A> {
    s: Option<S>,
    a: Option<A>,
    result: HashMap<(S, A), S>,
    H: HashMap<S, f64>,
    h: fn(&P, &S) -> f64,
}

impl<P, S, A> Agent<P, S, A> for LrtaAgent<P, S, A> 
where
    P: Problem<S, A>,
    S: Clone + Eq + Hash,
    A: Clone + Eq + Hash
{

    fn new() -> Self {
        Self {
            s: None,
            a: None,
            result: HashMap::new(),
            H: HashMap::new(),
            h: heuristic_function
        }
    }


    fn step(&mut self, problem: &P, s_star: S) -> A {
        if problem.is_goal(&s_star) {
            return problem.stop();
        }
        if !self.H.contains_key(&s_star) {
            self.H.insert(s_star.clone(), (self.h)(problem, &s_star));
        }

        if self.s.is_some() {
           self.result.insert((self.s.as_ref().unwrap().clone(), self.a.as_ref().unwrap().clone()), s_star.clone());
           self.H.insert(self.s.as_ref().unwrap().clone(), self.min_cost(problem));
        }

        self.a = Some(self.argmin_cost(problem, &s_star));
        self.s = Some(s_star.clone());

        return self.a.as_ref().unwrap().clone();   
    }
}

impl<P, S, A> LrtaAgent<P, S, A>
where
    P: Problem<S, A>,
    S: Clone + Eq + Hash,
    A: Clone + Eq + Hash
{

    fn min_cost(&self, problem: &P) -> f64 
    {
        let mut minimum = f64::MAX;
        for action in problem.actions(self.s.as_ref().unwrap()) {
            let cost = self.lrta_cost(problem, self.s.clone(), &action, Some(problem.result(self.s.as_ref().unwrap(), &action)));
            minimum = if cost < minimum { cost } else { minimum };
        }

        return minimum;
    }

    fn argmin_cost(&self, problem: &P, s_star: &S) -> A
    {
        let mut tmp_vec: Vec<(f64, A)> = Vec::new();

        for action in problem.actions(s_star) {
            tmp_vec.push((self.lrta_cost(problem, Some(s_star.clone()), &action, Some(problem.result(s_star, &action))), action.clone()));
        }

        tmp_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        return tmp_vec.first().unwrap().clone().1;
    } 

    fn lrta_cost(&self, problem: &P, s: Option<S>, a: &A, s_star: Option<S>) -> f64
    {
        if s_star.is_none() {
            return (self.h)(problem, &s.unwrap());
        } else {
            return problem.action_cost(s.as_ref().unwrap(), a, s_star.as_ref().unwrap()) + (self.h)(problem, s_star.as_ref().unwrap());
        }
    }
}


fn heuristic_function<P, S, A>(problem: &P, state: &S) -> f64 
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    return problem.get_heuristic_cost(state);
}
