use crate::core::instruction::*;

pub fn parse_instruction(ts3: &str) -> ThreeThoughts {
    ts3.lines()
        .find_map(|x| {
            let parts = x.split_whitespace().collect::<Vec<_>>();
            match parts.as_slice() {
                ["[WhoAmI]", ..] => {
                    Some(ThreeThoughts::WhoAmI(parse_who(x)))
                },
                ["[WhereAmI]", ..] => {
                    Some(ThreeThoughts::WhereAmI(parse_where(x)))
                },
                ["[WhatDoIDo]", ..] => {
                    Some(ThreeThoughts::WhatDoIDo(parse_what(x)))
                },
                _ => None
            }
        })
        .unwrap_or(ThreeThoughts::None)
}

pub fn get_sub<'a>(str: &'a str, index: usize) -> &'a str {
    str.split_whitespace().nth(index).unwrap_or("")
}

pub fn parse_who(str: &str) -> WhoInstruction {
    let sub = get_sub(str, 1);
    match sub {
        "Named" => WhoInstruction::Named(get_sub(str, 2).to_string()),
        "Renamed" => WhoInstruction::Renamed(get_sub(str, 2).to_string()),
        _ => panic!("None"),
    }
}

pub fn parse_where(str: &str) -> WhereInstruction {
    let sub = get_sub(str, 1);
    match sub {
        "Origin" => WhereInstruction::Origin,
        "Keep" => WhereInstruction::Keep,
        "JumpTo" => WhereInstruction::JumpTo(get_sub(str, 2).parse::<usize>().unwrap_or(0)),
        "Add" => WhereInstruction::Add(get_sub(str, 2).parse::<usize>().unwrap_or(0)),
        "Sub" => WhereInstruction::Sub(get_sub(str, 2).parse::<usize>().unwrap_or(0)),
        _ => panic!("None"),
    }
}

pub fn parse_what(str: &str) -> WhatInstruction {
    let sub = get_sub(str, 1);
    match sub {
        "Print" => WhatInstruction::Print,
        "Add" => WhatInstruction::Add(get_sub(str, 2).parse::<usize>().unwrap_or(0)),
        "Sub" => WhatInstruction::Sub(get_sub(str, 2).parse::<usize>().unwrap_or(0)),
        _ => panic!("None"),
    }
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
        assert_eq!(parse_who(str1), WhoInstruction::Named("I".to_string()));
        assert_eq!(parse_who(str2), WhoInstruction::Renamed("TrueI".to_string()));
    }

    #[test] 
    fn parse_where_test() {
        let str1 = "[WhereAmI] JumpTo 10";
        let str2 = "[WhereAmI] Add 10";
        assert_eq!(parse_where(str1), WhereInstruction::JumpTo(10));
        assert_eq!(parse_where(str2), WhereInstruction::Add(10));
    }

    #[test] 
    fn parse_what_test() {
        let str1 = "[WhatDoIDo] Print";
        let str2 = "[WhatDoIDo] Add 10";
        assert_eq!(parse_what(str1), WhatInstruction::Print);
        assert_eq!(parse_what(str2), WhatInstruction::Add(10));
    }

    #[test]
    fn parse_instruction_test() {
        let str1 = "[WhoAmI] Renamed TrueI";
        let str2 = "[WhatDoIDo] Add 7";
        let str3 = "[WhereAmI] JumpTo 5";
        assert_eq!(parse_instruction(str1), ThreeThoughts::WhoAmI(WhoInstruction::Renamed("TrueI".to_string())));
        assert_eq!(parse_instruction(str2), ThreeThoughts::WhatDoIDo(WhatInstruction::Add(7)));
        assert_eq!(parse_instruction(str3), ThreeThoughts::WhereAmI(WhereInstruction::JumpTo(5)));
    }
}