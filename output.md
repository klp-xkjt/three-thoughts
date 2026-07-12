# 代码汇总

## 📊 统计信息

- **扫描总数**: 34
- **包含文件**: 20
- **排除总数**: 14

---

## 📑 目录

1. [.gitignore](#file-0)
2. [License](#file-1)
3. [examples\condition.ts3](#file-2)
4. [examples\demo.ts3](#file-3)
5. [examples\hello_world.ts3](#file-4)
6. [examples\if_zero.ts3](#file-5)
7. [examples\loop.ts3](#file-6)
8. [examples\overview.ts3](#file-7)
9. [examples\panic.ts3](#file-8)
10. [examples\read.ts3](#file-9)
11. [examples\ts3.ts3](#file-10)
12. [src\core\error.rs](#file-11)
13. [src\core\execute.rs](#file-12)
14. [src\core\instruction.rs](#file-13)
15. [src\core\loop_and_condition.rs](#file-14)
16. [src\core\memory.rs](#file-15)
17. [src\core\mod.rs](#file-16)
18. [src\core\parser.rs](#file-17)
19. [src\lib.rs](#file-18)
20. [src\main.rs](#file-19)

---

<div id="file-0"></div>

## 📄 .gitignore

```
/target
```

<div id="file-1"></div>

## 📄 License

```
Copyright © 2026 <klp-xkjt>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

<div id="file-2"></div>

## 📄 examples\condition.ts3

```ts3
[WhatDoIDo] Note === 倒计时 5 到 0 ===

[WhatDoIDo] Add 53

[WhatDoIDo] IfSome 48,6
[WhatDoIDo] Println
[WhatDoIDo] Sub 1
[WhatDoIDo] JumpTo 2
[WhatDoIDo] Println
```

<div id="file-3"></div>

## 📄 examples\demo.ts3

```ts3
[WhatDoIDo] Note ===== 3Thoughts Demo =====
// PC 0: 程序标题（注释，不执行任何操作）

[WhatDoIDo] Reset
// PC 1: 清零所有内存单元，ptr 归 0

// ═══════════════════════════════════════════════
// Section 1: 用 Loop 打印 "LOOP:" 五个字符
// 策略：将 5 个 ASCII 码存入 cell[0]~cell[4]，
//       然后用 Loop 遍历打印
// ═══════════════════════════════════════════════

[WhatDoIDo] Add 76
// PC 2:  cell[0] = 76  = 'L'
[WhereAmI] Add 1
// PC 3:  ptr = 1
[WhatDoIDo] Add 79
// PC 4:  cell[1] = 79  = 'O'
[WhereAmI] Add 1
// PC 5:  ptr = 2
[WhatDoIDo] Add 79
// PC 6:  cell[2] = 79  = 'O'
[WhereAmI] Add 1
// PC 7:  ptr = 3
[WhatDoIDo] Add 80
// PC 8:  cell[3] = 80  = 'P'
[WhereAmI] Add 1
// PC 9:  ptr = 4
[WhatDoIDo] Add 58
// PC 10: cell[4] = 58  = ':'
[WhereAmI] Origin
// PC 11: ptr 回到 0（准备从头遍历打印）

[WhatDoIDo] Loop 13,15,5
// PC 12: 循环 5 次，循环体 = PC 13~15
//        首次：压栈 {start:13, end:15, times:5, loop_pc:12} → 跳到 PC 13
//        后续：times 递减，归零时弹出栈 → 跳到 PC 15+1=16

[WhatDoIDo] Print
// PC 13: [循环体] 打印当前单元格字符（as char）

[WhereAmI] Add 1
// PC 14: [循环体] ptr++，移到下一个字符

[WhatDoIDo] Note
// PC 15: [循环体结束] 空操作，仅标记循环尾
//        执行后 PC→16 > end_pc(15)，run() 检测到越界 → 跳回 loop_pc(12)

[WhereAmI] Add 1
// PC 16: ptr = 5（跳到干净单元格，避开已用数据）
[WhatDoIDo] Add 10
// PC 17: cell[5] = 10 = '\n'（换行符）
[WhatDoIDo] Print
// PC 18: 打印换行符 → 输出 "LOOP:\n"

// ═══════════════════════════════════════════════
// Section 2: 用 Loop 打印 "ABC" 三个字符
// 策略：从 'A'(65) 开始，每次循环打印后 +1
// ═══════════════════════════════════════════════

[WhereAmI] Origin
// PC 19: ptr 回到 0
[WhatDoIDo] Reset
// PC 20: 清零所有内存（避免 Section 1 的残留值干扰）
[WhatDoIDo] Add 65
// PC 21: cell[0] = 65 = 'A'

[WhatDoIDo] Loop 23,25,3
// PC 22: 循环 3 次，循环体 = PC 23~25

[WhatDoIDo] Print
// PC 23: [循环体] 打印当前字符 → 'A' → 'B' → 'C'
[WhatDoIDo] Add 1
// PC 24: [循环体] cell[0] += 1（'A'→'B'→'C'→'D'）
[WhatDoIDo] Note
// PC 25: [循环体结束] PC→26 > end_pc(25) → 跳回 loop_pc(22)

[WhereAmI] Add 1
// PC 26: ptr = 1（干净单元格）
[WhatDoIDo] Add 10
// PC 27: cell[1] = 10 = '\n'
[WhatDoIDo] Print
// PC 28: 打印换行符 → 输出 "ABC\n"

// ═══════════════════════════════════════════════
// Section 3: 倒计时 3→2→1→0  +  条件判断打印 "!"
// 策略：从 '3'(51) 开始，逐次 -1 并打印，
//       打印完 '0' 后用 IfSome 检测并跳到 Done
// ═══════════════════════════════════════════════

[WhereAmI] Origin
// PC 29: ptr 回到 0
[WhatDoIDo] Reset
// PC 30: 清零所有内存
[WhatDoIDo] Add 51
// PC 31: cell[0] = 51 = '3'

[WhatDoIDo] Println
// PC 32: 打印 '3' + 换行 → "3\n"
[WhatDoIDo] Sub 1
// PC 33: cell[0] = 50 = '2'
[WhatDoIDo] Println
// PC 34: 打印 '2' + 换行 → "2\n"
[WhatDoIDo] Sub 1
// PC 35: cell[0] = 49 = '1'
[WhatDoIDo] Println
// PC 36: 打印 '1' + 换行 → "1\n"
[WhatDoIDo] Sub 1
// PC 37: cell[0] = 48 = '0'
[WhatDoIDo] Println
// PC 38: 打印 '0' + 换行 → "0\n"

[WhatDoIDo] IfSome 48,42
// PC 39: 如果 cell[0] == 48 ('0') → 跳到 PC 42（Done 分支）
//        条件成立！跳转到 PC 42

[WhatDoIDo] JumpTo 48
// PC 40: 无条件跳转到结束（兜底逻辑，正常情况不会执行到此处）

[WhatDoIDo] Note
// PC 41: 占位（IfSome 跳转目标原本指向此处之前的 PC 42）

// ── Done! 分支：打印 "!\n" ──

[WhereAmI] Add 1
// PC 42: ptr = 1（干净单元格，cell[1]=0）
[WhatDoIDo] Add 33
// PC 43: cell[1] = 33 = '!'
[WhatDoIDo] Print
// PC 44: 打印 '!'
[WhereAmI] Add 1
// PC 45: ptr = 2
[WhatDoIDo] Add 10
// PC 46: cell[2] = 10 = '\n'
[WhatDoIDo] Print
// PC 47: 打印换行符 → 输出 "!\n"
//         程序结束（PC 48 >= 48 条指令，run() 退出）
```

<div id="file-4"></div>

## 📄 examples\hello_world.ts3

```ts3
[WhatDoIDo] Note This-is-a-"Hello-World!"-of-3Thoughts
[WhatDoIDo] Add 72
[WhatDoIDo] Print
[WhereAmI] Add 1  
[WhatDoIDo] Add 101
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 111
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 32
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 87
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 111
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 114
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 100
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 33
[WhatDoIDo] Println

[WhatDoIDo] Note This-is-a-"Hello-World!" reappearance
[WhatDoIDo] Reset
[WhatDoIDo] Add 72
[WhatDoIDo] Print
[WhereAmI] Add 1  
[WhatDoIDo] Add 101
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 111
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 32
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 87
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 111
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 114
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 100
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 33
[WhatDoIDo] Println
```

<div id="file-5"></div>

## 📄 examples\if_zero.ts3

```ts3
[WhatDoIDo] Note === IfZero 测试 ===

[WhatDoIDo] Add 40
[WhatDoIDo] IfZero 6
[WhatDoIDo] Println
[WhatDoIDo] Sub 1
[WhatDoIDo] JumpTo 2
[WhatDoIDo] Println

[WhatDoIDo] ResetOrigin
```

<div id="file-6"></div>

## 📄 examples\loop.ts3

```ts3
[WhatDoIDo] Note === 打印 0 到 2 ===
[WhatDoIDo] Add 48        // PC 1: 初始化为 ASCII '0'
[WhatDoIDo] Loop 3,5,3    // PC 2: 循环头（3次）
[WhatDoIDo] Println       // PC 3: 循环体开始 (start_pc=3)
[WhatDoIDo] Add 1         // PC 4: 加1
[WhatDoIDo] Note          // PC 5: 循环体结束 (end_pc=5)
```

<div id="file-7"></div>

## 📄 examples\overview.ts3

```ts3
// ============================================================
// 3Thoughts 综合示例 — 覆盖大部分指令
//
// 输出: HI \n 3 \n 2 \n 1 \n ! \n
// ============================================================

[WhatDoIDo] Note ===== Section 1: 身份与问候 =====
// PC 0

[WhoAmI] Named TS3
// PC 1: VM 名称为 "TS3"

// --- 在 cell[0] 和 cell[1] 存入 "HI" ---
[WhatDoIDo] Add 72
// PC 2:  cell[0] = 72 = 'H'
[WhereAmI] Add 1
// PC 3:  ptr = 1
[WhatDoIDo] Add 73
// PC 4:  cell[1] = 73 = 'I'
[WhereAmI] Origin
// PC 5:  ptr = 0，准备循环打印

[WhatDoIDo] Loop 7,9,2
// PC 6:  循环 2 次，循环体 PC 7~9
//        首次: 压栈 {start:7, end:9, times:2, loop_pc:6} → 跳到 PC 7
//        每次迭代结束: PC>9 → 跳回 loop_pc(6)，times--
//        times=0: 弹出栈 → pc=end_pc(9)，run() +1 → PC 10

[WhatDoIDo] Print
// PC 7:  [循环体] 打印当前 cell → 'H' → 'I'
[WhereAmI] Add 1
// PC 8:  [循环体] ptr++，移到下一个字符
[WhatDoIDo] Note
// PC 9:  [循环体结束] PC→10 > end_pc(9) → 跳回 loop_pc(6)

// --- 换行分隔 ---
[WhereAmI] Add 1
// PC 10: ptr = 2（干净 cell）
[WhatDoIDo] Add 10
// PC 11: cell[2] = 10 = '\n'
[WhatDoIDo] Println
// PC 12: 打印 '\n' + 换行（等价于空行）


[WhatDoIDo] Note ===== Section 2: 条件分支 =====
// PC 13

[WhereAmI] Origin
// PC 14: ptr 回到 0
[WhatDoIDo] Reset
// PC 15: 清空所有 cell

[WhatDoIDo] IfName TS3,18
// PC 16: name == "TS3" → 跳到 PC 18（继续执行）
//        条件成立 — PC 1 已命名为 TS3

[WhatDoIDo] Panic Identity lost!
// PC 17: （被跳过）仅 name 不匹配时触发


[WhatDoIDo] Note ===== Section 3: 倒计时 3-2-1 =====
// PC 18

[WhatDoIDo] Add 3
// PC 19: cell[0] = 3（数值，非 ASCII）

// ==== While 循环: cell[0] != 0 时重复 ====
[WhatDoIDo] Note
// PC 20: LOOP_START — 循环入口

[WhatDoIDo] IfZero 33
// PC 21: cell[0] == 0 → 跳到 PC 33（退出循环）
//        否则继续往下

// --- 循环体: 数字→ASCII→打印→恢复→换行 ---
[WhatDoIDo] Add 48
// PC 22: 数值→ASCII: 3→'3'(51), 2→'2'(50), 1→'1'(49)
[WhatDoIDo] Print
// PC 23: 打印数字字符
[WhatDoIDo] Sub 48
// PC 24: ASCII→数值: 恢复为 3,2,1
[WhereAmI] Add 1
// PC 25: ptr = 1
[WhatDoIDo] Add 10
// PC 26: cell[1] += 10 → 10 = '\n'（第一次为 0+10，后续先 Sub 10 清零再加）
[WhatDoIDo] Print
// PC 27: 打印 '\n'
[WhatDoIDo] Sub 10
// PC 28: cell[1] -= 10 → 归零，避免下次迭代累加
[WhereAmI] Origin
// PC 29: ptr 回到 0
[WhatDoIDo] Sub 1
// PC 30: cell[0] -= 1（3→2→1→0）

[WhatDoIDo] JumpTo 20
// PC 31: 无条件跳回 LOOP_START（PC 20）

[WhatDoIDo] Note
// PC 32: （跳转越过了这里）


[WhatDoIDo] Note ===== Section 4: 结束 =====
// PC 33: 循环出口 — cell[0] == 0 时跳到这里

[WhatDoIDo] ResetOrigin
// PC 34: 清空所有 cell + ptr 归零

[WhereAmI] Add 1
// PC 35: ptr = 1
[WhatDoIDo] Add 33
// PC 36: cell[1] = 33 = '!'
[WhatDoIDo] Print
// PC 37: 打印 '!'

[WhereAmI] Add 1
// PC 38: ptr = 2
[WhatDoIDo] Add 10
// PC 39: cell[2] = 10 = '\n'
[WhatDoIDo] Print
// PC 40: 打印 '\n'
//         程序结束（PC 41 >= 41 条指令，run() 退出）


// ════════════════════════════════════════════════════════
// 本示例使用的指令（15/22）:
//
// WhoAmI(1):  Named
// WhereAmI(2): Origin, Add
// WhatDoIDo(12): Add, Sub, Loop, Print, Println, Note,
//                Reset, ResetOrigin, IfZero, JumpTo,
//                IfName, Panic
//
// 未展示（7）:
//   Renamed, Keep, JumpTo, Sub(WhereAmI)
//   IfNotZero, IfSome, IfNotSome
//   Read, ReadASCII（需要 stdin 交互）
// ════════════════════════════════════════════════════════
```

<div id="file-8"></div>

## 📄 examples\panic.ts3

```ts3
[WhatDoIDo] Add 72
[WhatDoIDo] Print
[WhereAmI] Add 1  
[WhatDoIDo] Add 101
[WhatDoIDo] Print
[WhereAmI] Add 1
[WhatDoIDo] Add 108
[WhatDoIDo] Print
[WhatDoIDo] Panic PaincTest
```

<div id="file-9"></div>

## 📄 examples\read.ts3

```ts3
// ============================================================
// 输入系统测试 — 演示 Read / ReadASCII
//
// 用法: cargo run -- run examples/read.ts3
// 然后依次输入:
//   第一行: 随便一个字符 (如 A)
//   第二行: 随便一个字符 (如 B)
//   第三行: 一个十进制 ASCII 码 (如 33 表示 '!')
// ============================================================

[WhatDoIDo] Note ===== Step 1: Read 原始字节 1 =====
// PC 0

[WhatDoIDo] Read 0
// PC 1: 从 stdin 读 1 字节 → cell[0]
[WhatDoIDo] Read 9
// PC 2: 再读 1 字节吞掉行尾 \n（扔进 cell[9]，不影响程序）
[WhereAmI] Origin
// PC 3: ptr = 0
[WhatDoIDo] Print
// PC 4: 回显该字节的字符
[WhereAmI] Add 1
// PC 5: ptr = 1
[WhatDoIDo] Add 10
// PC 6: cell[1] = 10 = '\n'
[WhatDoIDo] Print
// PC 7: 换行


[WhatDoIDo] Note ===== Step 2: Read 原始字节 2 =====
// PC 8

[WhereAmI] Origin
// PC 9: ptr = 0
[WhatDoIDo] Read 1
// PC 10: 从 stdin 读 1 字节 → cell[1]
[WhatDoIDo] Read 9
// PC 11: 吞掉行尾 \n
[WhereAmI] Add 1
// PC 12: ptr = 1
[WhatDoIDo] Print
// PC 13: 回显
[WhereAmI] Add 1
// PC 14: ptr = 2
[WhatDoIDo] Add 10
// PC 15: cell[2] = '\n'
[WhatDoIDo] Print
// PC 16: 换行


[WhatDoIDo] Note ===== Step 3: ReadASCII 十进制 ASCII 码 =====
// PC 17

[WhereAmI] Origin
// PC 18: ptr = 0
[WhatDoIDo] ReadASCII 2
// PC 19: 读十进制数 → ASCII 字符 → cell[2]
//        例如输入 "65" → cell[2] = 65 = 'A'
//        ReadASCII 自动跳过前导 \n，不需要手动吞

[WhereAmI] JumpTo 2
// PC 20: ptr 直接跳到 cell[2]
[WhatDoIDo] Print
// PC 21: 打印该 ASCII 码对应的字符
[WhereAmI] Add 1
// PC 22: ptr = 3
[WhatDoIDo] Add 10
// PC 23: cell[3] = '\n'
[WhatDoIDo] Print
// PC 24: 换行


[WhatDoIDo] Note ===== Step 4: 条件判断 — 输入是否有效 =====
// PC 25

[WhereAmI] JumpTo 2
// PC 26: ptr = 2，检查 cell[2]
[WhatDoIDo] IfNotZero 30
// PC 27: cell[2] != 0（有有效输入）→ 跳到 PC 30（继续）
//        如果没输入或输入 "0"，cell[2] == 0 → 走到 Panic

[WhatDoIDo] Panic No valid ASCII code entered!
// PC 28: 输入无效时终止

[WhatDoIDo] Note
// PC 29: 占位


[WhatDoIDo] Note ===== Step 5: 打印分隔线 "---" =====
// PC 30

[WhereAmI] Origin
// PC 31: ptr = 0
[WhatDoIDo] Reset
// PC 32: 清空 cell，避免残留值干扰 Add
[WhatDoIDo] Add 45
// PC 33: cell[0] = 45 = '-'

[WhatDoIDo] Loop 35,37,3
// PC 34: 循环 3 次，循环体 PC 35~37

[WhatDoIDo] Print
// PC 35: [循环体] 打印 '-' → "---"
[WhereAmI] Keep
// PC 36: [循环体] ptr 不动（演示 Keep）
[WhatDoIDo] Note
// PC 37: [循环体结束]

[WhereAmI] Add 1
// PC 38: ptr = 1
[WhatDoIDo] Add 10
// PC 39: cell[1] = '\n'
[WhatDoIDo] Print
// PC 40: 换行


[WhatDoIDo] Note ===== Step 6: IfNotSome 检查 + 结束 =====
// PC 41

[WhereAmI] Origin
// PC 42: ptr = 0
[WhatDoIDo] IfNotSome 45,48
// PC 43: 如果 cell[0] != 45 ('-') → 跳到 PC 48 (Panic)
//        如果 cell[0] == 45 → 继续往下（正常路径）

[WhereAmI] JumpTo 4
// PC 44: ptr = 4（干净 cell）
[WhatDoIDo] Add 33
// PC 45: cell[4] = 33 = '!'
[WhatDoIDo] Println
// PC 46: 打印 "!\n"

[WhatDoIDo] JumpTo 49
// PC 47: 跳过 PC 48 的 Panic → 正常退出

[WhatDoIDo] Panic Separator corrupted!
// PC 48: （仅 IfNotSome 跳转至此 — cell[0] 不是 '-'）

[WhatDoIDo] Note
// PC 49: 程序结束


// ════════════════════════════════════════════════════════
// 本示例使用的指令（17/22）:
//
// WhereAmI(5):  Origin, Add, JumpTo, Keep
// WhatDoIDo(12): Add, Print, Println, Note, Loop,
//                Read, ReadASCII, Reset,
//                IfNotZero, IfNotSome, JumpTo, Panic
//
// 未展示（5）:
//   Named, Renamed, Sub(WhereAmI), Sub(WhatDoIDo)
//   IfZero, IfSome, IfName, ResetOrigin
// ════════════════════════════════════════════════════════
```

<div id="file-10"></div>

## 📄 examples\ts3.ts3

```ts3
[WhoAmI] Named Rustacean
// PC 0: 设置 VM 名称为 "Rustacean"

[WhereAmI] Origin
// PC 1: ptr = 0

[WhereAmI] Add 1
// PC 2: ptr = 1
[WhatDoIDo] Add 65
// PC 3: cell[1] = 65 = 'A'
[WhereAmI] Add 1
// PC 4: ptr = 2
[WhatDoIDo] Add 66
// PC 5: cell[2] = 66 = 'B'
[WhereAmI] Add 1
// PC 6: ptr = 3
[WhatDoIDo] Add 67
// PC 7: cell[3] = 67 = 'C'

[WhereAmI] Origin
// PC 8: ptr 回到 0，准备循环

[WhatDoIDo] Loop 10,17,3
// PC 9: 循环 3 次，循环体 = PC 10~17
//       打印 cell[0]~[3] → NUL + A + B + C（重复 3 次）

[WhereAmI] Origin
// PC 10: [循环体] ptr = 0
[WhatDoIDo] Print
// PC 11: [循环体] 打印 cell[0]（NUL，不可见）
[WhereAmI] Add 1
// PC 12: [循环体] ptr = 1
[WhatDoIDo] Print
// PC 13: [循环体] 打印 cell[1] = 'A'
[WhereAmI] Add 1
// PC 14: [循环体] ptr = 2
[WhatDoIDo] Print
// PC 15: [循环体] 打印 cell[2] = 'B'
[WhereAmI] Add 1
// PC 16: [循环体] ptr = 3（修复：原文件缺少此行，导致最后两个 Print 重复打印同一 cell）
[WhatDoIDo] Print
// PC 17: [循环体] 打印 cell[3] = 'C'
//         执行后 PC→18 > end_pc(17) → 跳回 loop_pc(9)

[WhereAmI] Origin
// PC 18: ptr 回到 0

[WhatDoIDo] IfName Rustacean,21
// PC 19: 如果 name == "Rustacean" → 跳到 PC 21
//        条件成立（PC 0 设置了 Rustacean）→ 跳过 PC 20

[WhatDoIDo] Println
// PC 20: （被跳过）仅 name 不匹配时才执行

[WhatDoIDo] Println
// PC 21: 换行，程序结束
```

<div id="file-11"></div>

## 📄 src\core\error.rs

```rs
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

    /// IfSome/IfNotSome 指令参数不完整（期望 2 个）
    #[error("{0} 指令参数不完整: 需要 value,pc 共 2 个参数，实际提供了 {1} 个")]
    IncompleteIfArgs(String, usize),

    /// IfName 指令参数不完整（期望 2 个：名称和 PC）
    #[error("IfName 指令参数不完整: 需要 name,pc 共 2 个参数，实际提供了 {0} 个")]
    IncompleteIfNameArgs(usize),

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
```

<div id="file-12"></div>

## 📄 src\core\execute.rs

```rs
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
                WhatInstruction::Sub(b) => {
                    self.memory.cells[self.memory.pointer] =
                        self.memory.cells[self.memory.pointer].wrapping_sub(*b as u8);
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
```

<div id="file-13"></div>

## 📄 src\core\instruction.rs

```rs
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
    Sub(usize), // 单元格值减少一定值
    Loop(LoopState),
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
    ReadASCII(usize),
}
```

<div id="file-14"></div>

## 📄 src\core\loop_and_condition.rs

```rs
#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct LoopState {
    pub start_pc: usize,
    pub end_pc: usize,
    pub times: usize,
    pub loop_pc: usize,
}
```

<div id="file-15"></div>

## 📄 src\core\memory.rs

```rs
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
```

<div id="file-16"></div>

## 📄 src\core\mod.rs

```rs
pub mod error;
pub mod execute;
pub mod instruction;
pub mod loop_and_condition;
pub mod memory;
pub mod parser;
```

<div id="file-17"></div>

## 📄 src\core\parser.rs

```rs
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
        "Sub" => Ok(WhatInstruction::Sub(parse_usize("Sub")?)),
        "Note" => Ok(WhatInstruction::Note),
        "Reset" => Ok(WhatInstruction::Reset),
        "ResetOrigin" => Ok(WhatInstruction::ResetOrigin),
        "IfZero" => Ok(WhatInstruction::IfZero(parse_usize("IfZero")?)),
        "IfNotZero" => Ok(WhatInstruction::IfNotZero(parse_usize("IfNotZero")?)),
        "IfSome" => {
            let rest = get_rest(str, 2);
            let vals: Vec<&str> = get_values(&rest)
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect();
            if vals.len() < 2 {
                return Err(ThreeThoughtsError::IncompleteIfArgs(
                    "IfSome".into(),
                    vals.len(),
                ));
            }
            let value =
                vals[0]
                    .parse::<usize>()
                    .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                        instruction: "IfSome".into(),
                        arg: vals[0].into(),
                    })?;
            let pc = vals[1]
                .parse::<usize>()
                .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                    instruction: "IfSome".into(),
                    arg: vals[1].into(),
                })?;
            Ok(WhatInstruction::IfSome(value, pc))
        }
        "IfNotSome" => {
            let rest = get_rest(str, 2);
            let vals: Vec<&str> = get_values(&rest)
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect();
            if vals.len() < 2 {
                return Err(ThreeThoughtsError::IncompleteIfArgs(
                    "IfNotSome".into(),
                    vals.len(),
                ));
            }
            let value =
                vals[0]
                    .parse::<usize>()
                    .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                        instruction: "IfNotSome".into(),
                        arg: vals[0].into(),
                    })?;
            let pc = vals[1]
                .parse::<usize>()
                .map_err(|_| ThreeThoughtsError::InvalidWhatArg {
                    instruction: "IfNotSome".into(),
                    arg: vals[1].into(),
                })?;
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
            Err(ThreeThoughtsError::IncompleteIfArgs("IfSome".into(), 1))
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
```

<div id="file-18"></div>

## 📄 src\lib.rs

```rs
pub mod core;
```

<div id="file-19"></div>

## 📄 src\main.rs

```rs
use anyhow::Result;
use clap::{Parser, Subcommand};

use std::fs;
use ts3::core::execute::VM;
use ts3::core::parser::parse_program;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        file: String,

        #[arg(long, default_value = "65536")]
        mem: usize,

        #[arg(long, default_value = "false")]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file, mem, debug } => {
            let content = fs::read_to_string(&file)?;
            let instructions = parse_program(&content)?;
            let mut vm = VM::new(instructions, mem, debug);
            vm.run()?;
        }
    }
    Ok(())
}
```

