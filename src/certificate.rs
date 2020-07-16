use std::collections::HashMap;
use std::result;

use crate::types::{Variable, Literal};

/// An Assignment for a variable is either True, False, or Unassigned.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Assignment {
    True = 1,
    False = -1,
    Unassigned = 0,
}

/// A certificate is represented by a HashMap of Index to Assignment,
/// where the index represents a variable.
/// A certificate may only contain True or False assignments.
#[derive(Debug, Clone)]
pub struct Certificate {
    pub assignments: HashMap<u32, Assignment>,
}

// TODO: impl Display for Certificate

impl Certificate {
    pub fn new(literal: Literal) -> Certificate {
        let mut certificate = Certificate::empty();
        certificate.insert(literal).unwrap();
        certificate
    }

    pub fn empty() -> Certificate {
        Certificate {
            assignments: HashMap::new(),
        }
    }

    pub fn from(assignments: HashMap<Variable, Assignment>) -> Certificate {
        Certificate { assignments }
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    pub fn contains_literal(&self, literal: Literal) -> bool {
        self.assignments.contains_key(&(literal.abs() as Variable))
    }

    pub fn contains_variable(&self, variable: Variable) -> bool {
        self.assignments.contains_key(&variable)
    }

    pub fn insert(&mut self, literal: Literal) -> result::Result<(), &str> {
        let variable = literal.abs() as Variable;
        let assignment = if literal > 0 {
            Assignment::True
        } else {
            Assignment::False
        };

        let &mut assignment = self.assignments.entry(variable).or_insert(assignment);
        let old_assignment = *self.assignments.get(&variable).unwrap();

        if old_assignment == assignment {
            Ok(())
        } else {
            Err("tried to insert variable with conflicting assignment")
        }
    }

    pub fn get(&self, literal: Literal) -> Assignment {
        let variable = literal.abs() as Variable;

        if self.assignments.contains_key(&variable) {
            let &value = self
                .assignments
                .get(&variable)
                .expect("this is literally impossible");
            if literal > 0 {
                value
            } else {
                // variable is negated so assignment must be reversed
                match value {
                    Assignment::True => Assignment::False,
                    Assignment::False => Assignment::True,
                    Assignment::Unassigned => {
                        panic!("unassigned variables should not be in certificate")
                    }
                }
            }
        } else {
            Assignment::Unassigned
        }
    }

    pub fn is_compatible(&self, other: &Certificate) -> bool {
        let mut common_variables: Vec<Variable> = vec![];

        for &variable in self.assignments.keys() {
            if other.contains_variable(variable) {
                common_variables.push(variable)
            } else {
                continue;
            }
        }

        for &variable in common_variables.iter() {
            let literal = variable as Literal;
            if self.get(literal) == other.get(literal) {
                continue;
            } else {
                return false;
            }
        }
        true
    }

    pub fn merge(&self, other: &Certificate) -> Option<Certificate> {
        if self.is_compatible(other) {
            let mut certificate = Certificate::from(self.assignments.clone());
            for (&k, _) in other.assignments.iter() {
                certificate.insert(k as Literal).unwrap();
            }
            Some(certificate)
        } else {
            None
        }
    }
}
