use crate::core::error::ThreeThoughtsError;
use crate::core::instruction::*;
use crate::core::loop_and_condition::LoopState;
use crate::core::memory::Memory;

pub struct VM {
    pub memory: Memory,
    pub pc: usize,
    pub instructions: Vec<ThreeThoughts>,
    pub running: bool,
    pub name: String,
    pub loop_stack: Vec<LoopState>,
    pub debug: bool,
}

impl VM {
    pub fn new(instructions: Vec<ThreeThoughts>, mem: usize, debug: bool) -> Self {
        VM {
            memory: Memory::new(mem),
            pc: 0,
            instructions,
            running: true,
            name: String::new(),
            loop_stack: Vec::new(),
            debug,
        }
    }

    pub fn execute_instruction(
        &mut self,
        inst: &ThreeThoughts,
    ) -> Result<bool, ThreeThoughtsError> {
        match inst {
            ThreeThoughts::WhoAmI(who) => {
                match who {
                    WhoInstruction::Named(a) => self.name = a.to_string(),
                    WhoInstruction::Renamed(a) => self.name = a.to_string(),
                }
                return Ok(false);
            }
            ThreeThoughts::WhereAmI(where_inst) => match *where_inst {
                WhereInstruction::Origin => {
                    self.memory.jump(0);
                    return Ok(false);
                }
                WhereInstruction::Keep => return Ok(false),
                WhereInstruction::Add(add) => {
                    self.memory.forward(add);
                    return Ok(false);
                }
                WhereInstruction::Sub(sub) => {
                    self.memory.backward(sub);
                    return Ok(false);
                }
                WhereInstruction::JumpTo(to) => {
                    self.memory.jump(to);
                    return Ok(false);
                }
            },
            ThreeThoughts::WhatDoIDo(what) => match what {
                WhatInstruction::Add(a) => {
                    self.memory.cells[self.memory.pointer] =
                        self.memory.cells[self.memory.pointer].wrapping_add(*a as u8);
                }
                WhatInstruction::AddOther(addr,add) => {
                    self.memory.cells[*addr] =
                        self.memory.cells[*addr].wrapping_add(*add as u8);
                }
                WhatInstruction::Sub(b) => {
                    self.memory.cells[self.memory.pointer] =
                        self.memory.cells[self.memory.pointer].wrapping_sub(*b as u8);
                }
                WhatInstruction::SubOther(addr,add) => {
                    self.memory.cells[*addr] =
                        self.memory.cells[*addr].wrapping_sub(*add as u8);
                }
                WhatInstruction::Free => {
                    self.memory.set_current(0);
                }
                WhatInstruction::FreeOther(a) => {
                    self.memory.cells[*a] = 0
                }
                WhatInstruction::Print => {
                    print!("{}", self.memory.cells[self.memory.pointer] as char)
                }
                WhatInstruction::Println => {
                    println!("{}", self.memory.cells[self.memory.pointer] as char)
                }
                WhatInstruction::Note => {}
                WhatInstruction::Reset => {
                    self.memory.cells.fill(0);
                }
                WhatInstruction::ResetOrigin => {
                    self.memory.cells.fill(0);
                    self.memory.jump(0);
                }
                // ========== 核心修改：Loop 分支 ==========
                WhatInstruction::Loop(l) => {
                    let current_pc = self.pc; // 记下 Loop 指令自己的位置

                    // 检查栈顶是不是当前循环
                    if let Some(top) = self.loop_stack.last()
                        && top.start_pc == l.start_pc
                        && top.end_pc == l.end_pc
                    {
                        // 已经在循环中：减少次数
                        let last = self.loop_stack.last_mut().unwrap();
                        last.times -= 1;

                        if last.times == 0 {
                            // 循环结束：弹出栈，跳到循环体之后
                            self.loop_stack.pop();
                            self.pc = l.end_pc; // 设置 pc 为循环体末尾，run() 会 +1 跳到循环体之后
                            return Ok(false);
                        } else {
                            // 还有迭代：跳回循环体开头
                            self.pc = l.start_pc;
                            return Ok(true);
                        }
                    }

                    // 首次进入循环：压栈（带上 loop_pc），跳到循环体开头
                    self.loop_stack.push(LoopState {
                        start_pc: l.start_pc,
                        end_pc: l.end_pc,
                        times: l.times,
                        loop_pc: current_pc, // 👈 存下 Loop 指令的位置
                    });
                    self.pc = l.start_pc;
                    return Ok(true);
                }
                // =====================================
                WhatInstruction::IfZero(to) => {
                    if self.memory.current() == 0 {
                        self.pc = *to;
                        return Ok(true);
                    }
                    return Ok(false);
                }
                WhatInstruction::IfNotZero(to) => {
                    if self.memory.current() != 0 {
                        self.pc = *to;
                        return Ok(true);
                    }
                    return Ok(false);
                }
                WhatInstruction::IfSome(a, to) => {
                    if self.memory.current() == *a as u8 {
                        self.pc = *to;
                        return Ok(true);
                    }
                    return Ok(false);
                }
                WhatInstruction::IfNotSome(a, to) => {
                    if self.memory.current() != *a as u8 {
                        self.pc = *to;
                        return Ok(true);
                    }
                    return Ok(false);
                }
                WhatInstruction::IfName(a, to) => {
                    if self.name == *a {
                        self.pc = *to;
                        return Ok(true);
                    }
                    return Ok(false);
                }
                WhatInstruction::JumpTo(to) => {
                    self.pc = *to;
                    return Ok(true);
                }
                WhatInstruction::Panic(a) => {
                    panic!("{}", a)
                }
                WhatInstruction::Read(addr) => {
                    use std::io::Read;
                    let mut buf = [0u8; 1];
                    let byte = if std::io::stdin().read(&mut buf).unwrap_or(0) == 1 {
                        buf[0]
                    } else {
                        0
                    };
                    if *addr >= self.memory.cells.len() {
                        self.memory.expand(*addr);
                    }
                    self.memory.cells[*addr] = byte;
                }
                WhatInstruction::ReadASCII(addr) => {
                    use std::io::Read;
                    // 跳过前导空白和换行，避免被上次输入残留干扰
                    let mut buf = [0u8; 1];
                    loop {
                        if std::io::stdin().read(&mut buf).unwrap_or(0) != 1 {
                            break; // EOF
                        }
                        if buf[0].is_ascii_digit() {
                            break; // 第一个数字，开始解析
                        }
                        // 非数字（空白/换行/字母）→ 跳过继续等
                    }
                    // 解析连续数字
                    let mut value: u8 = 0;
                    if buf[0].is_ascii_digit() {
                        value = value.wrapping_mul(10).wrapping_add(buf[0] - b'0');
                        loop {
                            match std::io::stdin().read(&mut buf) {
                                Ok(1) if buf[0].is_ascii_digit() => {
                                    value = value.wrapping_mul(10).wrapping_add(buf[0] - b'0');
                                }
                                _ => break,
                            }
                        }
                    }
                    if *addr >= self.memory.cells.len() {
                        self.memory.expand(*addr);
                    }
                    self.memory.cells[*addr] = value;
                }
                WhatInstruction::Dump(addr1, addr2) => {
                    let start = *addr1;
                    let end = (*addr2).min(self.memory.cells.len());
                    for i in start..end {
                        print!("{} ", self.memory.cells[i]);
                    }
                    println!();
                },
                WhatInstruction::DumpWN(addr1, addr2) => {
                    let start = *addr1;
                    let end = (*addr2).min(self.memory.cells.len());
                    for i in start..end {
                        print!("{}:{} ", i, self.memory.cells[i]);
                    }
                    println!();
                },
                WhatInstruction::Reverse(addr1, addr2) => {
                    let start = *addr1;
                    let end = (*addr2).min(self.memory.cells.len());
                    self.memory.cells[start..end].reverse();
                },
                WhatInstruction::GetOther(a) => {
                    self.memory.cells[self.memory.pointer] = self.memory.cells[*a]
                },
                WhatInstruction::While(addr, pc1, pc2) => {
                    if self.memory.cells[*addr] != 0 {
                        self.memory.cells[*addr] -= 1;
                        self.pc = *pc1;
                        return Ok(true);
                    }
                    self.pc = *pc2;
                    return Ok(true);
                },
                WhatInstruction::Copy(from,to ) => {
                    self.memory.cells[*to] = self.memory.cells[*from];
                    self.memory.cells[*from] = 0;
                    return Ok(false);
                }
            },
        }
        Ok(false)
    }

