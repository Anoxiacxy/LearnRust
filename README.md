# 量化交易平台

一个基于 Rust 的多 crate 量化交易平台项目，具有清晰的架构和模块化设计。

## 📁 项目结构

```
.
├── Cargo.toml          # 工作空间配置
├── bin/                # 二进制程序
│   ├── cli/           # 命令行工具
│   └── gui/           # GUI 应用
└── crates/            # 核心库
    ├── common/        # 共享工具和基础设施
    ├── app-core/      # 应用核心逻辑
    ├── data/          # 数据获取和处理
    └── plot/          # 图表绘制
```

## 📦 模块说明

- **`common`**: 共享工具、配置管理、依赖注入等基础设施
- **`app-core`**: 应用核心逻辑，包括 CLI 定义、市场监控、用户服务等
- **`data`**: 数据层，支持多种数据源（Binance、Yahoo Finance 等）
- **`plot`**: 可视化层，基于 plotters 的图表绘制功能
- **`cli`**: 命令行工具，提供绘图和实时监控功能
- **`gui`**: GUI 应用，基于 egui 的图形界面

## 🚀 构建和运行

### 构建整个项目
```bash
cargo build
```

### 运行 CLI 工具
```bash
# 绘制 BTC 行情图
cargo run -p cli -- plot --symbol BTC

# 实时监控多个币种
cargo run -p cli -- realtime --symbols "BTC,ETH,SOL" --interval 300
```

### 运行 GUI 应用
```bash
cargo run -p gui
```

## 🛠 开发

本项目使用：
- Rust 2021 edition
- Tokio 异步运行时
- Serde 序列化
- eframe/egui GUI 框架
- plotters 图表库
- 统一的错误处理（thiserror + anyhow）

## 📈 功能特性

- 🔄 实时市场数据获取
- 📊 K线图表绘制
- 🖥️ 命令行和图形界面
- 🏗️ 模块化架构设计
- ⚡ 异步处理
- 🛡️ 类型安全的错误处理