# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.0] - 2026-07-12

### Added

- 虚拟机核心：`VM` 结构体，支持 PC、指令执行、运行循环
- 内存模型：线性 `Vec<u8>` 单元 + 指针，支持动态扩展
- 三类指令体系（共 31 条）：
  - **WhoAmI**（2）：`Named`、`Renamed` — VM 身份
  - **WhereAmI**（5）：`Origin`、`Keep`、`JumpTo`、`Add`、`Sub` — 指针操作
  - **WhatDoIDo**（24）：`Add`、`Sub`、`AddOther`、`SubOther`、`Free`、`FreeOther`、`GetOther`、`Print`、`Println`、`Note`、`Reset`、`ResetOrigin`、`Loop`、`JumpTo`、`IfZero`、`IfNotZero`、`IfSome`、`IfNotSome`、`IfName`、`Panic`、`Read`、`ReadASCII`、`Dump`、`Reverse` — 计算与控制流
- 循环支持：`Loop`（固定次数）+ `IfZero`/`JumpTo`（条件 while 循环），支持嵌套
- 条件分支：`IfZero`、`IfNotZero`、`IfSome`、`IfNotSome`、`IfName` 五种条件跳转
- 输入系统：`Read`（读原始字节）、`ReadASCII`（读十进制 ASCII 码），自动跳过空白
- 调试工具：`Dump`（打印内存）、`--debug` CLI flag（逐指令追踪 PC/ptr/cell/循环状态）
- 错误系统：17 种结构化错误类型，覆盖解析和运行时
- `//` 注释支持（整行 + 行内）
- CLI：`cargo run -- run <file>`，支持 `--mem` 和 `--debug`
- 示例程序：hello_world、loop、condition、if_zero、demo、overview、read、playground、fibonacci、add_sub_other


## 版本标记说明

- `Added`：新增功能
- `Changed`：功能变更（非破坏性）
- `Deprecated`：标记为废弃，将在未来移除
- `Fixed`：问题修复
- `Removed`：移除功能
- `Security`：安全相关修复