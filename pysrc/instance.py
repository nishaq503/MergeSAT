from typing import Set, Union, List, Dict

from pysrc.types import Assignments, Literal, Variable, Assignment, Literals, Size


class Certificate:
    def __init__(self, assignments: Union[Dict[Variable, Assignment], None] = None):
        if assignments is None:
            assignments = dict()

        if any((k <= 0 for k in assignments.keys())):
            raise ValueError(f"assignments can only be made for variables, "
                             f"and all variables must be positive integers.")
        self.assignments: Dict[Variable, Assignment] = {
            k: v for k, v in assignments.items()
            if v is not Assignment.unassigned
        }

    def __len__(self):
        return len(self.assignments)

    def __bool__(self):
        return len(self.assignments) != 0

    def __contains__(self, literal: Literal):
        return abs(literal) in self.assignments

    def copy(self) -> 'Certificate':
        assignments: Dict[Variable, Assignment] = {k: v for k, v in self.assignments.items()}
        return Certificate(assignments)

    def insert_pair(self, variable: Variable, assignment: Assignment):
        if not variable > 0:
            raise ValueError(f"Variables must be positive integers.")
        if assignment is Assignment.unassigned:
            return

        if variable not in self.assignments:
            self.assignments[variable] = assignment
        old_assignment: Assignment = self.assignments[variable]

        if assignment == old_assignment:
            return
        else:
            raise ValueError(f"tried to insert a conflicting assignment: "
                             f"{variable} was assigned {old_assignment}")

    def insert(self, literal: Literal):
        if literal == 0:
            raise ValueError(f"0 is not a valid literal.")

        variable: Variable = abs(literal)
        assignment: Assignment = Assignment.true if literal > 0 else Assignment.false
        return self.insert_pair(variable, assignment)

    def get(self, literal: Literal) -> Assignment:
        if literal == 0:
            raise ValueError(f"0 is not a valid literal.")

        variable: Variable = abs(literal)

        if variable in self.assignments:
            assignment: Assignment = self.assignments[variable]
            return assignment if literal > 0 else assignment.inverse
        else:
            return Assignment.unassigned

    def is_compatible(self, other: 'Certificate') -> bool:
        common_variables: Set[Variable] = set(self.assignments.keys()).intersection(set(other.assignments.keys()))

        for variable in common_variables:
            if self.get(variable) != other.get(variable):
                return False
            else:
                continue
        else:
            return True

    def merge(self, other: 'Certificate') -> Union['Certificate', None]:
        if self.is_compatible(other):
            assignments: Assignments = {k: v for k, v in self.assignments.items()}
            assignments.update(other.assignments)
            return Certificate(assignments)
        else:
            return None


class _Clause:
    def __init__(
            self,
            literals: Literals,
            partial_literals: Union[Literals, None] = None,
    ):
        self.literals: Literals = literals
        if partial_literals is None:
            self.partial_literals: Literals = [i for i in literals]
        else:
            self.partial_literals: Literals = partial_literals

    def size(self):
        return len(self.literals)

    def __len__(self):
        return len(self.partial_literals)

    def __contains__(self, variable: Variable) -> bool:
        return any((variable == abs(literal) for literal in self.partial_literals))

    def copy(self) -> '_Clause':
        literals = [i for i in self.literals]
        partial_literals = [i for i in self.partial_literals]
        return _Clause(literals, partial_literals)

    def apply(self, certificate: Certificate) -> Union[bool, Literals]:
        partial_literals: Literals = list()
        for literal in self.partial_literals:
            assignment: Assignment = certificate.get(literal)
            if assignment is Assignment.true:
                return True
            elif assignment is Assignment.false:
                continue
            else:
                partial_literals.append(literal)
        return partial_literals

    def solve(self) -> List[Certificate]:
        certificates: List[Certificate] = list()
        for literal in self.partial_literals:
            certificate: Certificate = Certificate()
            certificate.insert(literal)
            certificates.append(certificate)
        return certificates


class Instance:
    def __init__(
            self,
            clauses: List[_Clause],
            partial_clauses: Union[List[_Clause], None] = None,
            certificates: Union[List[Certificate], None] = None,
    ):

        self.clauses: List[_Clause] = clauses

        if partial_clauses is None:
            self.partial_clauses: List[_Clause] = [clause.copy() for clause in clauses]
        else:
            self.partial_clauses: List[_Clause] = partial_clauses

        # self.common_certificate: Certificate = Certificate()
        if certificates is None:
            self.certificates: List[Certificate] = list()
        else:
            self.certificates: List[Certificate] = certificates

        self.partial_size: Size = (-1, -1, -1)
        self.recalculate_partial_size()
        self.size: Size = (self.partial_size[0], self.partial_size[1], self.partial_size[2])

        self.left: Union['Instance', None] = None
        self.right: Union['Instance', None] = None

    def recalculate_partial_size(self) -> None:
        k = max(clause.size() for clause in self.clauses)

        variables = set()
        [variables.update(set(map(abs, clause.literals))) for clause in self.clauses]
        m = len(variables)

        n = len(self.clauses)
        self.partial_size = (k, m, n)
        return

    def apply(self, certificate: Certificate) -> Union[bool, List[_Clause]]:
        partial_clauses: List[_Clause] = list()
        for clause in self.partial_clauses:
            partial_literals = clause.apply(certificate)
            if isinstance(partial_literals, bool):
                if partial_literals:
                    continue
                else:
                    return False
            else:
                literals: Literals = [i for i in clause.literals]
                partial_clauses.append(_Clause(literals, partial_literals))
        if len(partial_clauses) > 0:
            return partial_clauses
        else:
            return True

    def _extend_common(self, literals: Literals) -> Union[bool, List[_Clause]]:
        raise NotImplementedError

    def _cascade(self) -> Union[bool, List[_Clause]]:
        raise NotImplementedError

    def _partition(self) -> None:
        if len(self.partial_clauses) > 1:
            half = len(self.partial_clauses) // 2

            self.left = Instance(self.partial_clauses[:half])
            self.right = Instance(self.partial_clauses[half:])
        else:
            return

    def _merge(self, other: 'Instance') -> 'Instance':
        clauses = [clause.copy() for clause in self.clauses]
        clauses.extend((clause.copy() for clause in other.clauses))

        partial_clauses = [clause.copy() for clause in self.partial_clauses]
        partial_clauses.extend((clause.copy() for clause in other.partial_clauses))

        certificates: List[Certificate] = list()
        for left in self.certificates:
            for right in other.certificates:
                new_certificate = left.merge(right)
                if new_certificate is None:
                    continue
                else:
                    certificates.append(new_certificate)

        return Instance(
            clauses,
            partial_clauses,
            certificates,
        )

    def solve(self) -> bool:
        pass
