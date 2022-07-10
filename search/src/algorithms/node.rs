///
/// Node structure for use in algorithms
///


#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub struct Node<S, A> {
    pub state: S,
    pub parent: Option<Box<Node<S, A>>>,
    pub action: Option<A>,
    pub path_cost: f32
}

impl<S, A> Node<S, A> {
    
    pub const fn new(state: S, parent: Option<Box<Node<S, A>>>, action: Option<A>, path_cost: f32) -> Self {
        Self { state, parent, action, path_cost }
    }   
}
