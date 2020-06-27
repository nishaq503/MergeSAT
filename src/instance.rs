use std::collections::HashMap;
use std::collections::HashSet;
use std::{fmt, result};

type Literal = i32;
type Variable = u32;
// TODO: Consider switching to HashSet<Literal> instead.
type Literals = Vec<Literal>;
type InstanceSize = (u32, u32, u32);

/// An Assignment for a variable is either True, False, or Unassigned.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Assignment {
    True,
    False,
    Unassigned,
}

/// A certificate is represented by a HashMap of Index to Assignment,
/// where the index represents a variable.
/// A certificate may only contain True or False assignments.
#[derive(Debug, Clone)]
pub struct Certificate {
    assignments: HashMap<u32, Assignment>,
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

    pub fn insert(&mut self, literal: Literal) -> result::Result<(), &str> {
        let variable = literal.abs() as Variable;
        let assignment = if literal > 0 {
            Assignment::True
        } else {
            Assignment::False
        };

        if self.assignments.contains_key(&variable) {
            let &old_assignment = self.assignments.get(&variable).unwrap();
            if old_assignment == assignment {
                Ok(())
            } else {
                Err("tried to insert variable with conflicting assignment")
            }
        } else {
            self.assignments.insert(variable, assignment);
            Ok(())
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
}

/// A Clause is represented as a Vec of integers.
/// Each integer value represents a variable, the sign represents whether that variable is negated.
/// For example, the vec [3, -1, 4] represents the clause (x_3 or (not x_1) or x_4).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clause {
    literals: Literals,
    partial_literals: Literals,
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

    // pub fn partition(&self) -> Instance {}
    // pub fn solve(&self) -> EvaluatedInstance {}
}
