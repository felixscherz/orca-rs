use std::str::Lines;

use crate::outputs::casscf::CASSCFOutput;
use crate::outputs::casscf::Contribution;
use crate::outputs::casscf::Orbital;

#[derive(Debug)]
pub enum Token {
    TableHeader(Vec<String>),
    Contribution(String, String, String, Vec<String>),
    TableDivider,
    EmptyLine,
    SecionDivider,
}

fn line_to_token(line: &str) -> Token {
    match line {
        x if x.is_empty() => Token::EmptyLine,
        x if x.contains(
            "------------------------------------------------------------------------------",
        ) =>
        {
            Token::SecionDivider
        }
        x if x.contains("--------") => Token::TableDivider,
        x => {
            let mut parts = x.split_whitespace();
            match x.split_whitespace().count() {
                5 | 6 => Token::TableHeader(parts.map(|x| x.to_string()).collect()),
                _ => Token::Contribution(
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.map(|x| x.to_string()).collect(),
                ),
            }
        }
    }
}

fn tokens_to_orbitals(tokens: &Vec<Token>) -> Vec<Orbital> {
    let mut iterator = tokens.iter();
    let mut orbitals: Vec<Orbital> = Vec::new();
    match iterator.next() {
        None => (),
        Some(Token::SecionDivider) => (),
        Some(Token::EmptyLine) => (),
        Some(Token::TableHeader(nrs)) => {
            // start of a table -> parse three lines, then a divider, then orbitals until empty
            // line
            // have to deal with 6 orbitals at  a time here
            let mut new_orbitals: Vec<Orbital> = Vec::new();
            if let Some(Token::TableHeader(energies)) = iterator.next() {
                if let Some(Token::TableHeader(occupations)) = iterator.next() {
                    (0..nrs.len()).for_each(|i| {
                        let orbital = Orbital {
                            orbital_nr: nrs.get(i).unwrap().parse().unwrap(),
                            energy: energies.get(i).unwrap().parse().unwrap(),
                            occupation: occupations.get(i).unwrap().parse().unwrap(),
                            contributions: Vec::new(),
                        };
                        new_orbitals.push(orbital);
                    });
                };
            };
            if let Some(Token::TableDivider) = iterator.next() {
            } else {
                panic!("Table divider expected");
            }
            // orbitals are parsed now go one until an emptyLine token is encountered
            loop {
                match iterator.next() {
                    Some(Token::EmptyLine) => break,
                    Some(Token::Contribution(atom_nr, element, orbital_type, contributions)) => {
                        new_orbitals
                            .iter_mut()
                            .enumerate()
                            .map(|(i, x)| {
                                x.contributions.push(Contribution {
                                    atom_nr: atom_nr.parse().unwrap(),
                                    element: element.to_string(),
                                    orbital_type: orbital_type.to_string(),
                                    contribution: contributions.get(i).unwrap().parse().unwrap(),
                                });
                            })
                            .count();
                    }
                    x => panic!("Unexpected token {:?}", x),
                }
            }
            orbitals.extend(new_orbitals);
        }
        Some(Token::Contribution(_, _, _, _)) => {
            panic!("Encountered Contribution")
        }
        Some(Token::TableDivider) => panic!("Encountered Divider"),
    }
    orbitals
}

pub fn parse(lines: &mut Lines) -> Vec<Orbital> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut started = false;
    loop {
        let candidate = lines.next();
        if candidate.is_none() {
            break;
        }
        let next = candidate.unwrap();
        if started {
            let token = line_to_token(next);
            match token {
                Token::SecionDivider => {
                    break;
                }
                _ => tokens.push(token),
            }
        }
        match next.trim_start() {
            "----------------------------" => {
                lines.next();
                started = true;
            }
            _ => (),
        }
    }
    tokens_to_orbitals(&tokens)
}

pub fn parse_casscf(content: String) -> Option<CASSCFOutput> {
    let mut lines = content.lines();
    loop {
        let candidate = lines.next();
        if candidate.is_none() {
            break None;
        }
        let next = candidate.unwrap();
        match next.trim_start() {
            "LOEWDIN ORBITAL-COMPOSITIONS" => {
                break Some(CASSCFOutput {
                    orbital_contributions: parse(&mut lines),
                })
            }
            _ => (),
        }
    }
}
