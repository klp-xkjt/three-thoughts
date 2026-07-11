#[derive(Debug, PartialEq)]
pub struct Memory {
    pub(crate) cells: Vec<u8>,
    pub(crate) pointer: usize,
}
impl Memory {
    pub fn new(mem: usize) -> Self {
        Memory {
            cells: vec![0; mem],
            pointer: 0,
        }
    }
    pub fn forward(&mut self, add: usize) {
        let new_ptr = self.pointer + add;
        if new_ptr >= self.cells.len() {
            self.expand(new_ptr);
        }
        self.pointer = new_ptr;
    }
    pub fn backward(&mut self, sub: usize) {
        self.pointer = self.pointer.saturating_sub(sub);
    }
    pub fn jump(&mut self, addr: usize) {
        if addr >= self.cells.len() {
            self.expand(addr);
        }
        self.pointer = addr;
    }

    pub fn current(&self) -> u8 {
        self.cells[self.pointer]
    }
    pub fn set_current(&mut self, value: u8) {
        self.cells[self.pointer] = value
    }

    pub fn expand(&mut self, addr: usize) {
        if addr >= self.cells.len() {
            self.cells.resize(addr + 1, 0);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_test() {
        assert_eq!(
            Memory::new(65536),
            Memory {
                cells: vec![0; 65536],
                pointer: 0
            }
        )
    }

    #[test]
    fn ward_test() {
        let mut memory = Memory::new(65536);
        memory.forward(4);
        assert_eq!(memory.pointer, 4);
        memory.backward(2);
        assert_eq!(memory.pointer, 2)
    }

    #[test]
    fn jump() {
        let mut memory = Memory::new(65536);
        memory.jump(10);
        assert_eq!(memory.pointer, 10)
    }

    #[test]
    fn current() {
        let mut memory = Memory::new(65536);
        assert_eq!(memory.current(), 0);
        memory.set_current(10);
        assert_eq!(memory.cells[memory.pointer], 10)
    }

    #[test]
    fn expand_test() {
        let mut memory = Memory::new(65536);
        for _ in 1..65535 {
            memory.set_current(1);
            memory.forward(1);
        }
        memory.forward(1);
        assert_eq!(memory.cells.len(), 65536)
    }
}
