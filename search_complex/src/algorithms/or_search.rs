/// function And-Or-Search(problem) returns a conditional plan, or failure
///     return Or-Search(problem, problem.initial, [])
///
/// function Or-Search(problem, state, path) returns a conditional plan, or failure
///     if problem.is_goal(state) then return the empty plan
///     if Is-Cycle(path) then return failure
///     for each action in problem.Actions(state) do
///         plan <- And-Search(problem, Results(state, action), [state] + path])
///         if plan != failure then return [action] + plan]
///     return failure
///
/// function And-Search(problem, states, path) returns a conditional plan, or failure
///     for each s_i in states do
///         plan_i <- Or-Search(problem, s_i, path)
///         if plan_i = failure then return failure
///     return [if s_1 then plan_1 else if s_2 then plan_2 else ... if s_n-1 then plan_n-1 else
///     plan_n]
///

use std::fmt::Debug;

use crate::algorithms::problem::*;

pub type Plan<A> = Result<Vec<A>, SearchError>;

pub fn and_or_search<P, S, A>(problem: &P) -> Plan<A>
where
    P: Problem<S, A>,
    S: Clone + PartialEq + Debug,
    A: Clone + Debug
{
    let mut path: Vec<S> = Vec::new();
    let mut plan: Vec<A> = Vec::new();
    or_search(problem, problem.get_initial_node().state, &mut path, &mut plan)?;
    Ok(plan)
}

fn or_search<P, S, A>(problem: &P, state: S, path: &mut Vec<S>, plan: &mut Vec<A>) -> Plan<A>
where
    P: Problem<S, A>,
    S: Clone + PartialEq + Debug,
    A: Clone + Debug
{
    if problem.is_goal(&state) {
        return Ok(Vec::new());
    }
    
    if path.contains(&state) {
        return Err(SearchError::Failure);
    }

    for action in problem.actions(&state) {
        path.push(state.clone());
        if let Ok(mut p) = and_search(problem, &problem.results(&state), path, plan) {
            plan.push(action.clone());
            p.push(action);
            return Ok(p);
        }
        path.pop();
    }
     
    return Err(SearchError::Failure);
}

fn and_search<P, S, A>(problem: &P, states: &Vec<S>, path: &mut Vec<S>, plan: &mut Vec<A>) -> Plan<A> 
where
    P: Problem<S, A>,
    S: Clone + PartialEq + Debug,
    A: Clone + Debug
{
    let mut plans: Vec<Plan<A>> = Vec::new();
    for s in states {
        let plan_i = or_search(problem, s.clone(), path, plan);
        if plan_i.is_err() {
            //return Err(SearchError::Failure);
        } else if plan_i.is_ok() {
            plans.push(plan_i); 
        }
    }

    if plans.is_empty() {
        return Err(SearchError::Failure);
    }

    return plans.last().unwrap().clone();
}

