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
    fn results(&self, state: &S) -> Vec<S>;
    fn action_cost(&self, state: &S, action: &A, new_state: &S) -> f64;
    fn get_initial_node(&self) -> Node<S, A>;
    fn get_goal_node(&self) -> Node<S, A>;
    fn get_heuristic_cost(&self, state: &S) -> f64;
    fn stop(&self) -> A;
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
    ToNeamt,
    Stop
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

    fn results(&self, state: &State) -> Vec<State> {
        let mut states: Vec<State> = Vec::new();
        for a in self.actions(state) {
            states.push(self.result(state, &a));
        }

        states
    }

    fn action_cost(&self, state: &State, action: &Action, new_state: &State) -> f64 {
        *PATH_COST.get(&(state, new_state, action.clone())).unwrap()
    }

    fn get_initial_node(&self) -> Node<State, Action> {
        INITIAL_NODE
    }

    fn get_goal_node(&self) -> Node<State, Action> {
        GOAL_NODE 
    }

    fn get_heuristic_cost(&self, state: &State) -> f64 {
        *H_SLD.get(state).unwrap()
    }

    fn stop(&self) -> Action {
        return Action::Stop;
    }
}

lazy_static! {
    pub static ref PATH_COST: HashMap<(State, State, Action), f64> = [
        (("Arad", "Sibiu", Action::ToSibiu), 140.),
        (("Sibiu", "Arad", Action::ToArad), 140.),
        (("Arad", "Zerind", Action::ToZerind), 75.),
        (("Zerind", "Arad", Action::ToArad), 75.),
        (("Arad", "Timisoara", Action::ToTimisoara), 118.),
        (("Timisoara", "Arad", Action::ToArad), 118.),
        (("Zerind", "Oradea", Action::ToOradea), 71.),
        (("Oradea", "Zerind", Action::ToZerind), 71.),
        (("Timisoara", "Lugoj", Action::ToLugoj), 111.),
        (("Lugoj", "Timisoara", Action::ToTimisoara), 111.),
        (("Lugoj", "Mehadia", Action::ToMehadia), 70.),
        (("Mehadia", "Lugoj", Action::ToLugoj), 70.),
        (("Mehadia", "Drobeta", Action::ToDrobeta), 75.),
        (("Drobeta", "Mehadia", Action::ToMehadia), 75.),
        (("Drobeta", "Craiova", Action::ToCraiova), 120.),
        (("Craiova", "Drobeta", Action::ToDrobeta), 120.),
        (("Oradea", "Sibiu", Action::ToSibiu), 151.),
        (("Sibiu", "Oradea", Action::ToOradea), 151.),
        (("Sibiu", "Fagaras", Action::ToFagaras), 99.),
        (("Fagaras", "Sibiu", Action::ToSibiu), 99.),
        (("Sibiu", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 80.),
        (("Rimnicu Vilcea", "Sibiu", Action::ToSibiu), 80.),
        (("Craiova", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 146.),
        (("Rimnicu Vilcea", "Craiova", Action::ToCraiova), 146.),
        (("Craiova", "Pitesti", Action::ToPitesti), 138.),
        (("Pitesti", "Craiova", Action::ToCraiova), 138.),
        (("Rimnicu Vilcea", "Craiova", Action::ToCraiova), 146.),
        (("Craiova", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 146.),
        (("Rimnicu Vilcea", "Pitesti", Action::ToPitesti), 97.),
        (("Pitesti", "Rimnicu Vilcea", Action::ToRimnicuVilcea), 97.),
        (("Fagaras", "Bucharest", Action::ToBucharest), 211.),
        (("Bucharest", "Fagaras", Action::ToFagaras), 211.),
        (("Pitesti", "Bucharest", Action::ToBucharest), 101.),
        (("Bucharest", "Pitesti", Action::ToPitesti), 101.),
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

    pub static ref H_SLD: HashMap<State, f64> = [
        ("Arad", 366.),
        ("Bucharest", 0.),
        ("Craiova", 160.),
        ("Drobeta", 242.),
        ("Eforie", 161.),
        ("Fagaras", 176.),
        ("Giurgiu", 77.),
        ("Hirsova", 151.),
        ("Iasi", 226.),
        ("Lugoj", 244.),
        ("Mehadia", 241.),
        ("Neamt", 234.),
        ("Oradea", 380.),
        ("Pitesti", 100.),
        ("Rimnicu Vilcea", 193.),
        ("Sibiu", 253.),
        ("Timisoara", 329.),
        ("Urziceni", 80.),
        ("Vaslui", 199.),
        ("Zerind", 374.)
    ].iter().cloned().collect();
}

pub const INITIAL_NODE: Node<State, Action> = Node::new(
    "Arad",
    None,
    None,
    0.,
    366.
);

pub const GOAL_NODE: Node<State, Action> = Node::new(
    "Bucharest",
    None,
    None,
    0.,
    0.
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
        //let cost = node.path_cost + problem.action_cost(s, &action, &s_star);
        let cost = problem.action_cost(s, &action, &s_star);
        nodes.push(Node::new(s_star, Some(Box::new(node.clone())), Some(action), cost, cost));
    }

    nodes
}

pub struct GenericComplexProblem<S, A> {
    initial_node: Node<S, A>
}

pub struct GraphProblem {
    initial_node: Node<GraphState, GraphAction>,
    graph_fn: fn(f64) -> f64,
    graph_dfn: fn(f64) -> f64
}

pub const GRAPH_PROBLEM: GraphProblem = GraphProblem {
    initial_node: Node {
        state: 0.,
        parent: None,
        action: None,
        path_cost: 0.,
        f: 0.
    },
    // cos(2x * sin(x)) * 5sin(4x)+cos(3x)+sin(4x)
    graph_fn: noise_fn,
    graph_dfn: noise_dfn
};

fn noise_fn(x: f64) -> f64 {
    10. * (0.2 * x + 2.).cos()
    //(2. * x * x.sin()).cos() * 5. * (4. * x).sin() + (3. * x).cos() + (4. * x).sin()
}

fn noise_dfn(x : f64) -> f64 {
    -2. * (0.2 * x + 2.).sin()
}

fn graph_actions(state: GraphState) -> Vec<GraphAction> {
    //
    // Functions as a stepper function, can be updated to be more fancy
    //
    vec![0.1, -0.1]
}

type GraphState = f64;
type GraphAction = f64;

impl Problem<GraphState, GraphAction> for GraphProblem {
    
    fn is_goal(&self, state: &GraphState) -> bool {
        (self.graph_dfn)(*state) == 0.
    }

    fn actions(&self, state: &GraphState) -> Vec<GraphAction> {
        graph_actions(*state)
    }

    fn result(&self, state: &GraphState, action: &GraphAction) -> GraphState {
        state + action
    }

    fn results(&self, state: &GraphState) -> Vec<GraphState> {
        let mut states: Vec<GraphState> = Vec::new();
        for a in self.actions(state) {
            states.push(self.result(state, &a));
        }
        states
    }

    fn action_cost(&self, state: &GraphState, action: &GraphAction, new_state: &GraphState) -> f64 {
        (self.graph_fn)(*state + action)
    }

    fn get_initial_node(&self) -> Node<GraphState, GraphAction> {
        Node {
            state: 0.,
            parent: None,
            action: None,
            path_cost: (self.graph_fn)(0.),
            f: (self.graph_dfn)(0.)
        }
    }

    fn get_goal_node(&self) -> Node<GraphState, GraphAction> {
        //
        // Can't know final point, just that it should have derivative of 0
        Node {
            state: 0.,
            parent: None,
            action: None,
            path_cost: 0.,
            f: 0.
        }
    }

    fn get_heuristic_cost(&self, state: &GraphState) -> f64 {
        (self.graph_fn)(*state)
    }

    fn stop(&self) -> GraphAction {
        return 0.;
    }
}
