from typing import TypedDict

class SOCElement(TypedDict):
    weight: float
    real: float
    imag: float
    root: int
    spin: int
    ms: int

class SOCEigenvector(TypedDict):
    n: int
    energy: float
    elements: list[SOCElement]

class SOCMatrix(TypedDict):
    eigenvectors: list[SOCEigenvector]

class OrcaOutput(TypedDict):
    soc_matrix: SOCMatrix

def parse(filename: str) -> OrcaOutput:
    """Parse orca output file"""
