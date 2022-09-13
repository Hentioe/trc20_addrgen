# trc20_addrgen

批量生成 TRC20 地址。

## 文档目录

- [使用指南](#使用指南)
- [编译指南](#编译指南)

## 使用指南

假设编译完成的 `trc20-addrgen` 在当前目录，使用如下命令生成单个 TRC20 账户信息：

```bash
./trc20-addrgen
```

（无需任何参数，直接运行即可）

输出内容：

```text
private: 8674e932dfc80eb7e6a7e938bc13e****351eb66e5fd9c1a6b7f000aa0804f3c
public:  bc376a95a03966a21439ff07d5488****4d50a434ff05881f5cee4b92710143ef2250b40e2009cbcbc7a5e66420e97fb40c8b53bc2d46d87e4f8d91355c4c05a
address: TSKyuMWVQ8db7mGnVobWokkN6Hg1W****k
```

以上三行信息分别是私钥、公钥和地址。

### 批量生成多个地址

```bash
./trc20-addrgen 9
```

（参数 `9` 表示生成 9 个私钥/地址对）

输出内容：

```text
ADDRESS                           :                                                         PRIVATE
TX4MXx2LWwanCoU****dya5zLy98fGYj4d:57ded402dd3edf4bafedbd138df1****8693ae0e059237c70299c959f1d6ca5e
TEbqZULwCrvEJ91****N33qKhttPGayuGk:b801b3ceb80ae5f27bb4a16528c8****2301de222b83b77b173e5af07adf3b76
TBAzMKWc3qg1g62****Fjchhc8699ArT2R:21b6b922679173fe7e25a99c82d3****87f0ae614bdbf8ec86a994eb5feeee71
TD46Sev1SgatwHJ****9keBEsDEuMrkZVi:2ed9a11e37cfa19ecd4602e7a0c2****2d1d5b5c5720962fa740b2eb290ebe05
TYfu8evzThu69iT****q29HYQbGHganEf5:1e6215969b389e612f034421c110****4786aae45cd9877451b63e986465e092
TLDqZmaFeKejhZ2****ScKMqtrYvXoxD1A:53452fa1d5c55ec3f5268c44906c****013a4a027c59e59d85a45d866077f625
TXfAUoYriVsgRkV****HfhLxpkDLUS6YvZ:c16054a6f86100bf540b22736f3e****25f82b5be675df043dbc9b3ccd0766ad
TRQY9tRG3U84ZH1****z8oWxeqziEH8Uq4:3dd00ab9fb7add613322c67f2103****e0fd774b50d614c178b69ff0f3ec5110
TNdcBXJoRZzH8BE****oGypPT5JrLbHSMB:1c9e47480098e432950e0cfe2704****e06468707056c06884917ab490081267
```

左边的是地址，右边的是私钥，它们用`:`间隔开来。

_注意：命令会直接输出明文地址和私钥，没有星号。上述内容为了避免地址被使用，手动用星号做过字符替换。_

## 编译指南

此指南包括了本项目的完整构建过程。

### 环境准备

构建本项目需要安装 Rust 1.63 版本，构建环境决定了对部署环境的选择（两种环境的系统版本须保持一致）。

### 安装 Rustup

Rustup 是 Rust 官方的安装工具，运行命令进行安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

_注意不要安装系统软件源中的 rust 相关的包，它们通常太旧了。_

此命令会产生一个交互式安装过程，一路默认（直接回车）即可。结束后执行 `rustc --version` 确认 Rust 工具链完整无误。

_如果提示 `rustc` 未找到，可能需要关闭/重新打开当前的终端，让一些新的环境变量生效。_

### 安装项目指定 Rust 版本

在上一步中 Rustup 安装后也同时安装了最新的 Rust 版本，但是 Rustup 只需要安装一次，后续 Rust 版本的变更（升级或降级）需要通过这一步来完成。

在项目源码目录下运行命令：

```bash
rustup show
```

此命令会读取源码根目录的 `rust-toolchain` 文件，安装其指定的 Rust 版本。

_注意：构建项目步骤中的 `cargo` 命令也可能会自动安装指定的 Rust 版本，所以这一步有时候可以省略。_

### 构建项目

运行命令：

```bash
cargo build --release
```

成功后在 `target/release` 目录会生成 `trc20-addrgen` 文件，复制它们到部署机器上运行即可。文件名 `trc20-addrgen` 的风格和 `ssh-keygen` 这一类程序类似，表示 TRC20 地址生成器。
