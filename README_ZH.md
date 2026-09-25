# macOS Task Cleaner CLI (`mtc` / `taskcleaner`)

<p align="left">
  <a href="README.md">English</a> | <a href="README_ZH.md">简体中文</a>
</p>

<p align="left">
  <a href="https://apple.com/macos"><img src="https://img.shields.io/badge/平台-macOS-000000?logo=apple&logoColor=white" alt="平台: macOS" /></a>
  <img src="https://img.shields.io/badge/架构-Apple%20Silicon%20%7C%20AMD64-blue" alt="架构: Apple Silicon | AMD64" />
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/语言-Rust-dea584?logo=rust&logoColor=white" alt="语言: Rust" /></a>
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust&logoColor=white" alt="Rust: 1.75+" />
  <img src="https://img.shields.io/badge/二进制格式-Universal%20Mach--O-purple" alt="格式: Universal Mach-O" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/开源协议-GNU%20AGPLv3-blue" alt="开源协议: GNU AGPLv3" /></a>
  <a href="COMMERCIAL.md"><img src="https://img.shields.io/badge/商业许可-可授权-orange" alt="商业许可: 可授权" /></a>
</p>

面向 macOS 的轻量级、工程级前台任务清场命令行工具。基于底层高性能核心引擎 [macos-task-cleaner-core](https://github.com/DonJone/macos-task-cleaner-core) 构建。

---

## 界面效果展示

<p align="center">
  <img src="docs/images/cli-interactive.png" width="720" alt="macOS Task Cleaner 交互式终端向导" />
</p>

---

## 核心功能

* **交互式任务向导 (`-i` / `--interactive`)**：直观展示当前运行的所有前台 GUI 应用，支持输入序号一键将应用永久加入配置文件白名单、临时跳过或执行平滑清场。
* **零弹窗静默下线**：绕过应用层事件循环确认对话框，通过 POSIX 信号级平滑通知下线。
* **分级降级算法**：`SIGTERM` 软通知退出 -> 宽限期轮询 -> `SIGKILL` 兜底强退。
* **四级白名单防御**：系统核心 (L1)、当前会话终端与 IDE (L2)、常驻设施与输入法 (L3)、用户配置与命令行保留 (L4)。
* **即时追加白名单 (`-a` / `--add-whitelist`)**：支持在终端一行命令追加白名单规则，支持按 Bundle ID 或应用名称自动识别并去重写入。
* **预检模式 (`-n` / `--dry-run`)**：仅输出待清场应用及白名单命中分析，支持 `--json` 输出结构化数据，方便与 Raycast、Alfred 等外部自动化工具对接。

---

## 安装与下载

### 1. 预编译独立二进制包下载

前往 [GitHub Releases](https://github.com/DonJone/macos-task-cleaner-cli/releases/latest) 直接获取适用于您当前 Mac 架构的压缩包：

| 硬件架构 | 适用设备 | 安装包直链下载 |
| :--- | :--- | :--- |
| **Apple Silicon** (`arm64`) | Apple M1 / M2 / M3 / M4 芯片 Mac | [mtc-macos-arm64.tar.gz](https://github.com/DonJone/macos-task-cleaner-cli/releases/latest/download/mtc-macos-arm64.tar.gz) |
| **AMD64 / Intel** (`x86_64`) | Intel 处理器 / AMD64 架构 Mac | [mtc-macos-x86_64.tar.gz](https://github.com/DonJone/macos-task-cleaner-cli/releases/latest/download/mtc-macos-x86_64.tar.gz) |
| **Universal** (`universal`) | 兼容全部 Apple Silicon 及 Intel Mac | [mtc-macos-universal.tar.gz](https://github.com/DonJone/macos-task-cleaner-cli/releases/latest/download/mtc-macos-universal.tar.gz) |

解压后放置于 PATH 环境变量目录下即可使用：

```bash
tar -xzvf mtc-macos-arm64.tar.gz
sudo mv mtc /usr/local/bin/   # 或 ~/.local/bin/
```

### 2. 源码本地编译

要求已安装 Rust 工具链（1.75+）：

```bash
git clone https://github.com/DonJone/macos-task-cleaner-cli.git
cd macos-task-cleaner-cli

# 编译 Release 二进制文件
cargo build --release

# 安装主命令到用户 bin 目录
cp target/release/mtc ~/.local/bin/

# (可选) 创建 taskcleaner 软链接别名
ln -sf ~/.local/bin/mtc ~/.local/bin/taskcleaner
```

---

## 使用指南

### 1. 交互式向导模式 (推荐日常使用)

```bash
mtc -i
# 或
mtc --interactive
```

操作指令：
* `w [编号...]` 或 `1 2`：将序号为 1 和 2 的应用永久写入配置文件白名单；
* `t [编号...]`：在本轮清场中临时跳过该应用（不写入文件）；
* `c` 或 `clean`：确认执行平滑清场；
* `f` 或 `force`：直接强制秒杀（直接发送 SIGKILL）；
* `p` 或 `protected`：查看当前已被白名单保护的清单；
* `r` 或 `refresh`：重新扫描前台进程；
* `q` 或 `quit`：取消并安全退出。

### 2. 预检预览 (不杀任何进程)

```bash
mtc --dry-run

# 临时指定保留特定应用
mtc -k "微信" -k "Google Chrome" --dry-run

# 以结构化 JSON 格式输出
mtc --json --dry-run
```

### 3. 一键追加白名单

```bash
# 支持按应用显示名称添加
mtc -a "微信"

# 支持按 Bundle ID 添加 (推荐)
mtc -a "com.spotify.client" -a "com.google.Chrome"
```

### 4. 实质执行清场

```bash
# 执行标准三段式平滑清场
mtc --execute

# 跳过宽限期直接强退 (秒杀模式)
mtc --force

# 清理后强制回收系统 inactive 内存缓存
mtc --execute --purge
```

---

## 命令行完整参数说明

```text
用法:
  mtc [选项]   (或 taskcleaner [选项])

核心选项:
  -i, --interactive         交互式清场向导 (推荐: 支持序号选择、一键添加白名单与确认清场)
  -n, --dry-run             预检预览模式 (仅扫描并分析白名单过滤，不发送任何终止信号)
  -e, --execute             执行实质清场动作 (执行 SIGTERM -> 轮询 -> SIGKILL 三段式下线)
  -f, --force               强制直接秒杀 (跳过宽限期，直接发送 SIGKILL)
  -a, --add-whitelist <ID>  向永久配置文件追加白名单规则 (支持名称或 Bundle ID，如: -a com.google.Chrome)
  -k, --keep <NAME/BUNDLE>  命令行临时追加豁免白名单 (仅对当前进程生效，支持多次传入)
  -p, --purge               清场完成后调用 /usr/sbin/purge 强制回收内存缓存
  -c, --config <FILE>       指定自定义 TOML 配置文件路径
      --init-config         在 ~/.config/mtc/config.toml 生成默认配置模板
      --json                以结构化 JSON 格式输出结果 (适配 Raycast / 脚本接入)
  -h, --help                显示帮助说明
  -v, --version             显示当前版本
```

---

## 配置文件说明

配置文件优先位于 `~/.config/mtc/config.toml`（亦兼容 `~/.config/taskcleaner/config.toml`）：

```toml
[general]
# 宽限期轮询超时时长 (单位: 毫秒，默认 400ms)
grace_period_ms = 400

# 默认是否以 --dry-run 预检模式运行 (true: 仅扫描分析; false: 默认执行清理)
default_dry_run = false

[whitelist]
# 用户自定义白名单 - 按 Bundle Identifier 豁免 (推荐)
bundle_ids = [
    "com.google.Chrome",
    "com.spotify.client",
]

# 用户自定义白名单 - 按应用显示名称豁免
names = [
    "Telegram",
    "Slack",
]
```

---

## 关联项目

* **核心算法与进程引擎库 (Rust)**：[macos-task-cleaner-core](https://github.com/DonJone/macos-task-cleaner-core)
* **状态栏常驻客户端 (Swift)**：[macos-task-cleaner-gui](https://github.com/DonJone/macos-task-cleaner-gui)

---

## 许可协议与商业授权

本项目采用双重授权模式（Dual-Licensing Model）：

1. **开源许可证**：遵循 **GNU Affero General Public License v3.0 (AGPLv3)** 协议。个人学习、学术研究与非商业开源项目可免费使用与修改；凡修改或基于本项目构建衍生作品（包括通过网络提供交互服务的 SaaS / 云端调用形态），均须向公众无偿开源全部衍生代码。详见 [LICENSE](LICENSE)。
2. **商业许可协议 (Commercial License)**：面向企业客户、闭源专有产品集成、白标重命名销售或无法遵守 AGPLv3 传染性条款的商业场景，必须事先取得商业授权许可证。详见 [COMMERCIAL.md](COMMERCIAL.md)。
3. **商标与品牌保护**：项目名称、标识图形与应用图标均受版权及商标保护。任何二次分发或分叉 (Fork) 版本必须彻底去除官方品牌元素。详见 [TRADEMARK.md](TRADEMARK.md)。

Copyright (c) 2026 DonJone. 保留所有权利。
