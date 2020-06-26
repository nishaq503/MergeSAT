use std::fmt::{Display, Formatter, Result};

/// A Clause is represented as a Vec of integers.
/// Each integer value represents a variable, and the sign represents whether a variable is negated.
/// For example, the vec [3, -1, 4] represents the clause (x_3 or (not x_1) or x_4).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clause {
    literals: Vec<i64>,
    partial_literals: Vec<i64>,
}

/// Return type from application of a certificate to a clause.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EvaluatedClause {
    True,
    False,
    Undecided(Clause),
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Clause {
    pub fn new(literals: Vec<i64>) -> Clause {
        Clause {
            literals: literals.clone(),
            partial_literals: literals.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        self.literals
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn len(&self) -> usize {
        self.partial_literals.len()
    }

    pub fn contains(&self, variable: u64) -> bool {
        if self
            .partial_literals
            .iter()
            .any(|&literal| (literal.abs() as u64) == variable)
        {
            true
        } else {
            false
        }
    }
}
