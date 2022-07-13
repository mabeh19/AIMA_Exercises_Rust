///
/// Node structure for use in algorithms
///


#[derive(Clone, PartialOrd, PartialEq, Eq, Debug, Ord)]
pub struct Node<S, A> {
    pub state: S,
    pub parent: Option<Box<Node<S, A>>>,
    pub action: Option<A>,
    pub path_cost: i32
}

impl<S, A> Node<S, A> {
    
    pub const fn new(state: S, parent: Option<Box<Node<S, A>>>, action: Option<A>, path_cost: i32) -> Self {
        Self { state, parent, action, path_cost }
    }
}

pub fn depth<S, A>(start_node: Node<S, A>) -> usize 
where 
    S: Clone, 
    A: Clone
{
    let mut len: usize = 1;
    let mut node = Some(Box::new(start_node.clone()));
    loop {
        if node.is_some() {
            len += 1;
            node = node.unwrap().parent;
        } else {
            break;
        }
    }
    drop(node);
    len
}

pub fn is_cycle<S, A>(node: Node<S, A>) -> bool 
where 
    S: Clone + Eq, 
    A: Clone + Eq
{
    let mut n = Some(Box::new(node.clone()));
    let tmp_n = Box::new(node);
    loop {
        n = n.unwrap().parent;
        if n.is_none() {
            break;
        }
        
        if tmp_n.state == n.as_ref().unwrap().state {
            drop(tmp_n);
            drop(n);
            return true;
        }
    }
    
    drop(tmp_n);
    drop(n);

    false
}