    pub fn run(&mut self) -> Result<(), ThreeThoughtsError> {
        while self.running && self.pc < self.instructions.len() {
            // ── debug 输出 ──
            if self.debug {
                let inst = &self.instructions[self.pc];
                let loop_info = if let Some(top) = self.loop_stack.last() {
                    format!(
                        " | loop: start={} end={} remain={}",
                        top.start_pc, top.end_pc, top.times
                    )
                } else {
                    String::new()
                };
                eprintln!(
                    "[DEBUG] PC={:<4} ptr={:<4} cell={:<4} {:?}{}",
                    self.pc,
                    self.memory.pointer,
                    self.memory.current(),
                    inst,
                    loop_info,
                );
            }

            let inst = self.instructions[self.pc].clone();
            let jumped = self.execute_instruction(&inst)?;

            if !jumped {
                self.pc += 1;
            }

            // 循环尾检测：检查当前 pc 是否越过某个循环的 end_pc
            if let Some(top) = self.loop_stack.last()
                && self.pc > top.end_pc
            {
                // 精准跳回 Loop 指令位置（用存储的 loop_pc）
                self.pc = top.loop_pc;
                // 下一次循环会重新执行 Loop 指令，触发次数递减
            }
        }
        Ok(())
    }
    // =====================================
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::parse_instruction;

