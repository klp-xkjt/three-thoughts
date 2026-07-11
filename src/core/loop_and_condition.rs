#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct LoopState {
    pub start_pc: usize,
    pub end_pc: usize,
    pub times: usize,
}
