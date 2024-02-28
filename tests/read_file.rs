use std::{fs, path::Path};

#[cfg(test)]
mod tests {
    use orca_rs::parsers::parse;

    use super::*;
    #[test]
    fn test_tokenize_file() {
        let filename = "tests/data/ch2o_soc_tddft.out";
        let content = fs::read_to_string(Path::new(&filename)).unwrap();
        let states = parse(content).unwrap();
        for state in states.iter() {
            dbg!(state);
        }
    }
}
