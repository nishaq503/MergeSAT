from enum import Enum
from typing import List, Tuple, Dict

import numpy as np

Variable = np.uint32
Literal = np.int32
Literals = List[Literal]
Size = Tuple[np.uint64, np.uint64, np.uint64]


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
