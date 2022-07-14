/* Std Library */
use std::collections::HashMap;

/* External crates */
use lazy_static::{lazy_static};

/* Internal crates */
use crate::algorithms::node::Node;

pub type SearchResult<S, A> = Result<Node<S, A>, SearchError>;

pub trait Problem<S, A> 
where 
    S: Clone,
    A: Clone
{
    fn is_goal(&self, state: &S) -> bool;
    fn actions(&self, state: &S) -> Vec<A>;
    fn result(&self, state: &S, action: &A) -> S;
    fn action_cost(&self, state: &S, action: &A, new_state: &S) -> i32;
    fn get_initial_node(&self) -> Node<S, A>;
    fn get_goal_node(&self) -> Node<S, A>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum SearchError {
    Failure,
    CutOffReached,
}

pub type Action = AradToBucharestAction;
pub type State = &'static str;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AradToBucharestAction {
    ToArad,
    ToSibiu,
    ToZerind,
    ToTimisoara,
    ToOradea,
    ToFagaras,
    ToRimnicuVilcea,
    ToLugoj,
    ToMehadia,
    ToDrobeta,
    ToCraiova,
    ToPitesti,
    ToBucharest,
    ToGiurgiu,
    ToUrziceni,
    ToHirsova,
    ToEforie,
    ToVaslui,
    ToIasi,
    ToNeamt
}

pub struct AradToBucharestProblem {
    initial_state: State,
    goal_state: State
}

impl Problem<State, Action> for AradToBucharestProblem {
    fn is_goal(&self, state: &State) -> bool {
        state == &self.goal_state
    }

    fn actions(&self, state: &State) -> Vec<Action> {
        ACTIONS.get(state).unwrap().clone()
    }

    fn result(&self, state: &State, action: &Action) -> State {
        *RESULT_STATE.get(&(state, action.clone())).unwrap()
    }

    fn action_cost(&self, state: &State, action: &Action, new_state: &State) -> i32 {
        *PATH_COST.get(&(state, new_state, action.clone())).unwrap()
    }

    fn get_initial_node(&self) -> Node<State, Action> {
        INITIAL_NODE
    }

    fn get_goal_node(&self) -> Node<State, Action> {
       GOAL_NODE 
    }
}

