use crate::core::loop_and_condition::LoopState;

#[derive(Debug, PartialEq, Clone)]
pub enum ThreeThoughts {
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
    AddOther(usize, usize),
    SubOther(usize, usize),
    Sub(usize), // 单元格值减少一定值
    Free,                    // 释放当前单元格的内存
    FreeOther(usize),        // 释放指定单元格的内存
    Loop(LoopState),         // 有限次数循环体
    Print,                   // 输出当前单元格的值（不换行）
    Println,                 // 输出当前单元格的值（换行）
    Note,                    // 注释
    Reset,                   // 重置
    ResetOrigin,             // 重置并使指针回到初始单元格
    IfZero(usize),           // 如果当前单元格是 0，跳转到 PC
    IfNotZero(usize),        // 如果当前单元格不是 0，跳转
    IfSome(usize, usize),    // 如果当前单元格是某一数值，跳转
    IfNotSome(usize, usize), // 如果当前单元格不是某一数值，跳转
    IfName(String, usize),   // 如果名字匹配，跳转
    JumpTo(usize),           // 直接跳转 PC
    Panic(String),           // 强行使程序 Panic,
    Read(usize),             // 读取单个字节于某一地址
    ReadASCII(usize),        // 读ASCII十进制码并转为ASCLL字符，存于指定地址
    Dump(usize, usize),      // 调试输出单元格
    DumpWN(usize, usize),      // 调试输出单元格，带有单元格
    Reverse(usize, usize),   // 反转单元格的值
    GetOther(usize),         // 获得指定单元格值于本单元格
    While(usize, usize, usize),
    Copy(usize, usize)
}
