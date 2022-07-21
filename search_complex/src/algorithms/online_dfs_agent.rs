/// function Online-DFS-Agent(problem, s') returns an action
///             s, a, the previous state and action initially null
///     persistent: result, a table mapping (s, a) to s', initially empty
///                 untried, a table mapping s to a list of untried actions
///                 unbacktracked, a table mapping s to a list of states never backtracked to
///
///     if problem.Is-Goal(s') then return stop
///     if s' is a new state (not in untried) then untried[s'] <- problem.Actions(s')
///     if s is not null then
///         result[s,a] <- s'
///         add s to the front of unbacktracked[s']
///
///     if untried[s'] is empty then
///         if unbacktracked[s'] is empty then return stop
///         else a <- an action b such that result[s',b] = Pop(unbakctracked[s'])
///
///     else a <- Pop(untried[s'])
///     s <- s'
///     return a
///

use std::{
    collections::HashMap,
    hash::Hash,
};

use crate::algorithms::{
    problem::*,
    agent::Agent,
};

pub struct OnlineDFSAgent<S, A>
where
    S: Clone + Eq + Hash,
    A: Clone + Eq + Hash
{
    s: Option<S>,
    a: Option<A>,
    result: HashMap<(S, A), S>,
    untried: HashMap<S, Vec<A>>,
    unbacktracked: HashMap<S, Vec<S>>, 
}

impl<P, S, A> Agent<P, S, A> for OnlineDFSAgent<S, A>
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
            untried: HashMap::new(),
            unbacktracked: HashMap::new()
        }
    }

    fn step(&mut self, problem: &P, s_star: S) -> A
    {
        if problem.is_goal(&s_star) {
            return problem.stop();
        }
        
        if !self.untried.contains_key(&s_star) {
            self.untried.insert(s_star.clone(), problem.actions(&s_star));
        }
        
        if self.s.is_some() {
            self.result.insert((self.s.as_ref().unwrap().clone(), self.a.as_ref().unwrap().clone()), s_star.clone());
            if !self.unbacktracked.contains_key(&s_star) {
                self.unbacktracked.insert(s_star.clone(), Vec::new());
            }
            self.unbacktracked.get_mut(&s_star).unwrap().push(self.s.as_ref().unwrap().clone());
        }

        if self.untried.get(&s_star).unwrap().is_empty() {
            if self.unbacktracked.get(&s_star).unwrap().is_empty() {
                return problem.stop();
            } else {
                let candidate_states = &self.unbacktracked.get(&s_star).unwrap();
                for b in problem.actions(&s_star) {
                    if candidate_states.contains(&problem.result(&s_star, &b)) {
                        self.a = Some(b.clone());
                        break;
                    }
                }
            }
        } else {
            self.a = self.untried.get_mut(&s_star).unwrap().pop();
        }

        self.s = Some(s_star);

        return self.a.as_ref().unwrap().clone();
    }

}
