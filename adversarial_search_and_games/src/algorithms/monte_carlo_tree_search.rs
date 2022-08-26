#![allow(unused_variables)]
/// function Monte-Carlo-Tree-Search(state) returns an action
///     tree <- Node(state)
///     while Is-Time-Remainig() do
///         leaf <- Select(tree)
///         child <- Expand(leaf)
///         result <- Simulate(child)
///         Back-Propagate(result, child)
///     return the move in Actions(state) whose node as highest number of playouts
use std::sync::{Arc, Mutex};

use rand::prelude::*;

use crate::algorithms::{
    game::*,
    node::*
};

const MAX_TIME: std::time::Duration = std::time::Duration::from_secs(1);
const MAX_STATES_PER_LEAF: usize = 10; // Only explore first 10 states

pub struct MonteCarloTree<S, A> 
where
    S: Clone,
    A: Clone + PartialEq
{
    pub root: Link<S, A>,
}

impl<S, A> MonteCarloTree<S, A> 
where
    S: Clone + PartialEq,
    A: Clone + PartialEq
{
    pub fn new(state: &S) -> Self {
        Self {
            root: Some(Arc::new(Mutex::new(MonteCarloNode::new(state.clone(), None, None))))
        }
    }

    pub fn monte_carlo_tree_search<G, P>(&mut self, game: &G, state: &S) -> Option<A>
    where
        G: Game<S, A, P> + Clone,
        P: Player<S, A>,
        S: PartialEq
    {
        let mut local_game = game.clone();
        let tree = state.clone();
        let start = std::time::Instant::now();

        while start.elapsed() < MAX_TIME {
            let mut leaf = self.select(&local_game, &tree);
            let mut child = Self::expand(&mut local_game, &mut leaf);
            if child.is_some() {
                leaf.as_mut().unwrap().lock().unwrap().children.push(child.clone());
                let result = self.simulate(&local_game, &child);
                Self::back_propagate(&local_game, result, &mut child);
            } else {
                Self::back_propagate(&local_game, false, &mut leaf);
            }
        }
        
        let mut ret_action: (u32, Option<A>) = (0, None);
        for c in &self.root.as_ref().unwrap().lock().unwrap().children {
            let c = c.as_ref().unwrap().lock().unwrap();
            if c.playouts > ret_action.0 {
                ret_action = (c.playouts, c.action.clone());
            }
        }

        for c in &Arc::clone(self.root.as_ref().unwrap()).lock().unwrap().children {
            if c.as_ref().unwrap().lock().unwrap().action == ret_action.1 {
                self.root = c.clone();
                break;
            }
        }

        ret_action.1
    }


    ///
    /// Starting at root of search tree, choose a move leading to a successor node,
    /// repeat the process, moving down the tree to a leaf, based on select policy
    ///
    fn select<G, P>(&mut self, game: &G, state: &S) -> Link<S, A>
    where
        G: Game<S, A, P>,
        P: Player<S, A>
    {
        let mut child = Arc::clone(self.root.as_ref().unwrap());
        while child.lock().unwrap().children.len() == MAX_STATES_PER_LEAF {
            child.as_ref().lock().unwrap().children.sort_by(|a, b| {
                let a = a.as_ref().unwrap().lock().unwrap();
                let b = b.as_ref().unwrap().lock().unwrap();
                if a.playouts < b.playouts { 
                    std::cmp::Ordering::Greater
                } else { 
                    std::cmp::Ordering::Less
                }
            });
            let tmp = Arc::clone(&child);
            let tmp = tmp.lock().unwrap();
            let best_candidate = tmp.children.first().unwrap();
            child = Arc::clone(best_candidate.as_ref().unwrap());
            drop(best_candidate);
            drop(tmp);
        }
        return Some(child);
    }

    ///
    /// Grow the search tree by generating a new child of the selected node
    ///
    fn expand<G, P>(game: &mut G, state: &mut Link<S, A>) -> Link<S, A>
    where
        G: Game<S, A, P>,
        P: Player<S, A>
    {
        let mut rng = rand::thread_rng();
        let possible_states = game.actions(&state.as_ref().unwrap().lock().unwrap().state);
        if possible_states.is_empty() {
            return None;
        }
        let leaf = state.as_ref().unwrap().lock().unwrap();
        let choice = rng.gen_range(0..possible_states.len());
 
        /*
        for c in &leaf.children {
            if c.as_ref().unwrap().lock().unwrap().state == leaf.state {
                return None;
            }
        }
        */

        let new_child = Arc::new(Mutex::new(MonteCarloNode::new(
            game.result(&leaf.state, &possible_states[choice]),
            Some(Arc::clone(&state.as_ref().unwrap())),
            Some(possible_states[choice].clone())
        )));
        
        return Some(new_child);
    }

    ///
    /// Perform playout from newly generated child node,
    /// choosing moves for both players according to the playout policy.
    /// These moves are NOT recorded in search tree.
    ///
    fn simulate<G, P>(&self, game: &G, state: &Link<S, A>) -> bool
    where
        G: Game<S, A, P>,
        P: Player<S, A>
    {
        let mut rng = rand::thread_rng();
        let state_copy = state.as_ref().unwrap().lock().unwrap().state.clone();
        let eval = game.utility(&state_copy, G::to_move(&state_copy));
        if rng.gen_ratio((50. + eval * 100.).clamp(0.,100.) as u32, 100) {
            true
        } else {
            false 
        }
    }

    ///
    /// Use the result of the simulation to update all the search tree nodes
    /// going up to the root. 
    ///
    fn back_propagate<G, P>(game: &G, result: bool, state: &mut Link<S, A>) 
    where
        G: Game<S, A, P>,
        P: Player<S, A>
    {
        let mut node = Arc::clone(state.as_ref().unwrap());
        loop {
            let mut n = node.lock().unwrap();           
            n.playouts += 1;
            if result == true {
                n.wins += 1;
            }
            if n.parent.is_none() {
                break;
            } else {
                let tmp = Arc::clone(n.parent.as_ref().unwrap());
                drop(n);
                node = tmp;
            }
        }
    }
}

pub fn print_playouts<S, A>(mcts: &Link<S, A>) {
    let mcts = mcts.as_ref().unwrap().lock().unwrap();
    println!("Evaluations after 1 iteration: {}, no. children: {}", mcts, mcts.children.len());
    mcts.children.clone().into_iter().for_each( |c| {
        print_playouts(&c);
    });
}