    #[test]
    fn test_vm_new() {
        let instructions = vec![];
        let vm = VM::new(instructions, 65536, false);
        assert_eq!(vm.pc, 0);
        assert!(vm.running);
        assert_eq!(vm.name, "");
        assert_eq!(vm.memory.pointer, 0);
    }

    #[test]
    fn test_add_and_sub() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 10")?,
            parse_instruction("[WhatDoIDo] Sub 3")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.memory.current(), 7);
        Ok(())
    }

    #[test]
    fn test_add_and_print() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 65")?,
            parse_instruction("[WhatDoIDo] Print")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        Ok(())
    }

    #[test]
    fn test_name_operations() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhoAmI] Named Alice")?,
            parse_instruction("[WhoAmI] Renamed Bob")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.name, "Bob");
        Ok(())
    }

    #[test]
    fn test_pointer_add_and_sub() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 5")?,
            parse_instruction("[WhereAmI] Add 3")?,
            parse_instruction("[WhatDoIDo] Add 2")?,
            parse_instruction("[WhereAmI] Sub 1")?,
            parse_instruction("[WhatDoIDo] Add 4")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.memory.pointer, 2);

        vm.memory.jump(0);
        assert_eq!(vm.memory.current(), 5);

        vm.memory.jump(1);
        assert_eq!(vm.memory.current(), 0);

        vm.memory.jump(2);
        assert_eq!(vm.memory.current(), 4);
        Ok(())
    }

    #[test]
    fn test_origin_resets_pointer() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhereAmI] Add 5")?,
            parse_instruction("[WhatDoIDo] Add 10")?,
            parse_instruction("[WhereAmI] Origin")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.memory.pointer, 0);
        assert_eq!(vm.memory.current(), 0);
        Ok(())
    }

    #[test]
    fn test_jump_to_cell() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 5")?,
            parse_instruction("[WhereAmI] JumpTo 3")?,
            parse_instruction("[WhatDoIDo] Add 7")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        vm.memory.jump(0);
        assert_eq!(vm.memory.current(), 5);

        vm.memory.jump(3);
        assert_eq!(vm.memory.current(), 7);

        assert_eq!(vm.memory.pointer, 3);
        Ok(())
    }

    #[test]
    fn test_keep_does_nothing() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 5")?,
            parse_instruction("[WhereAmI] Keep")?,
            parse_instruction("[WhatDoIDo] Add 3")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.memory.pointer, 0);
        assert_eq!(vm.memory.current(), 8);
        Ok(())
    }

    #[test]
    fn test_vm_stops_when_pc_out_of_bounds() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![parse_instruction("[WhatDoIDo] Add 5")?];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.pc, 1);
        assert!(vm.running);
        Ok(())
    }

    #[test]
    fn test_complex_sequence() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhoAmI] Named Alice")?,
            parse_instruction("[WhatDoIDo] Add 10")?,
            parse_instruction("[WhereAmI] Add 2")?,
            parse_instruction("[WhatDoIDo] Add 20")?,
            parse_instruction("[WhereAmI] Sub 1")?,
            parse_instruction("[WhatDoIDo] Add 5")?,
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.name, "Alice");
        assert_eq!(vm.memory.pointer, 1);

        vm.memory.jump(0);
        assert_eq!(vm.memory.current(), 10);

        vm.memory.jump(1);
        assert_eq!(vm.memory.current(), 5);
        Ok(())
    }

    // ========== 新增：真正能跑的循环测试 ==========
    #[test]
    fn test_loop_with_enhanced_state() -> Result<(), ThreeThoughtsError> {
        use crate::core::loop_and_condition::LoopState;

        // 打印 0 1 2（循环 3 次）
        // 结构：Loop 指令在 PC 0，循环体在 PC 1~3
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::Loop(LoopState {
                start_pc: 1,
                end_pc: 3,
                times: 3,
                loop_pc: 0, // 这个值会被 VM 覆盖，但为了 struct 完整先填上
            })),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println), // PC 1: 打印当前值
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(1)),  // PC 2: 加 1
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),    // PC 3: 循环体结束标记
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println), // PC 4: 结束后换行
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        // 预期输出：
        // 0
        // 1
        // 2
        // (空行)
        Ok(())
    }

    #[test]
    fn test_nested_loop() -> Result<(), ThreeThoughtsError> {
        use crate::core::loop_and_condition::LoopState;

        // 外层循环 2 次，内层循环 3 次
        // 打印：
        // Outer 0: Inner 0 1 2
        // Outer 1: Inner 0 1 2
        let instructions = vec![
            // ===== 外层循环 =====
            ThreeThoughts::WhatDoIDo(WhatInstruction::Loop(LoopState {
                start_pc: 6,
                end_pc: 18,
                times: 2,
                loop_pc: 0,
            })),
            // PC 1: 外层循环体开始
            ThreeThoughts::WhatDoIDo(WhatInstruction::Print), // 打印 "Outer "
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(1)),
            // ... 这里省略具体打印逻辑，只是结构演示
            // ===== 内层循环 =====
            ThreeThoughts::WhatDoIDo(WhatInstruction::Loop(LoopState {
                start_pc: 10,
                end_pc: 12,
                times: 3,
                loop_pc: 8,
            })),
            // PC 10-12: 内层循环体
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(1)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            // PC 13-18: 外层循环体剩余部分
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            // PC 19: 循环结束后
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }
    // =============================================

    #[test]
    fn test_if_zero() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfZero(3)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(1)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }

    #[test]
    fn test_if_not_zero() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfNotZero(3)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(10)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }

    #[test]
    fn test_if_some() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(10)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfSome(10, 4)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(20)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(30)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }

    #[test]
    fn test_if_some_no_match() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfSome(10, 4)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(20)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(30)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }

    #[test]
    fn test_if_name() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhoAmI(WhoInstruction::Named("Alice".to_string())),
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfName("Alice".to_string(), 4)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(10)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(20)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }

    #[test]
    fn test_if_name_no_match() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhoAmI(WhoInstruction::Named("Alice".to_string())),
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfName("Bob".to_string(), 4)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(10)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(20)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }

    #[test]
    fn test_while_basic() -> Result<(), ThreeThoughtsError> {
        // cell[10]=3, While 10,3,7
        // body: AddOther 11,5 (add 5 to cell[11])
        // loop: JumpTo 2 (back to While)
        // Expected: 3 iterations, cell[11]=15
        let instructions = vec![
            ThreeThoughts::WhereAmI(WhereInstruction::JumpTo(10)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(3)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::While(10, 3, 7)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::AddOther(11, 5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::JumpTo(2)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.memory.cells[11], 15, "3 iters x 5 = 15");
        assert_eq!(vm.memory.cells[10], 0, "counter should reach 0");
        Ok(())
    }

    #[test]
    fn test_while_zero_skips() -> Result<(), ThreeThoughtsError> {
        // cell[10]=0, While immediately exits
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::While(10, 2, 5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::JumpTo(0)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;

        assert_eq!(vm.memory.cells[0], 0, "body should not execute");
        Ok(())
    }

    #[test]
    fn test_conditional_loop() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            ThreeThoughts::WhatDoIDo(WhatInstruction::Add(5)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::IfZero(7)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Sub(1)),
            ThreeThoughts::WhereAmI(WhereInstruction::Origin),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Note),
            ThreeThoughts::WhereAmI(WhereInstruction::JumpTo(1)),
            ThreeThoughts::WhatDoIDo(WhatInstruction::Println),
        ];

        let mut vm = VM::new(instructions, 65536, false);
        vm.run()?;
        Ok(())
    }
}
