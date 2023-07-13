use std::{
    collections::{HashMap, VecDeque},
    hash::Hash
};

type Domain<T> = Vec<T>;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ConstraintType {
    Unary,
    Binary,
    Path
}

pub trait Constraint<T> 
where
    T: PartialEq
{
    type FirstVariable;
    type OtherVariables;

    fn is_satisfied(&self, one: &Self::FirstVariable, others: &Self::OtherVariables) -> bool;
    fn new(rtype: RuleType) -> Self;
}

pub struct UnaryConstraint {
    rule: Rule 
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct BinaryConstraint {
    rule: Rule
}

pub struct PathConstraint {
    rule: Rule
}

impl<T> Constraint<T> for UnaryConstraint
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug
{
    type FirstVariable = T;
    type OtherVariables = Domain<T>;

    fn is_satisfied(&self, one: &Self::FirstVariable, others: &Self::OtherVariables) -> bool {
        self.rule.check_set(one, others)
    }

    fn new(rtype: RuleType) -> Self {
        Self { rule: Rule::new(rtype) }
    }
}

impl<T> Constraint<T> for BinaryConstraint
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug
{
    type FirstVariable = T;
    type OtherVariables = T;

    fn is_satisfied(&self, one: &Self::FirstVariable, others: &Self::OtherVariables) -> bool {
        self.rule.check_num(one, others)
    }

    fn new(rtype: RuleType) -> Self {
        Self { rule: Rule::new(rtype) }
    }
}

impl<T> Constraint<T> for PathConstraint
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug
{
    type FirstVariable = T;
    type OtherVariables = Vec<T>;

    fn is_satisfied(&self, one: &Self::FirstVariable, others: &Self::OtherVariables) -> bool {
        self.rule.check_set(one, others)
    }

    fn new(rtype: RuleType) -> Self {
        Self { rule: Rule::new(rtype) }
    }
}

#[derive(Clone, PartialEq, std::fmt::Debug)]
pub struct Variable<T> 
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug
{
    name: String,
    domain: Domain<T>,
    value: T,
    neighbors: Vec<String>,
}

impl<T> Variable<T>
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug
{
    pub fn new(name: &str, value: T, domain: Domain<T>, neighbors: Option<Vec<String>>) -> Self {
        let neighbors = if neighbors.is_some() { neighbors.unwrap() } else { Vec::new() };
        Self {
            name: name.to_owned(), 
            domain,
            value,
            neighbors
        }
    }

    pub fn get_domain(&self) -> Domain<T> {
        self.domain.clone()
    }

    pub fn set_value(&mut self, value: &T) {
        self.value = value.clone();
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_domain_as_mut(&mut self) -> &mut Domain<T> {
        &mut self.domain
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_neighbors(&self) -> Vec<String> {
        self.neighbors.clone()
    }
}


#[derive(Clone, PartialEq)]
pub struct CSP<T> 
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug,
//    C: Constraint<T> + PartialEq + PartialOrd
{
    variables: HashMap<String, Variable<T>>,
    constraints: HashMap<(String, String), VecDeque<BinaryConstraint>>
}

impl<T> CSP<T>
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug,
//    C: Constraint<T> + PartialEq + PartialOrd
{
    pub fn new() -> Self {
        Self { variables: HashMap::new(), constraints: HashMap::new() }
    }

    pub fn add_variable(&mut self, name: &str, start_value: T, domain: Domain<T>, neighbors: Option<Vec<String>>) {
        self.variables.insert(name.to_owned(), Variable::new(name, start_value, domain, neighbors));
    }

    pub fn get_variables(&self) -> &HashMap<String, Variable<T>> {
        &self.variables
    }

    pub fn get_arcs(&self) -> VecDeque<(String, String)> {
        self.constraints.keys().map(|k| k.clone()).collect()
    }

    pub fn satisfies_constraint(&self, x_1: &str, x_2: &str, x: &T, y: &T) -> bool {
        self.constraints.get(&(x_1.to_owned(), x_2.to_owned())).unwrap()[0].is_satisfied(x, y)
    }

    pub fn add_constraint(&mut self, x_1: &str, x_2: &str, _ctype: ConstraintType, rtype: RuleType) {
        if !self.constraints.contains_key(&(x_1.to_owned(), x_2.to_owned())) {
            self.constraints.insert((x_1.to_owned(), x_2.to_owned()), VecDeque::new());
        }
        let c = <BinaryConstraint as Constraint<T>>::new(rtype);
        self.constraints.get_mut(&(x_1.to_owned(), x_2.to_owned())).unwrap().push_back( c );
    }

    pub fn get_variable(&self, name: &str) -> &Variable<T> {
        self.variables.get(name).unwrap()
    }

    pub fn get_variable_as_mut(&mut self, name: &str) -> &mut Variable<T> {
        self.variables.get_mut(name).unwrap()
    }

    pub fn set_domain(&mut self, name: &str, domain: Domain<T>) {
        self.variables.get_mut(name).unwrap().domain = domain;
    }

    pub fn assignment_complete(&self) -> bool
    {
        let mut is_assignment_complete: bool = true;
        
        for var in self.get_variables() {
            if var.1.get_domain().len() != 1 || 
                self.get_num_conflicts(var.0) > 0 {
                is_assignment_complete = false;
                break;
            }
        }

        return is_assignment_complete;
    }

    pub fn get_num_conflicts(&self, name: &str) -> u32 {
        let mut conflicts = 0;
        let var = self.get_variable(name);
        for n in &var.neighbors {
            for c in self.constraints.get(&(name.to_string(), n.to_string())).unwrap() {
                for o in self.get_variable(&n).get_domain() {
                    if !c.is_satisfied(&var.domain[0], &o) {
                        conflicts += 1;
                    }
                }
            }
        }
        conflicts
    }
}

impl<T> std::fmt::Display for CSP<T> 
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variables: \n")?;
        for var in &self.variables {
            write!(f, "     {} = {:?}\n", var.0, var.1.get_domain())?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Rule {
    rtype: RuleType
}

impl Rule {
    pub fn new(rtype: RuleType) -> Self {
        Self { rtype }
    }

    pub fn check_num<T>(&self, a: &T, b: &T) -> bool 
    where
        T: PartialEq + PartialOrd
    {
        match self.rtype {
            RuleType::EqualTo => a == b,
            RuleType::NotEqualTo => a != b,
            RuleType::LesserThan => a < b,
            RuleType::GreaterThan => a < b,
            _ => false
        }
    }

    pub fn check_set<T>(&self, a: &T, b: &Vec<T>) -> bool
    where
        T: PartialEq + PartialOrd
    {
        match self.rtype {
            RuleType::IsIn => b.contains(&a),
            RuleType::IsNotIn => !b.contains(&a),
            _ => false
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum RuleType {
    EqualTo,
    NotEqualTo,
    IsIn,
    IsNotIn,
    LesserThan,
    GreaterThan,
}

#[macro_export]
macro_rules! alldiff2binary {
    ( $csp:tt, $vars:tt ) => {
        for x_i in &$vars {
            for x_j in &$vars {
                if ( x_i != x_j ) {
                    $csp.add_constraint(&x_i, &x_j, constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
                }
            }
        }
    }
}
