use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum ThreeThoughtsError {
    // ═══════════════════════════════════════════
    // 解析错误
    // ═══════════════════════════════════════════
    /// 指令前缀无法识别（不是 [WhoAmI] / [WhereAmI] / [WhatDoIDo]）
    #[error("无法识别的指令前缀，期望 [WhoAmI]、[WhereAmI] 或 [WhatDoIDo]")]
    UnknownCategory,

    /// 未知的 WhoAmI 子指令
    #[error("未知的 WhoAmI 指令: '{0}'，可用: Named, Renamed")]
    UnknownWhoInstruction(String),

    /// WhoAmI Named/Renamed 缺少名称参数
    #[error("WhoAmI {0} 缺少名称参数，例如: [WhoAmI] Named MyName")]
    MissingWhoName(String),

    /// 未知的 WhereAmI 子指令
    #[error("未知的 WhereAmI 指令: '{0}'，可用: Origin, Keep, JumpTo, Add, Sub")]
    UnknownWhereInstruction(String),

    /// WhereAmI 数值参数解析失败
    #[error("WhereAmI {instruction} 参数无效: '{arg}' 不是有效的数值")]
    InvalidWhereArg { instruction: String, arg: String },

    /// 未知的 WhatDoIDo 子指令
    #[error("未知的 WhatDoIDo 指令: '{0}'")]
    UnknownWhatInstruction(String),

    /// WhatDoIDo 数值参数解析失败
    #[error("WhatDoIDo {instruction} 参数无效: '{arg}' 不是有效的数值")]
    InvalidWhatArg { instruction: String, arg: String },

    /// Loop 指令参数不完整（期望 3 个逗号分隔的数值）
    #[error("Loop 指令参数不完整: 需要 start_pc,end_pc,times 共 3 个参数，实际提供了 {0} 个")]
    IncompleteLoopArgs(usize),

    /// IfName 指令参数不完整（期望 2 个：名称和 PC）
    #[error("IfName 指令参数不完整: 需要 name,pc 共 2 个参数，实际提供了 {0} 个")]
    IncompleteIfNameArgs(usize),

    /// IfName 指令参数不完整（期望 2 个：名称和 PC）
    #[error("{0} 指令参数不完整: 需要 {1},{2} 共 2 个参数，实际提供了 {3} 个")]
    IncompleteAnyCoupleArgs(String, String, String, usize),

    /// 程序中没有有效指令
    #[error("程序为空，没有找到任何有效指令")]
    EmptyProgram,

    // ═══════════════════════════════════════════
    // 运行时错误
    // ═══════════════════════════════════════════
    /// PC 越界跳转
    #[error("PC 越界: 跳转到 #{pc}，但程序只有 {total} 条指令")]
    PcOutOfBounds { pc: usize, total: usize },

    /// 循环次数无效（0 或溢出）
    #[error("循环次数无效: times={0}，必须 >= 1")]
    InvalidLoopTimes(usize),

    /// 循环嵌套过深
    #[error("循环嵌套过深: 超过最大嵌套层数 {0}")]
    LoopNestingTooDeep(usize),

    /// 循环栈异常（栈顶状态与当前 Loop 不匹配）
    #[error("循环栈状态异常: {0}")]
    LoopStackCorruption(String),

    /// 内存访问越界
    #[error("内存访问越界: 地址 {addr}，最大地址 {max}")]
    MemoryOutOfBounds { addr: usize, max: usize },

    /// 通用运行时错误
    #[error("运行时错误: {0}")]
    RuntimeError(String),
}
