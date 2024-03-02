import orca_rs


def test_parser():
    a = orca_rs.parse_tddft("./tests/data/ch2o_soc_tddft.out")
    print(a)
