use std::str::Lines;

use crate::outputs::SOCEigenvector;
use crate::outputs::SOCElement;
use crate::outputs::SOCMatrix;
use crate::outputs::TDDFTOutput;

#[derive(Debug)]
pub enum SOCMatrixToken {
    Header,
    State(String, String),
    Component(String, String, String, String, String, String),
    EndOfTable,
}

fn line_to_token(line: &str) -> SOCMatrixToken {
    if line.is_empty() {
        return SOCMatrixToken::EndOfTable;
    };
    match line.trim_start().chars().next().unwrap() {
        'E' => SOCMatrixToken::Header,
        'S' => tokenize_state(line),
        _ => tokenize_component(line),
    }
}

fn tokenize_state(line: &str) -> SOCMatrixToken {
    let mut parts = line.split_whitespace();
    parts.next();
    let state_no = parts.next().unwrap().replace(":", "");
    let energy = parts.next().unwrap().to_string();
    SOCMatrixToken::State(state_no, energy)
}

fn tokenize_component(line: &str) -> SOCMatrixToken {
    let mut parts = line.split(":");
    let mut vals = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string());
    let mut orbs = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string());
    let weight = vals.next().unwrap();
    let real = vals.next().unwrap();
    let imag = vals.next().unwrap();
    let root = orbs.next().unwrap();
    let spin = orbs.next().unwrap();
    let ms = orbs.next().unwrap();

    SOCMatrixToken::Component(weight, real, imag, root, spin, ms)
}

fn to_states(tokens: &Vec<SOCMatrixToken>) -> Vec<SOCEigenvector> {
    let mut states: Vec<SOCEigenvector> = Vec::new();
    for token in tokens.iter() {
        match token {
            SOCMatrixToken::Header => (),
            SOCMatrixToken::State(n, energy) => {
                states.push(SOCEigenvector {
                    n: n.parse().unwrap(),
                    energy: energy.parse().unwrap(),
                    elements: Vec::new(),
                });
            }
            SOCMatrixToken::Component(weight, real, imag, root, spin, ms) => {
                let eigenvec = SOCElement {
                    weight: weight.parse().unwrap(),
                    real: real.parse().unwrap(),
                    imag: imag.parse().unwrap(),
                    root: root.parse().unwrap(),
                    spin: spin.parse().unwrap(),
                    ms: ms.parse().unwrap(),
                };
                if let Some(last_state) = &mut states.last_mut() {
                    last_state.elements.push(eigenvec);
                }
            }
            SOCMatrixToken::EndOfTable => break,
        }
    }
    states
}

pub fn parse_soc(lines: &mut Lines) -> SOCMatrix {
    let mut tokens: Vec<SOCMatrixToken> = Vec::new();
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
                SOCMatrixToken::EndOfTable => {
                    break;
                }
                _ => tokens.push(token),
            }
        }
        match next.trim_start() {
            "Eigenvectors of the SOC matrix:" => {
                lines.next();
                started = true;
            }
            _ => (),
        }
    }
    SOCMatrix {
        eigenvectors: to_states(&tokens),
    }
}


pub fn parse_tddft(content: String) -> TDDFTOutput {
    let mut lines = content.lines();
    let mut soc_matrix = None;
    loop {
        let candidate = lines.next();
        if candidate.is_none() {
            break;
        }
        let next = candidate.unwrap();
        match next.trim_start() {
            "ORCA SPIN-ORBIT COUPLING CALCULATION" => {
                soc_matrix = Some(parse_soc(&mut lines));
            },
            _ => (),
        }
    }
    TDDFTOutput {
        soc_matrix: soc_matrix.unwrap(),
    }
}
