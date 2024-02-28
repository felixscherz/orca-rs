use crate::outputs::OrcaOutput;

use super::soc::parse_soc;

pub fn parse(content: String) -> OrcaOutput {
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
            }
            _ => (),
        }
    }
    OrcaOutput {
        soc_matrix: soc_matrix.unwrap(),
    }
}
