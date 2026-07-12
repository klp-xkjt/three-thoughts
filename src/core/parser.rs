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
    Err(ThreeThoughtsError::UnknownCategory)
}

pub fn get_sub(str: &str, index: usize) -> &str {
    str.split_whitespace().nth(index).unwrap_or("")
}
pub fn get_values(str: &str) -> Vec<&str> {
    str.split(",").collect::<Vec<&str>>()
}

/// 将 index 之后的所有 token 拼接成一个字符串（去除中间空格）
/// "Loop 10, 16, 3" → get_rest(2) → "10,16,3"
pub fn get_rest(str: &str, index: usize) -> String {
    str.split_whitespace()
        .skip(index)
        .collect::<Vec<_>>()
        .join("")
}

pub fn parse_who(str: &str) -> Result<WhoInstruction, ThreeThoughtsError> {
    let sub = get_sub(str, 1);
    match sub {
        "Named" => {
            let name = get_sub(str, 2);
            if name.is_empty() {
                return Err(ThreeThoughtsError::MissingWhoName("Named".into()));
            }
            Ok(WhoInstruction::Named(name.to_string()))
        }
        "Renamed" => {
            let name = get_sub(str, 2);
            if name.is_empty() {
                return Err(ThreeThoughtsError::MissingWhoName("Renamed".into()));
            }
            Ok(WhoInstruction::Renamed(name.to_string()))
        }
        _ => Err(ThreeThoughtsError::UnknownWhoInstruction(sub.to_string())),
    }
}

pub fn parse_where(str: &str) -> Result<WhereInstruction, ThreeThoughtsError> {
    let sub = get_sub(str, 1);
    match sub {
        "Origin" => Ok(WhereInstruction::Origin),
        "Keep" => Ok(WhereInstruction::Keep),
        "JumpTo" => {
            let arg = get_sub(str, 2);
            let val = arg
                .parse::<usize>()
                .map_err(|_| ThreeThoughtsError::InvalidWhereArg {
                    instruction: "JumpTo".into(),
                    arg: arg.into(),
                })?;
            Ok(WhereInstruction::JumpTo(val))
        }
        "Add" => {
            let arg = get_sub(str, 2);
            let val = arg
                .parse::<usize>()
                .map_err(|_| ThreeThoughtsError::InvalidWhereArg {
                    instruction: "Add".into(),
                    arg: arg.into(),
                })?;
            Ok(WhereInstruction::Add(val))
        }
        "Sub" => {
            let arg = get_sub(str, 2);
            let val = arg
                .parse::<usize>()
                .map_err(|_| ThreeThoughtsError::InvalidWhereArg {
                    instruction: "Sub".into(),
                    arg: arg.into(),
                })?;
            Ok(WhereInstruction::Sub(val))
        }
        _ => Err(ThreeThoughtsError::UnknownWhereInstruction(sub.to_string())),
    }
}

