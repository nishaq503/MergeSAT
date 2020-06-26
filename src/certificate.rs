use std::collections::HashMap;

/// An Assignment for a variable is either True, False, or Unassigned.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Assignment {
    True,
    False,
    Unassigned,
}

/// A certificate is represented by a HashMap of Index to Assignment,
/// where the index represents a variable.
/// A certificate knows which Instance it is associated with.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Certificate {
    assignments: HashMap<u64, Assignment>,
}

impl Certificate {
    pub fn new(assignments: HashMap<u64, Assignment>) -> Certificate {
        Certificate { assignments }
    }

    pub fn empty() -> Certificate {
        Certificate {
            assignments: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn insert(&mut self, variable: u64, assignment: Assignment) -> () {
        self.assignments.insert(variable, assignment);
    }
}
