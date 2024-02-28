use std::str::Lines;

use crate::outputs::SOCEigenvector;
use crate::outputs::SOCState;

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

// should this take a buffer to continonusly read?
// or a large string reference?
// should be able to read the entire file, without slicing beforehand
// tokenizer should tokenize every line on a higher level
pub fn tokenize(lines: &Vec<&str>) -> Vec<SOCMatrixToken> {
    lines.iter().map(|x| line_to_token(x)).collect()
}

fn to_states(tokens: &Vec<SOCMatrixToken>) -> Vec<SOCState> {
    let mut states: Vec<SOCState> = Vec::new();
    for token in tokens.iter() {
        match token {
            SOCMatrixToken::Header => (),
            SOCMatrixToken::State(n, energy) => {
                states.push(SOCState {
                    n: n.parse().unwrap(),
                    energy: energy.parse().unwrap(),
                    eigenvectors: Vec::new(),
                });
            }
            SOCMatrixToken::Component(weight, real, imag, root, spin, ms) => {
                let eigenvec = SOCEigenvector {
                    weight: weight.parse().unwrap(),
                    real: real.parse().unwrap(),
                    imag: imag.parse().unwrap(),
                    root: root.parse().unwrap(),
                    spin: spin.parse().unwrap(),
                    ms: ms.parse().unwrap(),
                };
                if let Some(last_state) = &mut states.last_mut() {
                    last_state.eigenvectors.push(eigenvec);
                }
            }
            SOCMatrixToken::EndOfTable => break,
        }
    }
    states
}

pub fn parse(lines: &Vec<&str>) -> Vec<SOCState> {
    let tokens = tokenize(lines);
    to_states(&tokens)
}

pub fn parse_soc(lines: &mut Lines) -> Vec<SOCState> {
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
                SOCMatrixToken::EndOfTable => {break;}
                _ => tokens.push(token)
            }
        }
        match next.trim_start() {
            "Eigenvectors of the SOC matrix:" => {lines.next(); started = true;},
            _ => (),
        }
    }
    to_states(&tokens)
}

