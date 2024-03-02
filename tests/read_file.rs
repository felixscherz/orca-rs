use std::{fs, path::Path};

#[cfg(test)]
mod tests {
    use orca_rs::parsers::{parse_tddft, parse_casscf};

    use super::*;
    #[test]
    fn test_parse_tddft() {
        let filename = "tests/data/ch2o_soc_tddft.out";
        let content = fs::read_to_string(Path::new(&filename)).unwrap();
        let states = parse_tddft(content);
        dbg!(states);
    }
    #[test]
    fn test_parse_casscf() {
        let filename = "tests/data/product_sa_casscf_v8_5a897afa.out";
        let content = fs::read_to_string(Path::new(&filename)).unwrap();
        let states = parse_casscf(content);
        dbg!(states);
    }
}
