# 部署指南

本指南将引导您完成将 SwapV3 程序部署到 Solana 网络（包括本地测试网、开发网和主网）的整个过程。

## 1. 环境准备

在开始之前，请确保您已经安装了以下工具：

- **Rust:** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **Solana CLI:** [https://docs.solana.com/cli/install](https://docs.solana.com/cli/install)

安装完成后，请通过以下命令验证安装是否成功：

```bash
rustc --version
solana --version
```

## 2. 构建程序

在部署之前，您需要将 Rust 代码编译成 Solana 程序所使用的 BPF（Berkeley Packet Filter）字节码。

在项目根目录下运行以下命令：

```bash
cargo build-bpf
```

编译成功后，您将在 `target/deploy/` 目录下找到一个名为 `swapv3.so` 的文件。这就是您需要部署的程序文件。

## 3. 部署到不同网络

Solana CLI 允许您轻松地将程序部署到不同的网络环境。

### 部署到本地测试网

本地测试网是在您的计算机上运行的私有 Solana 集群，非常适合开发和调试。

1. **启动本地测试网：**
    打开一个新的终端窗口，运行以下命令来启动一个本地验证器：

    ```bash
    solana-test-validator
    ```

2. **配置 CLI 到本地网络：**
    在另一个终端窗口中，将 Solana CLI 的目标网络设置为本地：

    ```bash
    solana config set --url localhost
    ```

3. **部署程序：**
    执行部署命令，并指定编译好的 `.so` 文件：

    ```bash
    solana program deploy target/deploy/swapv3.so
    ```

    部署成功后，CLI 会输出一个 **Program ID**。请务必保存好这个 ID，因为它是与您的程序交互的唯一标识。

### 部署到开发网 (Devnet)

开发网是一个公开的测试网络，由 Solana 官方维护，用于测试和分享您的应用。

1. **配置 CLI 到开发网：**

    ```bash
    solana config set --url https://api.devnet.solana.com
    ```

2. **获取测试 SOL：**
    开发网上的操作需要支付 gas 费。您可以通过以下命令免费获取一些测试 SOL：

    ```bash
    solana airdrop 2
    ```

3. **部署程序：**
    与本地部署类似，执行部署命令：

    ```bash
    solana program deploy target/deploy/swapv3.so
    ```

    同样，请记下返回的 **Program ID**。

### 部署到主网 (Mainnet Beta)

将程序部署到主网是最后一步，这意味着您的应用将正式上线并处理真实的资产。

**警告：** 主网操作涉及真实资金，请务必在本地和开发网充分测试后再进行部署。

1. **配置 CLI 到主网：**

    ```bash
    solana config set --url https://api.mainnet-beta.solana.com
    ```

2. **确保您的钱包有足够的 SOL：**
    主网部署需要支付真实的 SOL 作为 gas 费。

3. **部署程序：**

    ```bash
    solana program deploy target/deploy/swapv3.so
    ```

部署完成后，您的 SwapV3 程序就可以在所选的网络上接收指令并处理交易了。
