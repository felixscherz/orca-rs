import pytest

import orca_rs


def test_parser():
    a = orca_rs.parse("./tests/data/matrix.out")
    print(a)
