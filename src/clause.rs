use std::fmt;

use crate::types::{Variable, Literals};
use crate::certificate::{Certificate, Assignment};

/// A Clause is represented as a Vec of integers.
/// Each integer value represents a variable, the sign represents whether that variable is negated.
/// For example, the vec [3, -1, 4] represents the clause (x_3 or (not x_1) or x_4).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clause {
    pub literals: Literals,
    pub partial_literals: Literals,
}

/// Clauses can be evaluated using certificates.
#[derive(Debug, Clone)]
pub enum EvaluatedClause {
    True,
    False,
    Undecided(Literals),
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.literals
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Clause {
    pub fn new(literals: Literals) -> Clause {
        Clause {
            literals: literals.clone(),
            partial_literals: literals,
        }
    }

    pub fn size(&self) -> usize {
        self.literals.len()
    }

    pub fn len(&self) -> usize {
        self.partial_literals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.partial_literals.is_empty()
    }

    pub fn contains(&self, variable: Variable) -> bool {
        self.partial_literals
            .iter()
            .any(|&literal| (literal.abs() as Variable) == variable)
    }

    pub fn apply(&self, certificate: &Certificate) -> EvaluatedClause {
        let mut partial_literals: Literals = vec![];
        for &literal in self.partial_literals.iter() {
            match certificate.get(literal) {
                Assignment::True => {
                    return EvaluatedClause::True;
                }
                Assignment::False => {
                    continue;
                }
                Assignment::Unassigned => {
                    partial_literals.push(literal);
                }
            }
        }
        EvaluatedClause::Undecided(partial_literals)
    }

    pub fn solve(&self) -> Vec<Certificate> {
        let mut certificates: Vec<Certificate> = vec![];
        for &literal in self.partial_literals.iter() {
            certificates.push(Certificate::new(literal));
        }
        certificates
    }
}
