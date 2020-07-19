import random
from typing import Set, Union, List, Dict, TextIO, Tuple, FrozenSet

from pysrc.solver_types import Assignments, Literal, Variable, Assignment, Literals, Size

__all__ = [
    'Certificate',
    'Instance',
]


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

    def __str__(self):
        pairs: List[Tuple[int, str]] = [
            (k, "True " if self.assignments[k] is Assignment.true else "False")
            if k in self.assignments else (k, 'Any  ')
            for k in range(1, max(self.assignments.keys()) + 1)
        ]
        return ' '.join((f'{k}: {v}' for k, v in pairs))

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
            literals: List[Literal],
            partial_literals: Union[List[Literal], None] = None,
    ):
        self.literals: Literals = Literals(literals)
        if partial_literals is None:
            self.partial_literals: Literals = Literals(literals)
        else:
            self.partial_literals: Literals = Literals(partial_literals)

    def size(self):
        return len(self.literals)

    def __len__(self):
        return len(self.partial_literals)

    def __contains__(self, variable: Variable) -> bool:
        return any((variable == abs(literal) for literal in self.partial_literals))

    def __str__(self):
        return ' '.join(map(str, self.literals))

    def copy(self) -> '_Clause':
        literals = [i for i in self.literals]
        partial_literals = [i for i in self.partial_literals]
        return _Clause(literals, partial_literals)

    def apply(self, certificate: Certificate) -> Union[bool, Literals]:
        partial_literals: Literals = Literals(list())
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

    @staticmethod
    def read(fp: TextIO) -> 'Instance':
        clauses: List[_Clause] = list()
        m, n = -1, -1

        line = fp.readline().split()
        while line:
            if line[0] == 'c':
                pass
            elif line[0] == 'p':
                m = int(line[2])
                n = int(line[3])
            else:
                literals = list(map(int, line[:-1]))
                clauses.append(_Clause(literals))
            line = fp.readline().split()

        instance: Instance = Instance(clauses)
        assert m >= instance.size[1]
        assert n == instance.size[2]
        return instance

    def __str__(self):
        size = f'k = {self.size[0]}, m = {self.size[1]}, n = {self.size[2]}'
        clauses = f'\n'.join(map(str, self.clauses))
        return f'\n'.join((size, clauses))

    def write(self, fp: TextIO):
        size = f'p cnf {self.size[1]} {self.size[2]}\n'
        fp.write(size)
        for clause in self.clauses:
            fp.write(str(clause) + ' 0 \n')
        return

    @staticmethod
    def generate_random(k: int, num_vars: int, num_clauses: int) -> 'Instance':
        clauses: Set[FrozenSet[Literal]] = set()

        variables = list(range(1, num_vars + 1))
        while len(clauses) < num_clauses:
            # size = k
            size = random.randint(3, k)
            signs = random.choices([1, -1], k=size)
            literals = random.sample(variables, size)
            clauses.add(frozenset((sign * literal for sign, literal in zip(signs, literals))))

        clauses: List[Literals] = [Literals(sorted(literals)) for literals in clauses]
        clauses: List[_Clause] = [_Clause([i for i in literals]) for literals in sorted(clauses)]
        return Instance(clauses)

    def apply(self, certificate: Certificate) -> Union[bool, List[_Clause]]:
        partial_clauses: List[_Clause] = list()
        for clause in self.partial_clauses:
            partial_literals: Union[bool, Literals] = clause.apply(certificate)
            if isinstance(partial_literals, bool):
                if partial_literals:
                    continue
                else:
                    return False
            else:
                literals: List[Literal] = [i for i in clause.literals]
                partial_literals: List[Literal] = [i for i in partial_literals]
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

    def _merge(self, other: 'Instance') -> Union[bool, 'Instance']:
        certificates: List[Certificate] = list()
        for left in self.certificates:
            for right in other.certificates:
                new_certificate = left.merge(right)
                if new_certificate is None:
                    continue
                else:
                    certificates.append(new_certificate)

        if len(certificates) > 0:
            clauses = [clause.copy() for clause in self.clauses]
            clauses.extend((clause.copy() for clause in other.clauses))

            partial_clauses = [clause.copy() for clause in self.partial_clauses]
            partial_clauses.extend((clause.copy() for clause in other.partial_clauses))

            return Instance(
                clauses,
                partial_clauses,
                certificates,
            )
        else:
            return False

    def solve(self) -> bool:
        leaves: List['Instance'] = [Instance([clause.copy()]) for clause in self.partial_clauses]
        for leaf in leaves:
            leaf.certificates = leaf.clauses[0].solve()

        while len(leaves) > 1:
            num_solutions = sum((leaf.num_solutions() for leaf in leaves))
            num_certificates = sum((len(leaf.certificates) for leaf in leaves))
            print(f'{len(leaves)} instances, {num_solutions} potential solutions, {num_certificates} certificates')
            new_leaves: List['Instance'] = list()
            for i, j in zip(range(0, len(leaves), 2), range(1, len(leaves), 2)):
                new_leaf = leaves[i]._merge(leaves[j])
                if isinstance(new_leaf, Instance):
                    new_leaves.append(new_leaf)
                else:
                    return False
            else:
                if len(leaves) % 2 == 1:
                    new_leaves.append(leaves[-1])
            leaves = new_leaves

        self.certificates = leaves[0].certificates
        return len(self.certificates) > 0

    def num_solutions(self) -> int:
        num_vars, num_solutions = self.size[1], 0
        return sum((2**(num_vars - len(certificate)) for certificate in self.certificates))
