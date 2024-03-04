import orca_rs


def test_parse_tddft():
    a = orca_rs.parse_tddft("./tests/data/ch2o_soc_tddft.out")
    print(a)

def test_parse_casscf():
    a = orca_rs.parse_casscf("./tests/data/product_sa_casscf_v8_5a897afa.out")
    print(a)
