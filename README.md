# Solana Token Swap Contract

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

## 技术规格

- Anchor 框架版本：0.31.0
- Solana 程序版本：2.1.0
- Rust 版本：2021 版本

## 构建指南

### 环境准备

1. 安装 Rust 和 Solana CLI 工具：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
```

2. 安装 Anchor 框架：

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
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

2. 运行 Anchor 测试：

```bash
anchor test
```

这将执行`tests`目录中的测试套件，包括：

- 创建代币 A 和代币 B 的铸币厂
- 为用户和交换池设置代币账户
- 初始化交换池
- 测试代币 A 兑换代币 B
- 测试代币 B 兑换代币 A

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

## 依赖版本

- anchor-lang: 0.31.0 (使用 init-if-needed 特性)
- anchor-spl: 0.31.0
- solana-program: 2.1.0

## 许可证

本项目使用 MIT 许可证。

---

# Solana Token Swap Contract

A simple token swap smart contract on the Solana blockchain, developed using the Anchor framework. This contract allows users to swap between two tokens at a 1:1 ratio.

## Features

- Simple 1:1 token swap mechanism
- Support for swapping between any SPL tokens
- Built with the Anchor framework for clean, concise code
- Comprehensive test suite

## Contract Functionality

1. **Initialize Swap Pool**: Create a new swap pool, setting up token A and token B mints and accounts
2. **Swap Token A to Token B**: Users can swap token A for an equal amount of token B
3. **Swap Token B to Token A**: Users can swap token B for an equal amount of token A

## Code Structure

- **SwapPool**: Account structure that stores swap pool information, including mint addresses and token accounts for both tokens
- **Initialize**: Instruction context for initializing the swap pool
- **Swap**: Instruction context for executing token swaps

## Setup Guide

### Prerequisites

1. Install Rust and Solana CLI tools:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
```

2. Install Anchor framework:

```bash
npm install -g @project-serum/anchor-cli
```

3. Install project dependencies:

```bash
npm install
```

### Building the Project

1. Build the Anchor project:

```bash
anchor build
```

This will generate deployment files in the `target/deploy` directory.

2. Update the program ID:

After building, copy the generated program ID and update the value in the `declare_id!()` macro and in the Anchor.toml file.

```bash
solana address -k target/deploy/solana_swap-keypair.json
```

Update the address in the `lib.rs` file in the `declare_id!()` and in the `Anchor.toml` file.

## Testing Guide

### Testing in Local Environment

1. Start a local Solana test validator:

```bash
solana-test-validator
```

2. Run Anchor tests:

```bash
anchor test
```

This will execute the test suite in the `tests` directory, which will:

- Create token mints for token A and token B
- Set up user and pool token accounts
- Initialize the swap pool
- Test swapping token A for token B
- Test swapping token B for token A

## Deployment Guide

1. Configure Solana cluster:

```bash
# Use devnet
solana config set --url devnet
# Or use mainnet
solana config set --url mainnet-beta
```

2. Deploy the program:

```bash
anchor deploy
```

## Usage Flow

1. Initialize the swap pool
2. Provide sufficient token A and token B liquidity to the swap pool
3. Users can call the `swap_a_to_b` function to swap token A for token B
4. Users can call the `swap_b_to_a` function to swap token B for token A

## Security Considerations

1. This contract only implements basic 1:1 token swap functionality without advanced features like fees or slippage protection
2. Additional security checks and error handling should be added for production environments
3. A comprehensive security audit is recommended before deploying to mainnet

## Contributing

Contributions are welcome through Pull Requests or Issues to improve the code or suggest enhancements.

## License

This project is licensed under the MIT License.

---

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

2. 运行 Anchor 测试：

```bash
anchor test
```

这将执行`tests`目录中的测试套件，包括：

- 创建代币 A 和代币 B 的铸币厂
- 为用户和交换池设置代币账户
- 初始化交换池
- 测试代币 A 兑换代币 B
- 测试代币 B 兑换代币 A

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
