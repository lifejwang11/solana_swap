# Solana 代币交换合约

一个基于 Solana 区块链的简单代币交换智能合约，使用 Anchor 框架开发。该合约允许用户以 1:1 的比例在两种代币之间进行互换。

## 功能特点

- 简单的 1:1 代币交换机制
- 支持任意 SPL 代币之间的兑换
- 使用 Anchor 框架开发，代码简洁易懂
- 完整的测试套件

## 合约功能

1. **初始化交换池**：创建一个新的交换池，设置代币 A 和代币 B 的铸币厂和代币账户
2. **代币 A 兑换代币 B**：用户可以将代币 A 兑换为等量的代币 B
3. **代币 B 兑换代币 A**：用户可以将代币 B 兑换为等量的代币 A

## 代码结构

- **SwapPool**：存储交换池信息的账户结构，包含两种代币的铸币厂地址和代币账户地址
- **Initialize**：初始化交换池的指令上下文
- **Swap**：执行代币交换的指令上下文

## 构建指南

### 环境准备

1. 安装 Rust 和 Solana CLI 工具：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
```

2. 安装 Anchor 框架：

```bash
npm install -g @project-serum/anchor-cli
```

3. 安装项目依赖：

```bash
npm install
```

### 构建项目

1. 构建 Anchor 项目：

```bash
anchor build
```

这将在`target/deploy`目录下生成部署文件。

2. 更新程序 ID：

构建后，复制生成的程序 ID 并更新`declare_id!()`宏中的值和 Anchor.toml 文件中的程序 ID。

```bash
solana address -k target/deploy/solana_swap-keypair.json
```

将输出的地址更新到`lib.rs`文件中的`declare_id!()`和`Anchor.toml`文件中。

## 测试指南

### 本地环境测试

1. 启动本地 Solana 测试验证节点：

```bash
solana-test-validator
```

2. 创建测试代币：

```bash
# 创建代币A
spl-token create-token --decimals 9
# 创建代币B
spl-token create-token --decimals 9
```

记下生成的代币铸币厂地址。

3. 创建代币账户并铸造代币：

```bash
# 为代币A创建账户
spl-token create-account <代币A铸币厂地址>
# 为代币B创建账户
spl-token create-account <代币B铸币厂地址>
# 铸造代币A
spl-token mint <代币A铸币厂地址> 1000000000
# 铸造代币B
spl-token mint <代币B铸币厂地址> 1000000000
```

4. 运行 Anchor 测试：

```bash
anchor test
```

### 编写测试用例

在`tests`目录下编写测试用例。示例测试脚本可参考项目文档。

## 部署指南

1. 设置 Solana 集群：

```bash
# 使用开发网
solana config set --url devnet
# 或使用主网
solana config set --url mainnet-beta
```

2. 部署程序：

```bash
anchor deploy
```

## 使用流程

1. 初始化交换池
2. 为交换池提供足够的代币 A 和代币 B 流动性
3. 用户可以调用`swap_a_to_b`函数将代币 A 兑换为代币 B
4. 用户可以调用`swap_b_to_a`函数将代币 B 兑换为代币 A

## 安全注意事项

1. 此合约仅实现了最基本的 1:1 代币交换功能，没有实现费用收取、滑点保护等高级功能
2. 实际生产环境中应增加额外的安全检查和错误处理
3. 在主网部署前，建议进行全面的安全审计

## 贡献

欢迎通过 Pull Request 或 Issue 贡献代码或提出改进建议。

## 许可证

本项目采用 MIT 许可证。
