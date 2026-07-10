#[derive(Debug, PartialEq, Clone)]
pub enum ThreeThoughts {
    None,
    WhoAmI(WhoInstruction),
    WhereAmI(WhereInstruction),
    WhatDoIDo(WhatInstruction),
}

#[derive(Debug, PartialEq, Clone)]
pub enum WhoInstruction {
    Named(String),   // 命名
    Renamed(String), // 重命名
}

#[derive(Debug, PartialEq, Clone)]
pub enum WhereInstruction {
    Origin,        // 回到 0 单元格
    Keep,          // 保持当前单元格位置
    JumpTo(usize), // 直接跳到某个单元格
    Add(usize),    // 在原单元格位置基础上增加一定数量的单元格
    Sub(usize),    // 在原单元格位置基础上减少一定数量的单元格
}

#[derive(Debug, PartialEq, Clone)]
pub enum WhatInstruction {
    Add(usize), // 单元格值增加一定值
    Sub(usize), // 单元格值减少一定值
    Print,      // 输出当前单元格的值（不换行）
    Println,    // 输出当前单元格的值（换行）
    Note,       // 注释    
    Reset       // 重置
}
