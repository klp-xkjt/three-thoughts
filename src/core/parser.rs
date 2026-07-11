use crate::core::error::*;
use crate::core::instruction::*;
use crate::core::loop_and_condition::LoopState;

pub fn parse_instruction(ts3: &str) -> Result<ThreeThoughts, ThreeThoughtsError> {
    let lines = ts3.lines();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["[WhoAmI]", ..] => return Ok(ThreeThoughts::WhoAmI(parse_who(line)?)),
            ["[WhereAmI]", ..] => return Ok(ThreeThoughts::WhereAmI(parse_where(line)?)),
            ["[WhatDoIDo]", ..] => return Ok(ThreeThoughts::WhatDoIDo(parse_what(line)?)),
            _ => continue,
        }
    }
    Err(ThreeThoughtsError::NoInstructions(
        "No matching instruction found".to_string(),
    ))
}

pub fn get_sub(str: &str, index: usize) -> &str {
    str.split_whitespace().nth(index).unwrap_or("")
}
pub fn get_values(str: &str) -> Vec<&str> {
    str.split(",").collect::<Vec<&str>>()
}

pub fn parse_who(str: &str) -> Result<WhoInstruction, ThreeThoughtsError> {
    let sub = get_sub(str, 1);
    match sub {
        "Named" => Ok(WhoInstruction::Named(get_sub(str, 2).to_string())),
        "Renamed" => Ok(WhoInstruction::Renamed(get_sub(str, 2).to_string())),
        _ => Err(ThreeThoughtsError::NoInstructions(format!(
            "No instructions of WhoAmI: {}",
            sub
        ))),
    }
}

pub fn parse_where(str: &str) -> Result<WhereInstruction, ThreeThoughtsError> {
    let sub = get_sub(str, 1);
    match sub {
        "Origin" => Ok(WhereInstruction::Origin),
        "Keep" => Ok(WhereInstruction::Keep),
        "JumpTo" => Ok(WhereInstruction::JumpTo(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        "Add" => Ok(WhereInstruction::Add(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        "Sub" => Ok(WhereInstruction::Sub(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        _ => Err(ThreeThoughtsError::NoInstructions(format!(
            "No instructions of WhereAmI: {}",
            sub
        ))),
    }
}

pub fn parse_what(str: &str) -> Result<WhatInstruction, ThreeThoughtsError> {
    let sub = get_sub(str, 1);
    match sub {
        "Print" => Ok(WhatInstruction::Print),
        "Println" => Ok(WhatInstruction::Println),
        "Loop" => {
            let sub = get_sub(str, 2);
            let vals = get_values(sub);
            let loop_inst = LoopState {
                start_pc: vals.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
                end_pc: vals.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
                times: vals.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
            };
            Ok(WhatInstruction::Loop(loop_inst))
        }
        "Add" => Ok(WhatInstruction::Add(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        "Sub" => Ok(WhatInstruction::Sub(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        "Note" => Ok(WhatInstruction::Note),
        "Reset" => Ok(WhatInstruction::Reset),
        "ResetOrigin" => Ok(WhatInstruction::ResetOrigin),
        "IfZero" => Ok(WhatInstruction::IfZero(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        "IfNotZero" => Ok(WhatInstruction::IfNotZero(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        "IfSome" => {
            let sub = get_sub(str, 2);
            let vals = get_values(sub);
            Ok(WhatInstruction::IfSome(
                vals.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
                vals.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            ))
        }
        "IfNotSome" => {
            let sub = get_sub(str, 2);
            let vals = get_values(sub);
            Ok(WhatInstruction::IfNotSome(
                vals.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
                vals.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            ))
        }
        "IfName" => {
            let sub = get_sub(str, 2);
            let vals: Vec<String> = sub.split(',').map(|s| s.to_string()).collect();
            let name = vals.get(0).cloned().unwrap_or_default();
            let pc = vals.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            Ok(WhatInstruction::IfName(name, pc))
        }
        "JumpTo" => Ok(WhatInstruction::JumpTo(
            get_sub(str, 2).parse::<usize>().unwrap_or(0),
        )),
        _ => Err(ThreeThoughtsError::NoInstructions(format!(
            "No instructions of WhatDoIDo: {}",
            sub
        ))),
    }
}

pub fn parse_program(content: &str) -> Result<Vec<ThreeThoughts>, ThreeThoughtsError> {
    let mut instructions = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let inst = parse_instruction(trimmed)?;
        instructions.push(inst);
    }
    if instructions.is_empty() {
        return Err(ThreeThoughtsError::NoInstructions(
            "No valid instructions found".to_string(),
        ));
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_test() {
        let str = "The quick brown fox jumps over the lazy dog.";
        assert_eq!("brown", get_sub(str, 2));
        assert_eq!("lazy", get_sub(str, 7));
    }

    #[test]
    fn parse_who_test() {
        let str1 = "[WhoAmI] Named I";
        let str2 = "[WhoAmI] Renamed TrueI";
        assert_eq!(parse_who(str1), Ok(WhoInstruction::Named("I".to_string())));
        assert_eq!(
            parse_who(str2),
            Ok(WhoInstruction::Renamed("TrueI".to_string()))
        );
    }

    #[test]
    fn parse_where_test() {
        let str1 = "[WhereAmI] JumpTo 10";
        let str2 = "[WhereAmI] Add 10";
        assert_eq!(parse_where(str1), Ok(WhereInstruction::JumpTo(10)));
        assert_eq!(parse_where(str2), Ok(WhereInstruction::Add(10)));
    }

    #[test]
    fn parse_what_test() {
        let str1 = "[WhatDoIDo] Print";
        let str2 = "[WhatDoIDo] Add 10";
        assert_eq!(parse_what(str1), Ok(WhatInstruction::Print));
        assert_eq!(parse_what(str2), Ok(WhatInstruction::Add(10)));
    }

    #[test]
    fn parse_instruction_test() {
        let str1 = "[WhoAmI] Renamed TrueI";
        let str2 = "[WhatDoIDo] Add 7";
        let str3 = "[WhereAmI] JumpTo 5";
        assert_eq!(
            parse_instruction(str1),
            Ok(ThreeThoughts::WhoAmI(WhoInstruction::Renamed(
                "TrueI".to_string()
            )))
        );
        assert_eq!(
            parse_instruction(str2),
            Ok(ThreeThoughts::WhatDoIDo(WhatInstruction::Add(7)))
        );
        assert_eq!(
            parse_instruction(str3),
            Ok(ThreeThoughts::WhereAmI(WhereInstruction::JumpTo(5)))
        );
    }

    #[test]
    fn parse_program_test() {
        let str0 = "[WhatDoIDo] Note This-is-a-\"Hello-World!\"-of-3Thoughts
[WhatDoIDo] Add 72
[WhatDoIDo] Print
[WhereAmI] Add 1  
[WhatDoIDo] Add 101
[WhatDoIDo] Print";
        assert_eq!(
            parse_program(str0),
            Ok(vec![
                ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
                ThreeThoughts::WhatDoIDo(WhatInstruction::Add(72)),
                ThreeThoughts::WhatDoIDo(WhatInstruction::Print),
                ThreeThoughts::WhereAmI(WhereInstruction::Add(1)),
                ThreeThoughts::WhatDoIDo(WhatInstruction::Add(101)),
                ThreeThoughts::WhatDoIDo(WhatInstruction::Print)
            ])
        );
    }
}