pub fn parse_what(str: &str) -> Result<WhatInstruction, ThreeThoughtsError> {
    let sub = get_sub(str, 1);

    // 辅助：解析单个 usize 参数并报告错误
    let parse_usize = |inst_name: &str| -> Result<usize, ThreeThoughtsError> {
        let arg = get_sub(str, 2);
        arg.parse::<usize>()
            .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                instruction: inst_name.into(),
                arg: arg.into(),
            })
    };

    // 辅助：解析两个逗号分隔的 usize 参数（IfSome/Dump/Reverse/AddOther... 共用）
    let parse_pair = |inst_name: &str| -> Result<(usize, usize), ThreeThoughtsError> {
        let rest = get_rest(str, 2);
        let vals: Vec<&str> = get_values(&rest)
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        if vals.len() < 2 {
            return Err(ThreeThoughtsError::IncompleteAnyCoupleArgs(
                inst_name.into(),
                "arg1".into(),
                "arg2".into(),
                vals.len(),
            ));
        }
        let v1 = vals[0].parse::<usize>().map_err(|_| ThreeThoughtsError::InvalidWhatArg {
            instruction: inst_name.into(),
            arg: vals[0].into(),
        })?;
        let v2 = vals[1].parse::<usize>().map_err(|_| ThreeThoughtsError::InvalidWhatArg {
            instruction: inst_name.into(),
            arg: vals[1].into(),
        })?;
        Ok((v1, v2))
    };

    match sub {
        "Print" => Ok(WhatInstruction::Print),
        "Println" => Ok(WhatInstruction::Println),
        "Loop" => {
            let rest = get_rest(str, 2);
            let vals: Vec<&str> = get_values(&rest)
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect();
            if vals.len() < 3 {
                return Err(ThreeThoughtsError::IncompleteLoopArgs(vals.len()));
            }
            let start_pc =
                vals[0]
                    .parse::<usize>()
                    .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                        instruction: "Loop".into(),
                        arg: vals[0].into(),
                    })?;
            let end_pc =
                vals[1]
                    .parse::<usize>()
                    .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                        instruction: "Loop".into(),
                        arg: vals[1].into(),
                    })?;
            let times =
                vals[2]
                    .parse::<usize>()
                    .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                        instruction: "Loop".into(),
                        arg: vals[2].into(),
                    })?;

            Ok(WhatInstruction::Loop(LoopState {
                start_pc,
                end_pc,
                times,
                loop_pc: 0,
            }))
        }
        "Add" => Ok(WhatInstruction::Add(parse_usize("Add")?)),
        "AddOther" => {
            let (addr, add) = parse_pair("AddOther")?;
            Ok(WhatInstruction::AddOther(addr, add))
        },
        "SubOther" => {
            let (addr, sub) = parse_pair("SubOther")?;
            Ok(WhatInstruction::SubOther(addr, sub))
        },
        "Sub" => Ok(WhatInstruction::Sub(parse_usize("Sub")?)),
        "Free" => Ok(WhatInstruction::Free),
        "FreeOther" => Ok(WhatInstruction::FreeOther(parse_usize("FreeOther")?)),
        "Note" => Ok(WhatInstruction::Note),
        "Reset" => Ok(WhatInstruction::Reset),
        "ResetOrigin" => Ok(WhatInstruction::ResetOrigin),
        "IfZero" => Ok(WhatInstruction::IfZero(parse_usize("IfZero")?)),
        "IfNotZero" => Ok(WhatInstruction::IfNotZero(parse_usize("IfNotZero")?)),
        "IfSome" => {
            let (value, pc) = parse_pair("IfSome")?;
            Ok(WhatInstruction::IfSome(value, pc))
        }
        "IfNotSome" => {
            let (value, pc) = parse_pair("IfNotSome")?;
            Ok(WhatInstruction::IfNotSome(value, pc))
        }
        "IfName" => {
            let rest = get_rest(str, 2);
            let vals: Vec<String> = rest.split(',').map(|s| s.to_string()).collect();
            let non_empty: Vec<&String> = vals.iter().filter(|s| !s.is_empty()).collect();
            if non_empty.len() < 2 {
                return Err(ThreeThoughtsError::IncompleteIfNameArgs(non_empty.len()));
            }
            let name = non_empty[0].clone();
            let pc =
                non_empty[1]
                    .parse::<usize>()
                    .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                        instruction: "IfName".into(),
                        arg: non_empty[1].clone(),
                    })?;
            Ok(WhatInstruction::IfName(name, pc))
        }
        "JumpTo" => Ok(WhatInstruction::JumpTo(parse_usize("JumpTo")?)),
        "Panic" => {
            let rest = get_rest(str, 2);
            Ok(WhatInstruction::Panic(rest))
        }
        "Read" => {
            let rest = get_rest(str, 2);
            Ok(WhatInstruction::Read(parse_usize(&rest)?))
        }
        "ReadASCII" => {
            let rest = get_rest(str, 2);
            Ok(WhatInstruction::ReadASCII(parse_usize(&rest)?))
        }
        "Dump" => {
            let (addr1, addr2) = parse_pair("Dump")?;
            Ok(WhatInstruction::Dump(addr1, addr2))
        }
        "Reverse" => {
            let (addr1, addr2) = parse_pair("Reverse")?;
            Ok(WhatInstruction::Reverse(addr1, addr2))
        },
        "GetOther" => Ok(WhatInstruction::GetOther(parse_usize("GetOther")?)),
        _ => Err(ThreeThoughtsError::UnknownWhatInstruction(sub.to_string())),
    }
}

