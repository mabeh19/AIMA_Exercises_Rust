
pub trait Problem<State, Action> 
where State: Clone,
      Action: Clone
{
    //type State;
    //type Action;
    fn is_goal(&self, state: &State) -> bool;
    fn actions(&self, state: &State) -> Vec<Action>;
    fn result(&self, state: &State, action: &Action) -> State;
    fn action_cost(&self, state: &State, action: &Action, new_state: &State) -> f32;
}
