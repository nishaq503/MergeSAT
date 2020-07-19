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
    def __init__(self, literals: Literals, partial_literals: Literals):
        self.literals: Literals = literals
        self.partial_literals: Literals = partial_literals

    def size(self):
        return len(self.literals)

    def __len__(self):
        return len(self.partial_literals)

    def __contains__(self, variable: Variable) -> bool:
        return any((variable == abs(literal) for literal in self.partial_literals))

    def copy(self) -> '_Clause':
        literals = [literal for literal in self.literals]
        partial_literals = [literal for literal in self.partial_literals]
        return _Clause(literals, partial_literals)

    def apply(self, certificate: Certificate) -> Union[bool, Literals]:
        pass

    def solve(self) -> List[Certificate]:
        pass


class Instance:
    def __init__(self, clauses: List[_Clause]):
        k = max(clause.size() for clause in clauses)

        variables = set()
        [variables.update(set(map(abs, clause.literals))) for clause in clauses]
        m = len(variables)

        n = len(clauses)

        self.clauses: List[_Clause] = [clause.copy() for clause in clauses]
        self.size: Size = (k, m, n)

        self.partial_clauses: List[_Clause] = [clause.copy() for clause in clauses]
        self.partial_size: Size = (k, m, n)

        self.common_certificate: Certificate = Certificate()
        self.certificates: List[Certificate] = list()

    def recalculate_partial_size(self) -> None:
        pass

    def apply(self, certificate: Certificate) -> Union[bool, List[_Clause]]:
        pass

    def _extend_common(self, literals: Literals) -> Union[bool, List[_Clause]]:
        pass

    def _cascade(self) -> Union[bool, List[_Clause]]:
        pass

    def _partition(self) -> None:
        pass

    def _merge(self) -> 'Instance':
        pass

    def solve(self) -> Union[bool, List[_Clause]]:
        pass