pub fn parse_program(content: &str) -> Result<Vec<ThreeThoughts>, ThreeThoughtsError> {
    let mut instructions = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        // 跳过空行和纯注释行
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        // 剥离行内 // 注释
        let code = match trimmed.find("//") {
            Some(pos) => trimmed[..pos].trim(),
            None => trimmed,
        };
        if code.is_empty() {
            continue;
        }
        let inst = parse_instruction(code)?;
        instructions.push(inst);
    }
    if instructions.is_empty() {
        return Err(ThreeThoughtsError::EmptyProgram);
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

    // ═══════════════════════════════════════════
    // 新增：错误类型专项测试
    // ═══════════════════════════════════════════

    #[test]
    fn error_unknown_category() {
        // 没有任何已知前缀的行
        let result = parse_instruction("just some random text");
        assert_eq!(result, Err(ThreeThoughtsError::UnknownCategory));
    }

    #[test]
    fn error_unknown_who_instruction() {
        let result = parse_who("[WhoAmI] UnknownOp");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::UnknownWhoInstruction(
                "UnknownOp".into()
            ))
        );
    }

    #[test]
    fn error_missing_who_name() {
        let result1 = parse_who("[WhoAmI] Named");
        assert_eq!(
            result1,
            Err(ThreeThoughtsError::MissingWhoName("Named".into()))
        );

        let result2 = parse_who("[WhoAmI] Renamed");
        assert_eq!(
            result2,
            Err(ThreeThoughtsError::MissingWhoName("Renamed".into()))
        );
    }

    #[test]
    fn error_unknown_where_instruction() {
        let result = parse_where("[WhereAmI] FlyTo 10");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::UnknownWhereInstruction("FlyTo".into()))
        );
    }

    #[test]
    fn error_invalid_where_arg() {
        let result = parse_where("[WhereAmI] JumpTo abc");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::InvalidWhereArg {
                instruction: "JumpTo".into(),
                arg: "abc".into(),
            })
        );
    }

    #[test]
    fn error_unknown_what_instruction() {
        let result = parse_what("[WhatDoIDo] Sing");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::UnknownWhatInstruction("Sing".into()))
        );
    }

    #[test]
    fn error_invalid_what_arg() {
        let result = parse_what("[WhatDoIDo] Add xyz");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::InvalidWhatArg {
                instruction: "Add".into(),
                arg: "xyz".into(),
            })
        );
    }

    #[test]
    fn error_incomplete_loop_args() {
        // 只有 1 个参数
        let result = parse_what("[WhatDoIDo] Loop 10");
        assert_eq!(result, Err(ThreeThoughtsError::IncompleteLoopArgs(1)));
    }

    #[test]
    fn error_incomplete_if_some_args() {
        // 只有 1 个参数
        let result = parse_what("[WhatDoIDo] IfSome 48");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::IncompleteAnyCoupleArgs(
                "IfSome".into(),
                "arg1".into(),
                "arg2".into(),
                1,
            ))
        );
    }

    #[test]
    fn error_incomplete_if_name_args() {
        // 只有名称，没有 PC
        let result = parse_what("[WhatDoIDo] IfName Rustacean");
        assert_eq!(result, Err(ThreeThoughtsError::IncompleteIfNameArgs(1)));
    }

    #[test]
    fn error_empty_program() {
        // 只有注释的程序
        let result = parse_program("// just a comment\n// another comment\n");
        assert_eq!(result, Err(ThreeThoughtsError::EmptyProgram));
    }

    #[test]
    fn error_parse_if_zero_with_bad_arg() {
        let result = parse_what("[WhatDoIDo] IfZero NaN");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::InvalidWhatArg {
                instruction: "IfZero".into(),
                arg: "NaN".into(),
            })
        );
    }

    #[test]
    fn error_parse_jump_to_with_bad_arg() {
        let result = parse_what("[WhatDoIDo] JumpTo nowhere");
        assert_eq!(
            result,
            Err(ThreeThoughtsError::InvalidWhatArg {
                instruction: "JumpTo".into(),
                arg: "nowhere".into(),
            })
        );
    }
}
