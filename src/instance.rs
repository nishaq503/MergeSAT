use std::collections::HashSet;
use std::fmt;

use crate::certificate::Certificate;
use crate::clause::{Clause, EvaluatedClause};
use crate::types::{InstanceSize, Literal, Variable};

/// An Instance is represented as a Vec of Clauses.
/// For example, the vec [C_1, C_2, C_3] represents the instance (C_1 and C_2 and C_3).
/// An Instance represents a SAT Instance and implements methods to solve a SAT instance.
/// It keeps track of the clauses that must be satisfied and all certificates that satisfy those clauses.
#[derive(Debug, Clone)]
pub struct Instance {
    clauses: Vec<Clause>,
    size: InstanceSize,
    partial_clauses: Vec<Clause>, // used for partial evaluations
    partial_size: InstanceSize,
    common_certificate: Certificate, // of assignments common to all
    certificates: Vec<Certificate>,  // collections of all satisfying certificates.
}

/// Return type from application of a certificate to an instance.
/// Instances are evaluated to True (SAT), False (UNSAT), of Undecided.
#[derive(Debug, Clone)]
pub enum EvaluatedInstance {
    True,
    False,                  // TODO: Return proof of UNSAT in standard format.
    Undecided(Vec<Clause>), // the clauses still to be satisfied
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
    /// A new instance can be created by providing the vector of clauses, and the number of variables in them.
    pub fn new(clauses: Vec<Clause>, m: usize) -> Instance {
        let k = clauses
            .iter()
            .map(|x| x.len())
            .max()
            .expect("expected max len of clauses");
        let n = clauses.len();

        Instance {
            clauses: clauses.clone(),
            size: (k, m, n),
            partial_clauses: clauses,
            partial_size: (k, m, n),
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
        let k = self
            .partial_clauses
            .iter()
            .map(|x| x.len())
            .max()
            .expect("expected max len of clauses");

        let mut variables: HashSet<Variable> = HashSet::new();
        for clause in self.partial_clauses.iter() {
            for &literal in clause.partial_literals().iter() {
                variables.insert(literal.abs() as Variable);
            }
        }

        let m = variables.len();
        let n = self.partial_clauses.len();

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
                    partial_clauses.push(Clause::new(clause.literals().clone(), partial_literals));
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

        for (&k, &v) in certificate.assignments().iter() {
            match self.common_certificate.insert_pair(k, v) {
                Ok(_) => {
                    continue;
                }
                Err(msg) => {
                    panic!(msg);
                }
            }
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

        loop {
            // cascade unit clauses and pure literals
            loop {
                // cascade unit clauses
                let mut unit_literals: HashSet<Literal> = HashSet::new();

                // find all clauses with len 1 and collect their literals.
                for clause in self.partial_clauses.iter() {
                    if clause.partial_literals().len() == 1 {
                        unit_literals.insert(clause.partial_literals()[0]);
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

            loop {
                // cascade pure literals
                // TODO: Only keep pure literals when setting them false would render the instance unsatisfiable.
                let mut pure_literals: HashSet<Literal> = HashSet::new();

                // get set of all literals
                let mut literals: HashSet<Literal> = HashSet::new();
                for clause in self.partial_clauses.iter() {
                    for &literal in clause.partial_literals().iter() {
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