lazy_static! {
    pub static ref PATH_COST: HashMap<(State, State, Action), i32> = [
        (("Arad", "Sibiu", Action::ToSibiu), 140),
        (("Sibiu", "Arad", Action::ToArad), 140),
        (("Arad", "Zerind", Action::ToZerind), 75),
        (("Zerind", "Arad", Action::ToArad), 75),
        (("Arad", "Timisoara", Action::ToTimisoara), 118),
        (("Timisoara", "Arad", Action::ToArad), 118),
        (("Zerind", "Oradea", Action::ToOradea), 71),
        (("Oradea", "Zerind", Action::ToZerind), 71),
        (("Timisoara", "Lugoj", Action::ToLugoj), 111),
        (("Lugoj", "Timisoara", Action::ToTimisoara), 111),
        (("Lugoj", "Mehadia", Action::ToMehadia), 70),
        (("Mehadia", "Lugoj", Action::ToLugoj), 70),
        (("Mehadia", "Drobeta", Action::ToDrobeta), 75),
        (("Drobeta", "Mehadia", Action::ToMehadia), 75),
        (("Drobeta", "Craiova", Action::ToCraiova), 120),
        (("Craiova", "Drobeta", Action::ToDrobeta), 120),
        (("Oradea", "Sibiu", Action::ToSibiu), 151),
        (("Sibiu", "Oradea", Action::ToOradea), 151),
        (("Sibiu", "Fagaras", Action::ToFagaras), 99),
        (("Fagaras", "Sibiu", Action::ToSibiu), 99),
        (("Sibiu", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 80),
        (("Rimnicu Vilcea", "Sibiu", Action::ToSibiu), 80),
        (("Craiova", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 146),
        (("Rimnicu Vilcea", "Craiova", Action::ToCraiova), 146),
        (("Craiova", "Pitesti", Action::ToPitesti), 138),
        (("Pitesti", "Craiova", Action::ToCraiova), 138),
        (("Rimnicu Vilcea", "Craiova", Action::ToCraiova), 146),
        (("Craiova", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 146),
        (("Rimnicu Vilcea", "Pitesti", Action::ToPitesti), 97),
        (("Pitesti", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 97),
        (("Fagaras", "Bucharest", Action::ToBucharest), 211),
        (("Bucharest", "Fagaras", Action::ToFagaras), 211),
        (("Pitesti", "Bucharest", Action::ToBucharest), 101),
        (("Bucharest", "Pitesti", Action::ToPitesti), 101),
    ].iter().cloned().collect();

    pub static ref RESULT_STATE: HashMap<(State, Action), State> = [
        (("Arad", Action::ToSibiu), "Sibiu"),
        (("Arad", Action::ToZerind), "Zerind"),
        (("Arad", Action::ToTimisoara), "Timisoara"),
        (("Zerind", Action::ToOradea), "Oradea"),
        (("Zerind", Action::ToArad), "Arad"),
        (("Timisoara", Action::ToLugoj), "Lugoj"),
        (("Timisoara", Action::ToArad), "Arad"),
        (("Lugoj", Action::ToMehadia), "Mehadia"),
        (("Lugoj", Action::ToTimisoara), "Timisoara"),
        (("Mehadia", Action::ToDrobeta), "Drobeta"),
        (("Mehadia", Action::ToLugoj), "Lugoj"),
        (("Drobeta", Action::ToCraiova), "Craiova"),
        (("Drobeta", Action::ToMehadia), "Mehadia"),
        (("Oradea", Action::ToSibiu), "Sibiu"),
        (("Oradea", Action::ToZerind), "Zerind"),
        (("Sibiu", Action::ToFagaras), "Fagaras"),
        (("Sibiu", Action::ToRimnicuVilcea), "Rimnicu Vilcea"),
        (("Sibiu", Action::ToArad), "Arad"),
        (("Sibiu", Action::ToOradea), "Oradea"),
        (("Craiova", Action::ToRimnicuVilcea), "Rimnicu Vilcea"),
        (("Craiova", Action::ToPitesti), "Pitesti"),
        (("Craiova", Action::ToDrobeta), "Drobeta"),
        (("Rimnicu Vilcea", Action::ToCraiova), "Craiova"),
        (("Rimnicu Vilcea", Action::ToPitesti), "Pitesti"),
        (("Rimnicu Vilcea", Action::ToSibiu), "Sibiu"),
        (("Fagaras", Action::ToBucharest), "Bucharest"),
        (("Fagaras", Action::ToSibiu), "Sibiu"),
        (("Pitesti", Action::ToBucharest), "Bucharest"),
        (("Pitesti", Action::ToCraiova), "Craiova"),
        (("Pitesti", Action::ToRimnicuVilcea), "Rimnicu Vilcea"),
        (("Bucharest", Action::ToPitesti), "Pitesti"),
        (("Bucharest", Action::ToFagaras), "Fagaras")
    ].iter().cloned().collect();

    pub static ref ACTIONS: HashMap<State, Vec<Action>> = [
        ("Arad", vec![ Action::ToSibiu, Action::ToZerind, Action::ToTimisoara ]),
        ("Zerind", vec![ Action::ToOradea, Action::ToArad ]),
        ("Timisoara", vec![ Action::ToLugoj, Action::ToArad ]),
        ("Lugoj", vec![ Action::ToMehadia, Action::ToTimisoara ]),
        ("Mehadia", vec![ Action::ToDrobeta, Action::ToLugoj ]),
        ("Drobeta", vec![ Action::ToCraiova, Action::ToMehadia ]),
        ("Oradea", vec![ Action::ToSibiu, Action::ToZerind ]),
        ("Sibiu", vec![ Action::ToFagaras, Action::ToRimnicuVilcea, Action::ToArad, Action::ToOradea ]),
        ("Craiova", vec![ Action::ToRimnicuVilcea, Action::ToPitesti, Action::ToDrobeta ]),
        ("Rimnicu Vilcea", vec![ Action::ToCraiova, Action::ToPitesti, Action::ToSibiu ]),
        ("Fagaras", vec![ Action::ToBucharest, Action::ToSibiu ]),
        ("Pitesti", vec![ Action::ToBucharest, Action::ToCraiova, Action::ToRimnicuVilcea ]),
        ("Bucharest", vec![ Action::ToPitesti, Action::ToFagaras ])
    ].iter().cloned().collect();
}

pub const INITIAL_NODE: Node<State, Action> = Node::new(
    "Arad",
    None,
    None,
    0
);

pub const GOAL_NODE: Node<State, Action> = Node::new(
    "Bucharest",
    None,
    None,
    0
);

pub const ARAD_TO_BUCHAREST_PROBLEM: AradToBucharestProblem = AradToBucharestProblem {
    initial_state: "Arad",
    goal_state: "Bucharest"
};


pub fn expand<'a, P, S, A>(problem: &P, node: Node<S, A>) -> Vec<Node<S, A>> 
where
    P: Problem<S, A>,
    S: Clone,
    A: Clone
{
    let s = &node.state;
    let mut nodes: Vec<Node<S, A>> = Vec::new();
    for action in problem.actions(s) {
        let s_star = problem.result(s, &action);
        let cost = node.path_cost + problem.action_cost(s, &action, &s_star);
        nodes.push(Node::new(s_star, Some(Box::new(node.clone())), Some(action), cost));
    }

    nodes
}


/*
#[macro_export]
macro_rules! bidirectional_actions {
    ( $node1:expr, $node2:expr, $cost:expr ) => {
        paste! {
            ((stringify!($node1), [<Action::To $node2>]), stringify!($node2)),
            ((stringify!($node2), [<Action::To $node1>]), stringify!($node1))
        }
    };
}
*/
