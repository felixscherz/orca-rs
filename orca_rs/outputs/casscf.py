from typing import TypedDict

class Contribution(TypedDict):
    atom_nr: int
    element: str
    orbital_type: str
    contribution: float

class Orbital(TypedDict):
    orbital_nr: int
    energy: float
    occupation: float
    contributions: list[Contribution]


class CASSCFOutput(TypedDict):
    orbital_contributions: list[Orbital]
