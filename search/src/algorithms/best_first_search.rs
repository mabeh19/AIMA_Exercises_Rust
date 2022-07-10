#![feature(macro_rules)]
///
///
/// Best first search algorithm based on the following pseudo-code from AIMA
///
/// function Best-First-Search(problem, f) returns a solution node or failure
///     node <- Node(State=problem.INITIAL)
///     frontier <- a priority queue ordered by f, with node as an element
///     reached <- a lookup table, with one entry with key problem.INITIAL and value node
///
///     while not Is_Empty(frontier) do
///         node <- Pop(frontier)
///         if problem.Is_Goal(node.STATE) then return node
///         for each child in Expand(node.STATE) do
///             s <- child.STATE
///             if s is not in reached or child.PATH_COST < reached[s].PATH_COST then
///                 reached[s] <- child
///                 add child to frontier
///     return failure
///
/// function Expand(problem, node) yields nodes
///     s <- node.STATE
///     for each action in problem.ACTIONS(s) do
///         s' <- problem.RESULT(s,action)
///         cost <- node.PATH_COST + problem.ACTION_COST(s,actions,s')
///         yield Node(State=s', Parent=node, Action=action, Path_Cost=cost)
///

/* Std library */
use std::collections::HashMap;

/* Internal crates */
use crate::algorithms::node::Node;
use crate::algorithms::problem::Problem;

/* External crates */
use lazy_static::{lazy_static};

pub type Action = AradToBucharestAction;
pub type State = &'static str;

#[derive(Clone, PartialEq, PartialOrd, Eq, Hash, Debug)]
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

struct AradToBucharestProblem {
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

    fn action_cost(&self, state: &State, action: &Action, new_state: &State) -> f32 {
        *PATH_COST.get(&(state, new_state, action.clone())).unwrap()
    }
}

lazy_static! {
    static ref PATH_COST: HashMap<(State, State, Action), f32> = [
        (("Arad", "Sibiu", Action::ToSibiu), 140.),
        (("Arad", "Zerind", Action::ToZerind), 75.),
        (("Arad", "Timisoara", Action::ToTimisoara), 118.),
        (("Zerind", "Oradea", Action::ToOradea), 71.),
        (("Timisoara", "Lugoj", Action::ToLugoj), 111.),
        (("Lugoj", "Mehadia", Action::ToMehadia), 70.),
        (("Mehadia", "Drobeta", Action::ToDrobeta), 75.),
        (("Drobeta", "Craiova", Action::ToCraiova), 120.),
        (("Oradea", "Sibiu", Action::ToSibiu), 151.),
        (("Sibiu", "Fagaras", Action::ToFagaras), 99.),
        (("Sibiu", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 80.),
        (("Craiova", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 146.),
        (("Craiova", "Pitesti", Action::ToPitesti), 138.),
        (("Rimnicu Vilcea", "Craiova", Action::ToCraiova), 146.),
        (("Rimnicu Vilcea", "Pitesti", Action::ToPitesti), 97.),
        (("Fagaras", "Bucharest", Action::ToBucharest), 211.),
        (("Pitesti", "Bucharest", Action::ToBucharest), 101.)
    ].iter().cloned().collect();

    static ref RESULT_STATE: HashMap<(State, Action), State> = [
        (("Arad", Action::ToSibiu), "Sibiu"),
        (("Arad", Action::ToZerind), "Zerind"),
        (("Arad", Action::ToTimisoara), "Timisoara"),
        (("Zerind", Action::ToOradea), "Oradea"),
        (("Timisoara", Action::ToLugoj), "Lugoj"),
        (("Lugoj", Action::ToMehadia), "Mehadia"),
        (("Mehadia", Action::ToDrobeta), "Drobeta"),
        (("Drobeta", Action::ToCraiova), "Craiova"),
        (("Oradea", Action::ToSibiu), "Sibiu"),
        (("Sibiu", Action::ToFagaras), "Fagaras"),
        (("Sibiu", Action::ToRimnicuVilcea), "Rimnicu Vilcea"),
        (("Craiova", Action::ToRimnicuVilcea), "Rimnicu Vilcea"),
        (("Craiova", Action::ToPitesti), "Pitesti"),
        (("Rimnicu Vilcea", Action::ToCraiova), "Craiova"),
        (("Rimnicu Vilcea", Action::ToPitesti), "Pitesti"),
        (("Fagaras", Action::ToBucharest), "Bucharest"),
        (("Pitesti", Action::ToBucharest), "Bucharest")
    ].iter().cloned().collect();

    static ref ACTIONS: HashMap<State, Vec<Action>> = [
        ("Arad", vec![ Action::ToSibiu, Action::ToZerind, Action::ToTimisoara ]),
        ("Zerind", vec![ Action::ToOradea ]),
        ("Timisoara", vec![ Action::ToLugoj ]),
        ("Lugoj", vec![ Action::ToMehadia ]),
        ("Mehadia", vec![ Action::ToDrobeta ]),
        ("Drobeta", vec![ Action::ToCraiova ]),
        ("Oradea", vec![ Action::ToSibiu ]),
        ("Sibiu", vec![ Action::ToFagaras, Action::ToRimnicuVilcea ]),
        ("Craiova", vec![ Action::ToRimnicuVilcea, Action::ToPitesti ]),
        ("Rimnicu Vilcea", vec![ Action::ToCraiova, Action::ToPitesti ]),
        ("Fagaras", vec![ Action::ToBucharest ]),
        ("Pitesti", vec![ Action::ToBucharest ])
    ].iter().cloned().collect();
}

const INITIAL_NODE: Node<State, Action> = Node::new(
    "Arad",
    None,
    None,
    0.
);

const ARAD_TO_BUCHAREST_PROBLEM: AradToBucharestProblem = AradToBucharestProblem {
    initial_state: "Arad",
    goal_state: "Bucharest"
};

#[derive(Debug)]
pub enum SearchError {
    Failure
}

pub fn best_first_search<>() -> Result<Node<State, Action>, SearchError>  {
    
    let mut node = INITIAL_NODE;
    let mut frontier: Vec<Node<State, Action>> = vec![node.clone()];
    let mut reached: HashMap<State, Node<State, Action>> = HashMap::new(); 
    reached.insert(INITIAL_NODE.state, INITIAL_NODE);
    
    while !frontier.is_empty() {
        node = frontier.pop().unwrap();
        if ARAD_TO_BUCHAREST_PROBLEM.is_goal(&node.state) {
            return Ok(node);
        }

        for child in expand(ARAD_TO_BUCHAREST_PROBLEM, node) {
            let s = child.state;
            if !reached.contains_key(s) || child.path_cost < reached.get(s).unwrap().path_cost {
                reached.insert(s, child.clone());
                frontier.push(child.clone());
            }
        }
    }

    Err(SearchError::Failure)
}

pub fn expand<'a, P: Problem<State, Action>>(problem: P, node: Node<State, Action>) -> Vec<Node<State, Action>> {
    let s = &node.state;
    let mut nodes: Vec<Node<State, Action>> = Vec::new();
    for action in problem.actions(s) {
        let s_star = problem.result(s, &action);
        let cost = node.path_cost + problem.action_cost(s, &action, &s_star);
        nodes.push(Node::new(s_star, Some(Box::new(node.clone())), Some(action), cost));
    }

    nodes
}

