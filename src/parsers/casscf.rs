use std::str::Lines;

#[derive(Debug)]
pub enum Token {
    TableHeader(String, String, String, String, String, String),
    LastTableHeader(String, String, String, String, String),
    Contribution(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ),
    LastContribution(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ),
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
                5 => Token::LastTableHeader(
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                ),
                6 => Token::TableHeader(
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                ),
                8 => Token::LastContribution(
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                ),
                _ => Token::Contribution(
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                ),
            }
        }
    }
}

#[derive(Debug)]
struct Contribution {
    pub atom_nr: u32,
    pub element: String,
    pub orbital_type: String,
    pub contribution: f32,
}

#[derive(Debug)]
struct Orbital {
    pub orbital_nr: u32,
    pub energy: f32,
    pub occupation: f32,
}

fn tokens_to_orbitals(tokens: &Vec<Token>) {
    let mut iterator = tokens.iter();
    match iterator.next() {
        None => (),
        Some(Token::SecionDivider) => (),
        Some(Token::EmptyLine) => (),
        Some(Token::TableHeader(x1, x2, x3, x4, x5, x6)) => (),
        Some(Token::Contribution(x1, x2, x3, x4, x5, x6, x7, x8, x9)) => (),
        Some(Token::LastTableHeader(x1, x2, x3, x4, x5)) => (),
        Some(Token::LastContribution(x1, x2, x3, x4, x5, x6, x7, x8)) => (),
        Some(Token::TableDivider) => (),
    }
}

pub fn parse(lines: &mut Lines) -> () {
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
}


pub fn parse_casscf(content: String) -> () {
    let mut lines = content.lines();
    loop {
        let candidate = lines.next();
        if candidate.is_none() {
            break;
        }
        let next = candidate.unwrap();
        match next.trim_start() {
            "LOEWDIN ORBITAL-COMPOSITIONS" => {
                parse(&mut lines);
            },
            _ => (),
        }
    }
}

