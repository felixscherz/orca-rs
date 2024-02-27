use std::{fs, path::Path};

#[cfg(test)]
mod tests {
    use orca_rs::parser::parse;

    use super::*;
    #[test]
    fn test_tokenize_file() {
        let filename = "tests/data/matrix.out";
        let content = fs::read_to_string(Path::new(&filename)).unwrap();
        let mut iterator = content.lines();
        while !iterator.next().unwrap().starts_with("Eigenvectors") {}
        iterator.next();
        let states = parse(&iterator.collect());
        for state in states.iter() {
            dbg!(state);
        }
    }
}
