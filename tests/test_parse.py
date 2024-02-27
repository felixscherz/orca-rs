import pytest

import orca_rs


def test_parser():
    a = orca_rs.pyparse("./tests/data/matrix.out")
    print(a)
