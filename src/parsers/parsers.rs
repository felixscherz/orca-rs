use crate::outputs::SOCState;

use super::soc::parse_soc;

pub fn parse(content: String) -> Option<Vec<SOCState>> {
    let mut lines = content.lines();
    let mut result = None;
    loop {
        let candidate = lines.next();
        if candidate.is_none() {
            break result;
        }
        let next = candidate.unwrap();
        match next.trim_start() {
            "ORCA SPIN-ORBIT COUPLING CALCULATION" => {
                result = Some(parse_soc(&mut lines));
            }
            _ => (),
        }
    }
}
