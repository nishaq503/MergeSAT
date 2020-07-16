use std::collections::HashMap;

/// variable
pub type Variable = u32;

/// literal
pub type Literal = i32;

/// literals
// TODO: Consider switching to HashSet<Literal> instead.
pub type Literals = Vec<Literal>;

/// instance size
pub type InstanceSize = (usize, usize, usize);

/// An Assignment for a variable is either True, False, or Unassigned.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Assignment {
    True = 1,
    False = -1,
    Unassigned = 0,
}

/// assignments
pub type Assignments = HashMap<Variable, Assignment>;
