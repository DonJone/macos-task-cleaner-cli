# macOS Task Cleaner CLI (`mtc` / `taskcleaner`)

面向 macOS 的轻量级、工程级前台任务清场命令行工具。基于核心引擎 [macos-task-cleaner-core](https://github.com/DonJone/macos-task-cleaner-core) 构建。

---

## 核心功能

* **交互式任务向导 (`-i / --interactive`)**：直观展示当前运行的所有前台 GUI 应用，支持输入序号一键将应用永久加入配置文件白名单、临时跳过或执行清场。
* **零弹窗静默下线**：绕过应用层事件循环确认对话框，通过 POSIX 信号级平滑通知下线。
* **分级降级算法**：`SIGTERM` 软通知退出 -> 宽限期轮询 -> `SIGKILL` 兜底强退。
* **四级白名单防御**：系统核心 (L1)、当前会话终端与 IDE (L2)、常驻设施与输入法 (L3)、用户配置与命令行保留 (L4)。
* **即时追加白名单 (`-a / --add-whitelist`)**：支持在终端一行命令追加白名单规则，支持按 Bundle ID 或应用名称自动识别并去重写入。
* **预检模式 (`-n / --dry-run`)**：仅输出待清场应用及白名单命中分析，支持 `--json` 输出结构化数据。

---

## 安装

### 方式一：从源码构建安装

要求已安装 Rust 工具链 (1.75+)：

```bash
git clone https://github.com/DonJone/macos-task-cleaner-cli.git
cd macos-task-cleaner-cli
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
# 启动交互式向导
mtc -i
# 或
mtc --interactive
```

操作指令：
* `w 1, 2` 或 `1 2`：将序号为 1 和 2 的应用永久写入配置文件白名单；
* `t 1`：本轮清场中临时跳过该应用（不写入文件）；
* `c` 或 `clean`：确认执行平滑清场；
* `f` 或 `force`：直接强制秒杀；
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
mtc -a "com.spotify.client" -a "com.tencent.xinWeChat"
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
  -a, --add-whitelist <ID>  向永久配置文件追加白名单规则 (支持名称或 Bundle ID，如: -a 微信)
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

# 默认是否以 --dry-run 预检模式运行 (true: 需显式传 -e 才会执行清理; false: 默认执行清理)
default_dry_run = false

[whitelist]
# 用户自定义白名单 - 按 Bundle Identifier 豁免 (推荐)
bundle_ids = [
    # "com.apple.Music",
    # "com.spotify.client",
    # "com.tencent.xinWeChat",
]

# 用户自定义白名单 - 按应用显示名称豁免
names = [
    # "Music",
    # "微信",
    # "Slack",
]
```

---

## 关联项目

* **核心算法与进程引擎库**：[macos-task-cleaner-core](https://github.com/DonJone/macos-task-cleaner-core)

---

## 许可协议

MIT License
