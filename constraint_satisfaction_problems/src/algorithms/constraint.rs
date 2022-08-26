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

pub trait Constraint<T, C> 
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd 
{
    type OtherVariables;

    fn is_satisfied(&self, one: &Variable<T, C>, others: &Self::OtherVariables) -> bool;
    fn new(rtype: RuleType) -> Self;
}

pub struct UnaryConstraint {
    rule: Rule 
}

pub struct BinaryConstraint {
    rule: Rule
}

pub struct PathConstraint {
    rule: Rule
}

impl<T, C> Constraint<T, C> for UnaryConstraint
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    type OtherVariables = Domain<T>;

    fn is_satisfied(&self, one: &Variable<T, C>, others: &Self::OtherVariables) -> bool {
        self.rule.check_set(&one.value, others)
    }

    fn new(rtype: RuleType) -> Self {
        Self { rule: Rule::new(rtype) }
    }
}

impl<T, C> Constraint<T, C> for BinaryConstraint
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    type OtherVariables = Variable<T, C>;

    fn is_satisfied(&self, one: &Variable<T, C>, others: &Self::OtherVariables) -> bool {
        self.rule.check_num(&one.value, &others.value)
    }

    fn new(rtype: RuleType) -> Self {
        Self { rule: Rule::new(rtype) }
    }
}

impl<T, C> Constraint<T, C> for PathConstraint
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    type OtherVariables = Vec<T>;

    fn is_satisfied(&self, one: &Variable<T, C>, others: &Self::OtherVariables) -> bool {
        self.rule.check_set(&one.value, &others)
    }

    fn new(rtype: RuleType) -> Self {
        Self { rule: Rule::new(rtype) }
    }
}

#[derive(Clone, PartialEq)]
pub struct Variable<T, C> 
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    domain: Domain<T>,
    value: T,
    constraints: HashMap<String, VecDeque<C>>,
}

impl<T, C> Variable<T, C>
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    pub fn new(value: T, domain: Domain<T>, constraints: Option<HashMap<String, VecDeque<C>>>) -> Self {
        let hashmap = if constraints.is_some() { constraints.unwrap() } else { HashMap::new() };
        Self {
            domain,
            value,
            constraints: hashmap
        }
    }

    pub fn add_constraint(&mut self, other: &str, ctype: ConstraintType, rtype: RuleType) {
        if !self.constraints.contains_key(&other.to_owned()) {
            self.constraints.insert(other.to_owned(), VecDeque::new());
        }

        let mut c = self.constraints.get_mut(&other.to_owned()).unwrap().push_back( C::new(rtype) );
    }

    pub fn get_neighbors(&self) -> Vec<String> {

    }
}


#[derive(Clone, PartialEq)]
pub struct CSP<T, C> 
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    variables: HashMap<String, Variable<T, C>>
}

impl<T, C> CSP<T, C>
where
    T: Clone + PartialEq + PartialOrd + Eq + Hash,
    C: Constraint<T, C> + PartialEq + PartialOrd
{
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    pub fn add_variable(&mut self, name: &str, start_value: T, domain: Domain<T>, constraints: Option<HashMap<String, VecDeque<C>>>) {
        self.variables.insert(name.to_owned(), Variable::new(start_value, domain, constraints));
    }

    pub fn add_constraint(&mut self, variable: &str, other_variable: &str, ctype: ConstraintType, rtype: RuleType) {
        self.variables.get_mut(variable).unwrap().add_constraint(other_variable, ctype, rtype);
    }

    pub fn get_arcs(&self) -> &HashMap<String, Variable<T, C>> {
        &self.variables
    }

    pub fn satisfies_constraint(&self) -> bool {
        false
    }
}

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

