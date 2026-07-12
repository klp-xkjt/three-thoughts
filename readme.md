# 3Thoughts

用哲学三问来编写代码的玩具编程语言。

> 我是谁？我在哪？我要做什么？—— 回答这三个问题，就能写程序。

## 安装
> 安装本语言解释器需要 Rust 环境
### 使用 `cargo install` 安装

```
cargo install ts3
```

### 编译项目并安装
```bash
git clone https://github.com/klp-xkjt/three-thoughts
cd three-thoughts
cargo build --release
```

## 使用

```bash
# 运行 .ts3 文件
cargo run -- run examples/hello_world.ts3

# 指定内存大小（默认 65536 字节）
cargo run -- run examples/demo.ts3 --mem 1024

# 开启调试模式，逐条打印指令执行过程
cargo run -- run examples/loop.ts3 --debug
```

## 语言设计

3Thoughts 是一门基于**内存单元 + 指针**的 esolang。程序由三类指令组成，对应三个哲学问题：

### [WhoAmI] — 我是谁

| 指令 | 参数 | 说明 |
|------|------|------|
| `Named` | name | 给 VM 命名 |
| `Renamed` | name | 重命名 VM |

### [WhereAmI] — 我在哪

| 指令 | 参数 | 说明 |
|------|------|------|
| `Origin` | — | 指针回到 0 号单元 |
| `Keep` | — | 指针不动 |
| `JumpTo` | addr | 指针跳到指定地址 |
| `Add` | n | 指针前进 n 个单元 |
| `Sub` | n | 指针后退 n 个单元 |

### [WhatDoIDo] — 我要做什么

| 指令 | 参数 | 说明 |
|------|------|------|
| `Add` | n | 当前单元值 +n |
| `Sub` | n | 当前单元值 -n |
| `Print` | — | 输出当前单元值（as char，不换行） |
| `Println` | — | 输出当前单元值（as char，换行） |
| `Note` | — | 注释，不执行任何操作 |
| `Reset` | — | 所有单元清零 |
| `ResetOrigin` | — | 所有单元清零，指针归零 |
| `Loop` | start,end,times | 循环 times 次，循环体为 PC [start, end] |
| `JumpTo` | pc | 无条件跳转到 PC |
| `IfZero` | pc | 当前单元 == 0 则跳转 |
| `IfNotZero` | pc | 当前单元 != 0 则跳转 |
| `IfSome` | val,pc | 当前单元 == val 则跳转 |
| `IfNotSome` | val,pc | 当前单元 != val 则跳转 |
| `IfName` | name,pc | VM 名称为 name 则跳转 |
| `Panic` | msg | 运行时 panic，输出错误信息 |
| `AddOther` | addr,n | cell[addr] += n（原地，不经过指针） |
| `SubOther` | addr,n | cell[addr] -= n |
| `Free` | — | 当前 cell 清零 |
| `FreeOther` | addr | cell[addr] 清零 |
| `GetOther` | addr | cell[ptr] = cell[addr]（从远处抄到当前） |
| `Read` | addr | 从 stdin 读 1 字节 → cell[addr] |
| `ReadASCII` | addr | 读十进制 ASCII 码 → cell[addr]（例: 输入 65 → 存 'A'） |
| `Dump` | start,end | 打印 cells[start..end] 的地址:值（调试用） |
| `Reverse` | start,end | 原地反转 cells[start..end] |

`//` 开头的行为注释，`//` 之后的内容在行内也会被忽略。

## 示例

### Hello World

```
[WhatDoIDo] Add 72      // 'H'
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 101     // 'e'
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108     // 'l'
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108     // 'l'
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 111     // 'o'
[WhatDoIDo] Print
```

### 循环

```
[WhatDoIDo] Add 48      // 初始化为 ASCII '0'
[WhatDoIDo] Loop 3,5,3  // 循环 3 次，循环体 PC 3~5
[WhatDoIDo] Println     // 打印当前字符
[WhatDoIDo] Add 1       // +1 变成下一个字符
[WhatDoIDo] Note        // 循环体结束标记
// 输出: 0 1 2
```

### 条件判断 + 倒计时

```
[WhatDoIDo] Add 51      // '3'
[WhatDoIDo] Println     // 3
[WhatDoIDo] Sub 1
[WhatDoIDo] Println     // 2
[WhatDoIDo] Sub 1
[WhatDoIDo] Println     // 1
[WhatDoIDo] Sub 1
[WhatDoIDo] Println     // 0
[WhatDoIDo] IfSome 48,10  // 如果是 '0'，跳到 Done
[WhatDoIDo] JumpTo 12     // 否则跳过
[WhatDoIDo] Note        // Done: 打印 !
[WhatDoIDo] Add 33
[WhatDoIDo] Println
// 输出: 3 2 1 0 !
```

### Fibonacci 数列

```
[WhereAmI] Origin           // cell[0]=0 (F0)
[WhereAmI] Add 1
[WhatDoIDo] Add 1           // cell[1]=1 (F1)
[WhatDoIDo] Loop 8,43,10    // 10 次迭代
// --- 循环体 ---
[WhatDoIDo] Dump 0,1        // 输出当前 cell[0] 的值
// temp = a + b (cell[2] = cell[0] + cell[1])
[WhereAmI] JumpTo 2; [WhatDoIDo] GetOther 0
[WhereAmI] JumpTo 3; [WhatDoIDo] GetOther 1
// 内层循环: AddOther 2,1 / SubOther 3,1 (累加)
// b = temp: FreeOther 1; 内层循环 AddOther 1,1
// a = old_b: FreeOther 0; 内层循环 AddOther 0,1
...
// 输出: 0 1 1 2 3 5 8 13 21 34 55 89
```

更多示例见 [`examples/`](examples/)。

## 项目结构

```
src/
├── main.rs                      # CLI 入口
└── core/
    ├── error.rs                 # 错误类型
    ├── instruction.rs           # 指令定义
    ├── loop_and_condition.rs    # 循环状态
    ├── memory.rs                # 内存模型
    ├── parser.rs                # 解析器
    └── execute.rs               # 虚拟机
examples/                        # 示例程序
```

## 许可

MIT
