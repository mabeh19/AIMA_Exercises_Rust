///
/// MonteCarloNode structure for use in algorithms
///
use std::sync::{Arc, Mutex};

pub type Link<S, A> = Option<Arc<Mutex<MonteCarloNode<S, A>>>>;

#[derive(Clone, Debug)]
pub struct MonteCarloNode<S, A> {
    pub state: S,
    pub parent: Link<S, A>,
    pub children: Vec<Link<S, A>>,
    pub action: Option<A>,
    pub playouts: u32,
    pub wins: u32
/*
    pub action: Option<A>,
    pub path_cost: u32,
    pub f: u32
*/
}

impl<S, A> std::fmt::Display for MonteCarloNode<S, A>
{
    fn fmt (&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{{ playouts: {} wins: {} }}", self.playouts, self.wins)
    }
}


impl<S, A> MonteCarloNode<S, A> {
    
    pub const fn new(state: S, parent: Link<S, A>, action: Option<A>) -> Self {
        Self { 
            state, 
            parent,
            children: Vec::new(),
            action,
            playouts: 0,
            wins: 0
        }
    }
}

/*
pub fn depth<S, A>(start_node: &Link<S, A>) -> usize 
where 
    S: Clone, 
{
    let mut len: usize = 1;
    let mut node = start_node;
    loop {
        if node.is_some() {
            len += 1;
            node = &node.unwrap().borrow().parent;
        } else {
            break;
        }
    }
    drop(node);
    len
}

pub fn is_cycle<S, A>(node: &Link<S, A>) -> bool 
where 
    S: Clone + Eq, 
{
    let mut n = node;
    let tmp_n = node;
    loop {
        n = &n.unwrap().borrow().parent;
        if n.is_none() {
            break;
        }
        
        if tmp_n.unwrap().borrow().state == n.as_ref().unwrap().borrow().state {
            drop(tmp_n);
            drop(n);
            return true;
        }
    }
    
    drop(tmp_n);
    drop(n);

    false
}

*/
