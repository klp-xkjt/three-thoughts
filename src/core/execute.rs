use crate::core::error::ThreeThoughtsError;
use crate::core::instruction::*;
use crate::core::memory::Memory;

pub struct VM {
    pub memory: Memory,
    pub pc: usize,
    pub instructions: Vec<ThreeThoughts>,
    pub running: bool,
    pub name: String,
}

impl VM {
    pub fn new(instructions: Vec<ThreeThoughts>) -> Self {
        VM {
            memory: Memory::new(),
            pc: 0,
            instructions,
            running: true,
            name: String::new(),
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
            ThreeThoughts::WhatDoIDo(what) => match *what {
                WhatInstruction::Add(a) => self.memory.cells[self.memory.pointer] += a as u8,
                WhatInstruction::Sub(b) => self.memory.cells[self.memory.pointer] -= b as u8,
                WhatInstruction::Print => {
                    print!("{}", self.memory.cells[self.memory.pointer] as char)
                },
                WhatInstruction::Println => {
                    println!("{}", self.memory.cells[self.memory.pointer] as char)
                },
                WhatInstruction::Note => {

                },
                WhatInstruction::Reset => {
                    self.memory.cells.fill(0);
                }
            },
            ThreeThoughts::None => {}
        }
        Ok(false)
    }

    pub fn run(&mut self) -> Result<(), ThreeThoughtsError> {
        while self.running && self.pc < self.instructions.len() {
            let inst = self.instructions[self.pc].clone();
            let jumped = self.execute_instruction(&inst)?;
            if !jumped {
                self.pc += 1
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::parse_instruction;

    #[test]
    fn test_vm_new() {
        let instructions = vec![];
        let vm = VM::new(instructions);
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
        
        let mut vm = VM::new(instructions);
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
        
        let mut vm = VM::new(instructions);
        vm.run()?;
        
        Ok(())
    }

    #[test]
    fn test_name_operations() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhoAmI] Named Alice")?,
            parse_instruction("[WhoAmI] Renamed Bob")?,
        ];
        
        let mut vm = VM::new(instructions);
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
        

        
        let mut vm = VM::new(instructions);
        println!("Before run: pointer = {}", vm.memory.pointer);
        vm.run()?;
        println!("After run: pointer = {}", vm.memory.pointer);
        
        // 最终指针应该在位置 2
        assert_eq!(vm.memory.pointer, 2);
        
        // 验证位置 0：初始 0 + 5 = 5
        vm.memory.jump(0);
        assert_eq!(vm.memory.current(), 5);
        
        // 验证位置 1：初始 0 + 2 = 2
        vm.memory.jump(1);
        assert_eq!(vm.memory.current(), 0);
        
        // 验证位置 2：初始 0 + 4 = 4
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
        
        let mut vm = VM::new(instructions);
        vm.run()?;
        
        assert_eq!(vm.memory.pointer, 0);
        assert_eq!(vm.memory.current(), 0); 
        Ok(())
    }

    #[test]
    fn test_jump_to_cell() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 5")?,      // pointer=0, cells[0]=5
            parse_instruction("[WhereAmI] JumpTo 3")?,    // 指针跳到位置 3
            parse_instruction("[WhatDoIDo] Add 7")?,      // cells[3]=7
        ];
        
        let mut vm = VM::new(instructions);
        vm.run()?;
        
        // 验证位置 0：5
        vm.memory.jump(0);
        assert_eq!(vm.memory.current(), 5);
        
        // 验证位置 3：7
        vm.memory.jump(3);
        assert_eq!(vm.memory.current(), 7);
        
        // 指针最终在位置 3
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
        
        let mut vm = VM::new(instructions);
        vm.run()?;
        
        // Keep 不移动指针，所以指针还在位置 0
        assert_eq!(vm.memory.pointer, 0);
        assert_eq!(vm.memory.current(), 8);
        Ok(())
    }

    #[test]
    fn test_vm_stops_when_pc_out_of_bounds() -> Result<(), ThreeThoughtsError> {
        let instructions = vec![
            parse_instruction("[WhatDoIDo] Add 5")?,
        ];
        
        let mut vm = VM::new(instructions);
        vm.run()?;
        
        assert_eq!(vm.pc, 1);
        assert!(vm.running);
        Ok(())
    }

    #[test]
    fn test_complex_sequence() -> Result<(), ThreeThoughtsError> {
        // 综合测试：名字 + 指针移动 + 数据操作
        let instructions = vec![
            parse_instruction("[WhoAmI] Named Alice")?,
            parse_instruction("[WhatDoIDo] Add 10")?,
            parse_instruction("[WhereAmI] Add 2")?,
            parse_instruction("[WhatDoIDo] Add 20")?,
            parse_instruction("[WhereAmI] Sub 1")?,
            parse_instruction("[WhatDoIDo] Add 5")?,
        ];
        
        let mut vm = VM::new(instructions);
        vm.run()?;
        
        assert_eq!(vm.name, "Alice");
        assert_eq!(vm.memory.pointer, 1); // 2 - 1 = 1
        
        vm.memory.jump(0);
        assert_eq!(vm.memory.current(), 10); // 位置 0：10
        
        vm.memory.jump(1);
        assert_eq!(vm.memory.current(), 5); // 位置 1：5
        Ok(())
    }
}