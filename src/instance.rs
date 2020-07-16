use std::collections::HashSet;
use std::fmt;

use crate::clause::{Clause, EvaluatedClause};
use crate::certificate::Certificate;

pub type Literal = i32;
pub type Variable = u32;
// TODO: Consider switching to HashSet<Literal> instead.
pub type Literals = Vec<Literal>;
pub type InstanceSize = (u32, u32, u32);

/// An Instance is represented as a Vec of Clauses.
/// For example, the vec [C_1, C_2, C_3] represents the instance (C_1 and C_2 and C_3).
/// An Instance has an associated vec of Certificates.
#[derive(Debug, Clone)]
pub struct Instance {
    clauses: Vec<Clause>,
    size: InstanceSize,
    partial_clauses: Vec<Clause>,
    partial_size: InstanceSize,
    common_certificate: Certificate,
    certificates: Vec<Certificate>,
}

/// Return type from application of a certificate to an instance.
#[derive(Debug, Clone)]
pub enum EvaluatedInstance {
    True,
    False,
    Undecided(Vec<Clause>),
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.clauses
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Instance {
    pub fn new(clauses: Vec<Clause>, num_variables: u32, num_clauses: u32) -> Instance {
        let k: u32 = clauses
            .iter()
            .map(|x| x.len())
            .max()
            .expect("expected max len of clauses") as u32;

        Instance {
            clauses: clauses.clone(),
            size: (k, num_variables, num_clauses),
            partial_clauses: clauses,
            partial_size: (k, num_variables, num_clauses),
            common_certificate: Certificate::empty(),
            certificates: vec![],
        }
    }

    pub fn from(other: &Instance) -> Instance {
        Instance {
            clauses: other.clauses.clone(),
            size: other.size,
            partial_clauses: other.partial_clauses.clone(),
            partial_size: other.partial_size,
            common_certificate: other.common_certificate.clone(),
            certificates: other.certificates.clone(),
        }
    }

    pub fn recalculate_partial_size(&mut self) {
        let k: u32 = self
            .partial_clauses
            .iter()
            .map(|x| x.len())
            .max()
            .expect("expected max len of clauses") as u32;

        let mut variables: HashSet<Variable> = HashSet::new();
        for clause in self.partial_clauses.iter() {
            for &literal in clause.partial_literals.iter() {
                variables.insert(literal.abs() as Variable);
            }
        }

        let m: u32 = variables.len() as u32;
        let n: u32 = self.partial_clauses.len() as u32;

        self.partial_size = (k, m, n);
    }

    pub fn apply(&self, certificate: &Certificate) -> EvaluatedInstance {
        // store evaluated clauses
        let mut partial_clauses: Vec<Clause> = vec![];

        // evaluate each partial clause
        for clause in self.partial_clauses.iter() {
            match clause.apply(certificate) {
                EvaluatedClause::True => {
                    // clauses that evaluate to True can be dropped from the instance
                    continue;
                }
                EvaluatedClause::False => {
                    // a single clause evaluating to False causes the instance to be judged False
                    return EvaluatedInstance::False;
                }
                EvaluatedClause::Undecided(partial_literals) => {
                    // clause was not evaluated either way so we keep it around.
                    partial_clauses.push(Clause {
                        literals: clause.literals.clone(),
                        partial_literals,
                    });
                }
            }
        }
        if partial_clauses.is_empty() {
            // all clauses evaluated to True, so the instance is satisfied
            EvaluatedInstance::True
        } else {
            // some clauses are still not satisfied
            EvaluatedInstance::Undecided(partial_clauses)
        }
    }

    fn extend_common(&mut self, literals: HashSet<Literal>) -> EvaluatedInstance {
        let mut certificate = Certificate::empty();
        for &literal in literals.iter() {
            match certificate.insert(literal) {
                Ok(()) => {
                    continue;
                }
                Err(_) => {
                    return EvaluatedInstance::False;
                }
            }
        }

        for (&k, &v) in certificate.assignments.iter() {
            self.common_certificate.assignments.insert(k, v);
        }

        match self.apply(&certificate) {
            EvaluatedInstance::True => EvaluatedInstance::True,
            EvaluatedInstance::False => EvaluatedInstance::False,
            EvaluatedInstance::Undecided(partial_clauses) => {
                self.partial_clauses = partial_clauses;
                EvaluatedInstance::Undecided(self.partial_clauses.clone())
            }
        }
    }

    pub fn cascade(&mut self) -> EvaluatedInstance {
        let mut num_clauses = self.partial_clauses.len();

        loop {  // cascade unit clauses and pure literals
            loop {  // cascade unit clauses
                let mut unit_literals: HashSet<Literal> = HashSet::new();

                // find all clauses with len 1 and collect their literals.
                for clause in self.partial_clauses.iter() {
                    if clause.partial_literals.len() == 1 {
                        unit_literals.insert(clause.partial_literals[0]);
                    } else {
                        continue;
                    }
                }
                if unit_literals.is_empty() {
                    break;
                } else {
                    self.extend_common(unit_literals);
                }
            }

            loop {  // cascade pure literals
                let mut pure_literals: HashSet<Literal> = HashSet::new();

                // get set of all literals
                let mut literals: HashSet<Literal> = HashSet::new();
                for clause in self.partial_clauses.iter() {
                    for &literal in clause.partial_literals.iter() {
                        literals.insert(literal);
                    }
                }
                // find every literal whose negation is not present in the set of all literals
                for &literal in literals.iter() {
                    if literals.contains(&(-literal)) {
                        continue;
                    } else {
                        pure_literals.insert(literal);
                    }
                }
                if pure_literals.is_empty() {
                    break;
                } else {
                    self.extend_common(pure_literals);
                }
            }

            // if any clauses were removed, the cascade might continue
            if self.partial_clauses.len() == num_clauses {
                break;
            } else {
                num_clauses = self.partial_clauses.len();
                continue;
            }
        }
        if self.partial_clauses.is_empty() {
            EvaluatedInstance::True
        } else {
            EvaluatedInstance::Undecided(self.partial_clauses.clone())
        }
    }

    // pub fn partition(&self) -> Instance {}
    // pub fn merge(&self) -> Instance {}
    // pub fn solve(&self) -> EvaluatedInstance {}
}
