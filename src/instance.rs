use crate::certificate::Certificate;
use crate::clause::Clause;

/// An Instance is represented as a Vec of Clauses.
/// For example, the vec [C_1, C_2, C_3] represents the instance (C_1 and C_2 and C_3).
/// An Instance has an associated vec of Certificates.
#[derive(Debug, Clone)]
pub struct Instance {
    clauses: Vec<Clause>,
    size: (u64, u64, u64),
    partial_clauses: Vec<Clause>,
    partial_size: (u64, u64, u64),
    common_certificate: Certificate,
    certificates: Vec<Certificate>,
}

/// Return type from application of a certificate to an instance.
#[derive(Debug, Clone)]
pub enum EvaluatedInstance {
    True,
    False,
    Undecided(Instance),
}

impl Instance {
    pub fn new(clauses: Vec<Clause>) -> Instance {
        Instance {
            clauses: clauses.clone(),
            size: (0, 0, 0),
            partial_clauses: clauses.clone(),
            partial_size: (0, 0, 0),
            common_certificate: Certificate::empty(),
            certificates: vec![],
        }
    }

    pub fn from(other: &Instance) -> Instance {
        Instance {
            clauses: other.clauses.clone(),
            size: other.size.clone(),
            partial_clauses: other.partial_clauses.clone(),
            partial_size: other.partial_size.clone(),
            common_certificate: other.common_certificate.clone(),
            certificates: other.certificates.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        self.partial_clauses
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn size(&self) -> (u64, u64, u64) {
        self.partial_size
    }
}
