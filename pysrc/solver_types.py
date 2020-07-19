from enum import Enum
from typing import List, Tuple, Dict

Variable = int
Literal = int


class Literals:
    def __init__(self, literals: List[Literal]):
        self.literals = [i for i in literals]

    def __lt__(self, other: 'Literals'):
        left = list(sorted([abs(i) for i in self.literals]))
        right = list(sorted([abs(i) for i in other.literals]))
        return left < right

    def __len__(self):
        return len(self.literals)

    def __iter__(self):
        yield from self.literals

    def __hash__(self):
        hash(self.literals)

    def append(self, literal: Literal):
        self.literals.append(literal)


Size = Tuple[int, int, int]


class Assignment(Enum):
    true = 1
    false = 0
    unassigned = -1

    def __lt__(self, other: 'Assignment') -> bool:
        return self > other

    @property
    def inverse(self) -> 'Assignment':
        if self is Assignment.true:
            return Assignment.false
        elif self is Assignment.false:
            return Assignment.true
        else:
            return Assignment.unassigned


Assignments = Dict[Variable, Assignment]
