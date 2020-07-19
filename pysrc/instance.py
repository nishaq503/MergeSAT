from typing import Set, Union

import numpy as np
from pysrc.types import Assignments, Literal, Variable, Assignment


class Certificate:
    def __init__(self, assignments: Union[Assignments, None] = None):
        self.assignments: Assignments = dict() if assignments is None else assignments

    def __len__(self):
        return len(self.assignments)

    def __bool__(self):
        return len(self.assignments) != 0

    def __contains__(self, literal: Literal):
        return np.abs(literal) in self.assignments

    def insert_pair(self, variable: Variable, assignment: Assignment):
        if variable not in self.assignments:
            self.assignments[variable] = assignment

        old_assignment: Assignment = self.assignments[variable]

        if assignment == old_assignment:
            return
        else:
            raise ValueError(f"tried to insert a conflicting assignment: {variable} was assigned {old_assignment}")

    def insert(self, literal: Literal):
        variable: Variable = np.abs(literal)
        assignment: Assignment = Assignment.true if literal > 0 else Assignment.false
        return self.insert_pair(variable, assignment)

    def get(self, literal: Literal) -> Assignment:
        variable: Variable = np.abs(literal)

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
