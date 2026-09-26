use macos_task_cleaner_core::Language;

/// CLI 多语言文案字典
pub struct CliMessages;

impl CliMessages {
    /// 命令行完整帮助信息
    pub fn help_text(version: &str, lang: Language) -> String {
        match lang {
            Language::ZhHant => format!(
                r#"macOS Task Cleaner (mtc) v{}
輕量級幕前任務清場工具 (面向 macOS 的免彈窗、多級白名單任務清理引擎)
Copyright (c) 2026 DonJone. All rights reserved.
License: GNU AGPLv3 / Commercial Dual License. (See COMMERCIAL.md & TRADEMARK.md)

用法:
  mtc [選項]   (或 taskcleaner [選項])

核心選項:
  -i, --interactive         互動式清場精靈 (推薦: 支援序號選擇、新增/移除白名單與確認清場)
  -n, --dry-run             預檢預覽模式 (僅掃描並分析白名單過濾，不發送任何終止信號)
  -e, --execute             執行實質終止動作 (按序執行 SIGTERM -> 輪詢 -> SIGKILL 階梯式終止)
  -f, --force               強制直接終止 (跳過寬限期，直接發送 SIGKILL)
  -t, --terminate-pid <PID> 針對指定的單個或多個 PID 執行階梯式終止
  -a, --add-whitelist <ID>  向永久設定檔追加白名單規則 (支援名稱或 Bundle ID，如: -a 微信)
  -r, --remove-whitelist <ID> 從白名單移除規則並記錄至停用清單 (支援內建預設與使用者規則)
  -k, --keep <NAME/BUNDLE>  命令列臨時追加豁免白名單 (僅對目前處理程序生效，支援多次傳入)
  -p, --purge               終止完成後呼叫 /usr/sbin/purge 清空系統快取
  -c, --config <FILE>       指定自訂 TOML 設定檔路徑
  -l, --lang <LANG>         指定介面語言 (支援: en, zh-Hans, zh-Hant, ja 等 24 種語言，預設自動偵測)
      --init-config         在 ~/.config/mtc/config.toml 產生預設設定範本
      --json                以結構化 JSON 格式輸出結果 (適配 Raycast / 指令碼串接)
  -h, --help                顯示說明文件
  -v, --version             顯示目前版本與授權資訊

白名單分級體系:
  L1: 系統核心層 (Finder, Dock, WindowServer)
  L2: 會話終端層 (保護目前呼叫終端機、父會話 PID 與常用終端機/IDE)
  L3: 常駐設施層 (Raycast, Alfred, 視窗管理與輸入法)
  L4: 使用者設定層 (來自設定檔與 -k/--keep 命令列參數)"#,
                version
            ),
            Language::Ja => format!(
                r#"macOS Task Cleaner (mtc) v{}
軽量フォアグラウンドタスク終了ツール (macOS向け ダイアログなし・多段ホワイトリスト プロセス整理エンジン)
Copyright (c) 2026 DonJone. All rights reserved.
License: GNU AGPLv3 / Commercial Dual License. (See COMMERCIAL.md & TRADEMARK.md)

使用方法:
  mtc [オプション]   (または taskcleaner [オプション])

主要オプション:
  -i, --interactive         インタラクティブウィザード (推奨: 番号選択、ホワイトリスト追加/解除、終了確認)
  -n, --dry-run             ドライランプレビュー (スキャンとフィルタリング分析のみ、終了シグナルは送信しません)
  -e, --execute             終了処理を実行 (SIGTERM -> ポーリング -> SIGKILL の段階的終了)
  -f, --force               強制即時終了 (猶予期間をスキップし、直接 SIGKILL を送信)
  -t, --terminate-pid <PID> 指定された PID に対して段階的終了を実行
  -a, --add-whitelist <ID>  設定ファイルにホワイトリスト規則を追加 (名称または Bundle ID)
  -r, --remove-whitelist <ID> ホワイトリストから除外し無効リストへ登録
  -k, --keep <NAME/BUNDLE>  現在のセッションのみホワイトリストを一時追加 (複数回指定可)
  -p, --purge               終了完了後に /usr/sbin/purge を実行しシステムキャッシュを解放
  -c, --config <FILE>       カスタム TOML 設定ファイルパスを指定
  -l, --lang <LANG>         表示言語を指定 (en, zh-Hans, zh-Hant, ja 等 24 言語対応、デフォルト: 自動検出)
      --init-config         ~/.config/mtc/config.toml にデフォルト設定テンプレートを生成
      --json                構造化 JSON 形式で出力 (Raycast / スクリプト連携用)
  -h, --help                ヘルプを表示
  -v, --version             バージョンとライセンス情報を表示

ホワイトリスト階層システム:
  L1: システムコア層 (Finder, Dock, WindowServer)
  L2: セッション端末層 (現在のターミナル、親プロセス PID、ターミナル/IDE を保護)
  L3: 常駐ユーティリティ層 (Raycast, Alfred, ウィンドウマネージャ, IME)
  L4: ユーザー設定層 (設定ファイルおよび -k/--keep コマンドライン引数)"#,
                version
            ),
            Language::ZhHans => format!(
                r#"macOS Task Cleaner (mtc) v{}
轻量级前台任务清场工具 (面向 macOS 的免弹窗、多级白名单任务清理引擎)
Copyright (c) 2026 DonJone. All rights reserved.
License: GNU AGPLv3 / Commercial Dual License. (See COMMERCIAL.md & TRADEMARK.md)

用法:
  mtc [选项]   (或 taskcleaner [选项])

核心选项:
  -i, --interactive         交互式清场向导 (推荐: 支持序号选择、添加/移除白名单与确认清场)
  -n, --dry-run             预检预览模式 (仅扫描并分析白名单过滤，不发送任何终止信号)
  -e, --execute             执行实质终止动作 (按序执行 SIGTERM -> 轮询 -> SIGKILL 梯次终止)
  -f, --force               强制直接终止 (跳过宽限期，直接发送 SIGKILL)
  -t, --terminate-pid <PID> 针对指定的单个或多个 PID 执行梯次终止
  -a, --add-whitelist <ID>  向永久配置文件追加白名单规则 (支持名称或 Bundle ID，如: -a 微信)
  -r, --remove-whitelist <ID> 从白名单移除规则并记录至禁用列表 (支持内置预设与用户规则)
  -k, --keep <NAME/BUNDLE>  命令行临时追加豁免白名单 (仅对当前进程生效，支持多次传入)
  -p, --purge               终止完成后调用 /usr/sbin/purge 清空系统缓存
  -c, --config <FILE>       指定自定义 TOML 配置文件路径
  -l, --lang <LANG>         指定界面语言 (支持: en, zh-Hans, zh-Hant, ja 等 24 种语言，默认自动检测)
      --init-config         在 ~/.config/mtc/config.toml 生成默认配置模板
      --json                以结构化 JSON 格式输出结果 (适配 Raycast / 脚本接入)
  -h, --help                显示帮助说明
  -v, --version             显示当前版本与许可信息

白名单分级体系:
  L1: 系统核心层 (Finder, Dock, WindowServer)
  L2: 会话终端层 (保护当前调用终端、父会话 PID 与常用终端/IDE)
  L3: 常驻设施层 (Raycast, Alfred, 窗口管理与输入法)
  L4: 用户配置层 (来自配置文件与 -k/--keep 命令行参数)"#,
                version
            ),
            _ => format!(
                r#"macOS Task Cleaner (mtc) v{}
Lightweight foreground app cleaner for macOS (dialog-free, tiered whitelist process termination engine)
Copyright (c) 2026 DonJone. All rights reserved.
License: GNU AGPLv3 / Commercial Dual License. (See COMMERCIAL.md & TRADEMARK.md)

Usage:
  mtc [OPTIONS]   (or taskcleaner [OPTIONS])

Core Options:
  -i, --interactive         Interactive task wizard (recommended: select by number, add/remove rules, confirm)
  -n, --dry-run             Dry-run preview mode (scan & filter analysis only, no termination signals sent)
  -e, --execute             Execute actual termination (SIGTERM -> poll -> SIGKILL tiered sequence)
  -f, --force               Force immediate termination (skip grace period, directly send SIGKILL)
  -t, --terminate-pid <PID> Execute tiered termination on specified PID(s)
  -a, --add-whitelist <ID>  Append rule to persistent config whitelist (app name or Bundle ID)
  -r, --remove-whitelist <ID> Remove rule from whitelist and record in disabled list
  -k, --keep <NAME/BUNDLE>  Temporarily whitelist an app for current execution only (repeatable)
  -p, --purge               Call /usr/sbin/purge after termination to release system memory cache
  -c, --config <FILE>       Specify custom TOML configuration file path
  -l, --lang <LANG>         Specify UI language (supports 24 locales: en, zh-Hans, zh-Hant, ja, etc.; default: auto)
      --init-config         Generate default configuration template at ~/.config/mtc/config.toml
      --json                Output results in structured JSON format (for Raycast / scripting)
  -h, --help                Display this help documentation
  -v, --version             Display version and license information

Whitelist Defense Matrix:
  L1: Core OS Layer (Finder, Dock, WindowServer)
  L2: Context Shell Layer (Protects caller terminal, parent session PID, and developer IDEs)
  L3: Persistent Utilities Layer (Raycast, Alfred, window managers, and input methods)
  L4: User Configuration Layer (Rules from config.toml and -k/--keep command-line overrides)"#,
                version
            ),
        }
    }

    /// 版本与版权信息
    pub fn version_text(version: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!(
                "macOS Task Cleaner CLI (mtc / taskcleaner) v{}\n\
                 版权所有 (c) 2026 DonJone. 保留所有权利。\n\
                 开源协议: GNU Affero General Public License v3.0 (AGPLv3) / 商业双重许可。\n\
                 合规政策: 参见 COMMERCIAL.md 获取商业授权细节，参见 TRADEMARK.md 获取品牌政策。\n\
                 官方主页: https://github.com/macos-task-cleaner/macos-task-cleaner-cli",
                version
            ),
            Language::ZhHant => format!(
                "macOS Task Cleaner CLI (mtc / taskcleaner) v{}\n\
                 版權所有 (c) 2026 DonJone. 保留所有權利。\n\
                 開源協議: GNU Affero General Public License v3.0 (AGPLv3) / 商業雙重許可。\n\
                 合規政策: 參閱 COMMERCIAL.md 獲取商業授權細節，參閱 TRADEMARK.md 獲取品牌政策。\n\
                 官方主頁: https://github.com/macos-task-cleaner/macos-task-cleaner-cli",
                version
            ),
            Language::Ja => format!(
                "macOS Task Cleaner CLI (mtc / taskcleaner) v{}\n\
                 Copyright (c) 2026 DonJone. All rights reserved.\n\
                 ライセンス: GNU Affero General Public License v3.0 (AGPLv3) / 商用デュアルライセンス。\n\
                 ポリシー: 商用ライセンスは COMMERCIAL.md、商標規定は TRADEMARK.md をご覧ください。\n\
                 プロジェクト: https://github.com/macos-task-cleaner/macos-task-cleaner-cli",
                version
            ),
            _ => format!(
                "macOS Task Cleaner CLI (mtc / taskcleaner) v{}\n\
                 Copyright (c) 2026 DonJone. All rights reserved.\n\
                 License: GNU Affero General Public License v3.0 (AGPLv3) / Commercial Dual License.\n\
                 Policy: See COMMERCIAL.md for commercial licensing and TRADEMARK.md for brand policy.\n\
                 Homepage: https://github.com/macos-task-cleaner/macos-task-cleaner-cli",
                version
            ),
        }
    }

    /// 缺少参数值错误
    pub fn err_missing_value(arg: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("缺少 {} 参数值", arg),
            Language::ZhHant => format!("缺少 {} 參數值", arg),
            Language::Ja => format!("{} の引数値が不足しています", arg),
            _ => format!("Missing value for argument {}", arg),
        }
    }

    /// 无效 PID 错误
    pub fn err_invalid_pid(val: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("无效的 PID: {}", val),
            Language::ZhHant => format!("無效的 PID: {}", val),
            Language::Ja => format!("無効な PID です: {}", val),
            _ => format!("Invalid PID: {}", val),
        }
    }

    /// 未知参数错误
    pub fn err_unknown_arg(arg: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("未知参数: {}", arg),
            Language::ZhHant => format!("未知參數: {}", arg),
            Language::Ja => format!("未知の引数: {}", arg),
            _ => format!("Unknown argument: {}", arg),
        }
    }

    /// 参数错误说明提示
    pub fn err_hint_help(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "请使用 --help 查看完整参数说明。",
            Language::ZhHant => "請使用 --help 查看完整參數說明。",
            Language::Ja => "詳細は --help でパラメータの説明をご確認ください。",
            _ => "Please use --help to view complete argument documentation.",
        }
    }

    /// 配置初始化成功
    pub fn config_init_success(path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[配置初始化成功] 配置文件已生成至: {}", path),
            Language::ZhHant => format!("[設定初始化成功] 設定檔已產生至: {}", path),
            Language::Ja => format!("[設定初期化成功] 設定ファイルを生成しました: {}", path),
            _ => format!("[Config Initialized] Configuration file created at: {}", path),
        }
    }

    /// 配置初始化失败
    pub fn config_init_failed(err: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[配置初始化失败] 无法写入配置文件: {}", err),
            Language::ZhHant => format!("[設定初始化失敗] 無法寫入設定檔: {}", err),
            Language::Ja => format!("[設定初期化失敗] 設定ファイルの書き込みに失敗しました: {}", err),
            _ => format!("[Config Init Failed] Cannot write configuration file: {}", err),
        }
    }

    /// 添加白名单成功
    pub fn whitelist_add_success(ident: &str, path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[白名单添加成功] 已将 '{}' 写入配置文件: {}", ident, path),
            Language::ZhHant => format!("[白名單新增成功] 已將 '{}' 寫入設定檔: {}", ident, path),
            Language::Ja => format!("[ホワイトリスト追加成功] '{}' を設定ファイルに追加しました: {}", ident, path),
            _ => format!("[Whitelist Added] Successfully saved '{}' to config file: {}", ident, path),
        }
    }

    /// 白名单已存在
    pub fn whitelist_add_exists(ident: &str, path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[白名单已存在] '{}' 已包含在配置文件中: {}", ident, path),
            Language::ZhHant => format!("[白名單已存在] '{}' 已包含在設定檔中: {}", ident, path),
            Language::Ja => format!("[ホワイトリスト既存] '{}' は既に設定ファイルに含まれています: {}", ident, path),
            _ => format!("[Whitelist Exists] '{}' is already in configuration file: {}", ident, path),
        }
    }

    /// 白名单添加失败
    pub fn whitelist_add_failed(ident: &str, err: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[白名单写入失败] 添加 '{}' 失败: {}", ident, err),
            Language::ZhHant => format!("[白名單寫入失敗] 新增 '{}' 失敗: {}", ident, err),
            Language::Ja => format!("[ホワイトリスト追加失敗] '{}' の追加に失敗しました: {}", ident, err),
            _ => format!("[Whitelist Add Failed] Failed to add '{}': {}", ident, err),
        }
    }

    /// 移除白名单成功
    pub fn whitelist_remove_success(ident: &str, path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[白名单移除成功] 已将 '{}' 从白名单移除/记录至禁用列表: {}", ident, path),
            Language::ZhHant => format!("[白名單移除成功] 已將 '{}' 從白名單移除/記錄至停用清單: {}", ident, path),
            Language::Ja => format!("[ホワイトリスト解除成功] '{}' を除外し無効リストに登録しました: {}", ident, path),
            _ => format!("[Whitelist Removed] Removed '{}' from whitelist and recorded to disabled rules: {}", ident, path),
        }
    }

    /// 白名单移除状态未变
    pub fn whitelist_remove_unchanged(ident: &str, path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[白名单状态未变] '{}' 未在白名单中或已处于禁用状态: {}", ident, path),
            Language::ZhHant => format!("[白名單狀態未變] '{}' 未在白名單中或已處於停用狀態: {}", ident, path),
            Language::Ja => format!("[ホワイトリスト変更なし] '{}' は含まれていないか既に無効化されています: {}", ident, path),
            _ => format!("[Whitelist Unchanged] '{}' was not in whitelist or already disabled: {}", ident, path),
        }
    }

    /// 移除白名单失败
    pub fn whitelist_remove_failed(ident: &str, err: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[白名单移除失败] 移除 '{}' 失败: {}", ident, err),
            Language::ZhHant => format!("[白名單移除失敗] 移除 '{}' 失敗: {}", ident, err),
            Language::Ja => format!("[ホワイトリスト解除失敗] '{}' の解除に失敗しました: {}", ident, err),
            _ => format!("[Whitelist Remove Failed] Failed to remove '{}': {}", ident, err),
        }
    }

    /// 单个 PID 终止成功
    pub fn term_pid_success(name: &str, pid: i32, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[终止成功] 已结束应用: {} (PID: {})", name, pid),
            Language::ZhHant => format!("[結束成功] 已結束應用程式: {} (PID: {})", name, pid),
            Language::Ja => format!("[終了成功] アプリを終了しました: {} (PID: {})", name, pid),
            _ => format!("[Terminated] Successfully terminated app: {} (PID: {})", name, pid),
        }
    }

    /// 单个 PID 终止失败
    pub fn term_pid_failed(name: &str, pid: i32, err: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[终止失败] 无法结束应用: {} (PID: {}): {}", name, pid, err),
            Language::ZhHant => format!("[結束失敗] 無法結束應用程式: {} (PID: {}): {}", name, pid, err),
            Language::Ja => format!("[終了失敗] アプリの終了に失敗しました: {} (PID: {}): {}", name, pid, err),
            _ => format!("[Termination Failed] Unable to terminate app: {} (PID: {}): {}", name, pid, err),
        }
    }

    /// 内存回收完成
    pub fn purge_success(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[内存回收完成] /usr/sbin/purge 缓存页面整理完成",
            Language::ZhHant => "[記憶體釋放完成] /usr/sbin/purge 快取頁面整理完成",
            Language::Ja => "[メモリ解放完了] /usr/sbin/purge によるキャッシュ解放が完了しました",
            _ => "[Memory Purge Complete] /usr/sbin/purge memory cache released",
        }
    }

    /// 内存回收警告
    pub fn purge_warn(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[警告] 执行 /usr/sbin/purge 失败或受权限限制",
            Language::ZhHant => "[警告] 執行 /usr/sbin/purge 失敗或受權限限制",
            Language::Ja => "[警告] /usr/sbin/purge の実行に失敗したか権限が不足しています",
            _ => "[Warning] /usr/sbin/purge execution failed or restricted by permissions",
        }
    }

    // ========================================================
    // Dry-Run 预检报表多语言支持
    // ========================================================

    pub fn dry_run_title(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "              macOS Task Cleaner - 预检预览 (Dry-Run)       ",
            Language::ZhHant => "              macOS Task Cleaner - 預檢預覽 (Dry-Run)       ",
            Language::Ja => "              macOS Task Cleaner - ドライランプレビュー      ",
            _ => "              macOS Task Cleaner - Dry-Run Preview          ",
        }
    }

    pub fn dry_run_stats(scanned: usize, protected: usize, targets: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!(
                "[统计概览] 发现前台图形应用: {} 个 | 白名单豁免: {} 个 | 拟清场应用: {} 个",
                scanned, protected, targets
            ),
            Language::ZhHant => format!(
                "[統計概覽] 發現幕前圖形應用: {} 個 | 白名單豁免: {} 個 | 擬清場應用: {} 個",
                scanned, protected, targets
            ),
            Language::Ja => format!(
                "[統計概要] フォアグラウンドアプリ検出: {} | ホワイトリスト除外: {} | 終了予定: {}",
                scanned, protected, targets
            ),
            _ => format!(
                "[Overview] Scanned Foreground Apps: {} | Whitelisted: {} | Targets to Clean: {}",
                scanned, protected, targets
            ),
        }
    }

    pub fn config_source_loaded(path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("已加载配置: {}", path),
            Language::ZhHant => format!("已載入設定: {}", path),
            Language::Ja => format!("読み込み済み設定: {}", path),
            _ => format!("Loaded config: {}", path),
        }
    }

    pub fn config_source_default(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "内置默认规则 (无外部配置文件)",
            Language::ZhHant => "內建預設規則 (無外部設定檔)",
            Language::Ja => "組み込みデフォルト規則 (外部設定ファイルなし)",
            _ => "Built-in default rules (no external config file)",
        }
    }

    pub fn dry_run_protected_banner(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[受保护应用清单 (豁免清理)]",
            Language::ZhHant => "[受保護應用程式清單 (豁免清理)]",
            Language::Ja => "[保護対象アプリ一覧 (終了除外)]",
            _ => "[Protected Applications (Exempt from Clean)]",
        }
    }

    pub fn dry_run_protected_cols(lang: Language) -> (&'static str, &'static str, &'static str, &'static str) {
        match lang {
            Language::ZhHans => ("PID", "应用名称", "保护层级", "豁免规则匹配"),
            Language::ZhHant => ("PID", "應用程式名稱", "保護層級", "豁免規則比對"),
            Language::Ja => ("PID", "アプリ名", "保護レベル", "一致した除外規則"),
            _ => ("PID", "App Name", "Protection Tier", "Matched Rule"),
        }
    }

    pub fn dry_run_targets_banner(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[拟终止前台应用清单 (待清场目标)]",
            Language::ZhHant => "[擬終止幕前應用程式清單 (待清場目標)]",
            Language::Ja => "[終了予定アプリ一覧 (整理対象)]",
            _ => "[Target Applications to Terminate]",
        }
    }

    pub fn dry_run_targets_cols(lang: Language) -> (&'static str, &'static str, &'static str, &'static str) {
        match lang {
            Language::ZhHans => ("PID", "应用名称", "Bundle ID", "存活状态"),
            Language::ZhHant => ("PID", "應用程式名稱", "Bundle ID", "存活狀態"),
            Language::Ja => ("PID", "アプリ名", "Bundle ID", "状態"),
            _ => ("PID", "App Name", "Bundle ID", "Status"),
        }
    }

    pub fn dry_run_targets_empty(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "(当前无需要清场的前台应用，工作区保持纯净)",
            Language::ZhHant => "(目前無需要清場的幕前應用程式，工作區保持純淨)",
            Language::Ja => "(終了対象のフォアグラウンドアプリはありません。作業空間は正常です)",
            _ => "(No foreground apps require termination; workspace is clean)",
        }
    }

    pub fn status_alive(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "活跃 [可发信号]",
            Language::ZhHant => "活躍 [可發送信號]",
            Language::Ja => "稼働中 [シグナル送信可]",
            _ => "Active [Signalable]",
        }
    }

    pub fn status_suspended(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "已挂起/已退出",
            Language::ZhHant => "已暫停/已結束",
            Language::Ja => "中断/終了済み",
            _ => "Suspended/Exited",
        }
    }

    pub fn dry_run_hint(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[提示] 当前为预检预览模式，未向任何目标发送实质终止信号。\n       若确认执行清场，请运行: mtc --execute (或 taskcleaner --execute)",
            Language::ZhHant => "[提示] 目前為預檢預覽模式，未向任何目標發送實質終止信號。\n       若確認執行清場，請執行: mtc --execute (或 taskcleaner --execute)",
            Language::Ja => "[ヒント] ドライランモードのため、実際の終了シグナルは送信されていません。\n         実行するには次を実行してください: mtc --execute (または taskcleaner --execute)",
            _ => "[Notice] Dry-run preview mode active; no termination signals were sent.\n         To execute cleanup, run: mtc --execute (or taskcleaner --execute)",
        }
    }

    pub fn dry_run_duration(ms: f64, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[扫描耗时] 检索与过滤完成耗时: {:.2}ms", ms),
            Language::ZhHant => format!("[掃描耗時] 檢索與過濾完成耗時: {:.2}ms", ms),
            Language::Ja => format!("[スキャン所要時間] 検出とフィルタリング所要時間: {:.2}ms", ms),
            _ => format!("[Scan Duration] Discovery & filtering completed in: {:.2}ms", ms),
        }
    }

    // ========================================================
    // 执行报告多语言支持
    // ========================================================

    pub fn report_title(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "              macOS Task Cleaner - 执行结果报告             ",
            Language::ZhHant => "              macOS Task Cleaner - 執行結果報告             ",
            Language::Ja => "              macOS Task Cleaner - 実行結果レポート          ",
            _ => "              macOS Task Cleaner - Execution Report         ",
        }
    }

    pub fn report_total_targets(count: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("* 目标应用总数:             {} 个", count),
            Language::ZhHant => format!("* 目標應用程式總數:         {} 個", count),
            Language::Ja => format!("* 対象アプリ総数:           {}", count),
            _ => format!("* Total Target Applications: {}", count),
        }
    }

    pub fn report_sigterm(count: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("* 标准终止 (SIGTERM):         {} 个", count),
            Language::ZhHant => format!("* 標準結束 (SIGTERM):         {} 個", count),
            Language::Ja => format!("* 標準終了 (SIGTERM):       {}", count),
            _ => format!("* Standard Terminate (SIGTERM): {}", count),
        }
    }

    pub fn report_sigkill(count: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("* 强制终止 (SIGKILL):         {} 个", count),
            Language::ZhHant => format!("* 強制結束 (SIGKILL):         {} 個", count),
            Language::Ja => format!("* 強制終了 (SIGKILL):       {}", count),
            _ => format!("* Forced Terminate (SIGKILL):   {}", count),
        }
    }

    pub fn report_failed(count: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("* 失败/权限拒绝:             {} 个", count),
            Language::ZhHant => format!("* 失敗/權限拒絕:             {} 個", count),
            Language::Ja => format!("* 失敗/アクセス拒否:         {}", count),
            _ => format!("* Failed / Permission Denied:   {}", count),
        }
    }

    pub fn report_duration(ms: f64, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("* 清场全流程总耗时:         {:.2}ms", ms),
            Language::ZhHant => format!("* 清場全流程總耗時:         {:.2}ms", ms),
            Language::Ja => format!("* 終了処理全所要時間:       {:.2}ms", ms),
            _ => format!("* Total Execution Duration:     {:.2}ms", ms),
        }
    }

    pub fn report_cols(lang: Language) -> (&'static str, &'static str, &'static str, &'static str) {
        match lang {
            Language::ZhHans => ("PID", "应用名称", "退出状态", "所用信号"),
            Language::ZhHant => ("PID", "應用程式名稱", "結束狀態", "使用信號"),
            Language::Ja => ("PID", "アプリ名", "終了状態", "シグナル"),
            _ => ("PID", "App Name", "Exit Status", "Signal Used"),
        }
    }

    // ========================================================
    // 交互式任务向导多语言支持
    // ========================================================

    pub fn interactive_title(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "              macOS Task Cleaner - 交互式任务向导           ",
            Language::ZhHant => "              macOS Task Cleaner - 互動式任務精靈           ",
            Language::Ja => "              macOS Task Cleaner - 対話型クリーンウィザード   ",
            _ => "              macOS Task Cleaner - Interactive Task Wizard  ",
        }
    }

    pub fn interactive_overview(total: usize, protected: usize, targets: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!(
                "[状态概览] 前台应用总计: {} 个 | 白名单已豁免: {} 个 | 待处置目标: {} 个",
                total, protected, targets
            ),
            Language::ZhHant => format!(
                "[狀態概覽] 幕前應用程式總計: {} 個 | 白名單已豁免: {} 個 | 待處理目標: {} 個",
                total, protected, targets
            ),
            Language::Ja => format!(
                "[状態概要] フォアグラウンドアプリ: {} | ホワイトリスト除外: {} | 整理対象: {}",
                total, protected, targets
            ),
            _ => format!(
                "[Overview] Foreground Apps: {} | Whitelisted: {} | Targets to Process: {}",
                total, protected, targets
            ),
        }
    }

    pub fn interactive_config_file(path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[配置文件] {}", path),
            Language::ZhHant => format!("[設定檔] {}", path),
            Language::Ja => format!("[設定ファイル] {}", path),
            _ => format!("[Config File] {}", path),
        }
    }

    pub fn interactive_config_default(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[配置文件] 内置默认规则 (未检测到外部配置文件)",
            Language::ZhHant => "[設定檔] 內建預設規則 (未偵測到外部設定檔)",
            Language::Ja => "[設定ファイル] 組み込みデフォルト規則 (外部設定なし)",
            _ => "[Config File] Built-in default rules (no external config detected)",
        }
    }

    pub fn interactive_all_protected(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[提示] 当前所有前台应用均已在白名单保护中，无待清理目标。",
            Language::ZhHant => "[提示] 目前所有幕前應用程式均已在白名單保護中，無待清理目標。",
            Language::Ja => "[ヒント] 現在すべてのフォアグラウンドアプリがホワイトリストで保護されています。終了対象はありません。",
            _ => "[Notice] All foreground applications are currently protected by whitelist rules; no cleanup needed.",
        }
    }

    pub fn interactive_empty_menu(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "操作指令:\n  p / protected   查看当前已被保护的应用清单\n  r / refresh     重新扫描系统前台应用\n  q / quit        退出程序",
            Language::ZhHant => "操作指令:\n  p / protected   查看目前已被保護的應用程式清單\n  r / refresh     重新掃描系統幕前應用程式\n  q / quit        結束程式",
            Language::Ja => "コマンド:\n  p / protected   保護中のアプリ一覧を表示\n  r / refresh     フォアグラウンドアプリを再スキャン\n  q / quit        終了",
            _ => "Commands:\n  p / protected   View list of currently protected applications\n  r / refresh     Rescan system foreground applications\n  q / quit        Exit program",
        }
    }

    pub fn interactive_prompt(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "请输入操作指令 > ",
            Language::ZhHant => "請輸入操作指令 > ",
            Language::Ja => "コマンドを入力してください > ",
            _ => "Enter command > ",
        }
    }

    pub fn interactive_exited(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[已退出] 未执行任何操作。",
            Language::ZhHant => "[已退出] 未執行任何操作。",
            Language::Ja => "[終了] 操作は実行されませんでした。",
            _ => "[Exited] No operations performed.",
        }
    }

    pub fn interactive_invalid_prompt(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[提示] 输入无效，请输入 p / r / q。",
            Language::ZhHant => "[提示] 輸入無效，請輸入 p / r / q。",
            Language::Ja => "[ヒント] 無効な入力です。p / r / q を入力してください。",
            _ => "[Hint] Invalid input. Please enter p / r / q.",
        }
    }

    pub fn interactive_targets_header(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[待清场前台应用清单]:",
            Language::ZhHant => "[待清場幕前應用程式清單]:",
            Language::Ja => "[終了対象フォアグラウンドアプリ一覧]:",
            _ => "[Foreground Applications to Clean]:",
        }
    }

    pub fn interactive_targets_cols(lang: Language) -> (&'static str, &'static str, &'static str, &'static str) {
        match lang {
            Language::ZhHans => ("序号", "PID", "应用名称", "Bundle ID"),
            Language::ZhHant => ("序號", "PID", "應用程式名稱", "Bundle ID"),
            Language::Ja => ("番号", "PID", "アプリ名", "Bundle ID"),
            _ => ("No.", "PID", "App Name", "Bundle ID"),
        }
    }

    pub fn interactive_guide(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "操作指令指南:\n  w [编号...]   将指定应用永久加入配置文件白名单 (例: w 1, 2 或 w 1 3)\n  t [编号...]   在本轮清场中临时跳过/豁免 (例: t 1)\n  c / clean     确认执行标准终止 (向目标发送 SIGTERM，超时升级为 SIGKILL)\n  f / force     立即强制终止 (跳过宽限期，直接发送 SIGKILL)\n  p / protected 查看并管理当前已被保护的应用清单 (可移出白名单)\n  r / refresh   重新扫描系统前台应用\n  q / quit      取消并安全退出",
            Language::ZhHant => "操作指令指南:\n  w [序號...]   將指定應用程式永久加入設定檔白名單 (例: w 1, 2 或 w 1 3)\n  t [序號...]   在本輪清場中臨時略過/豁免 (例: t 1)\n  c / clean     確認執行標準結束 (向目標發送 SIGTERM，超時升級為 SIGKILL)\n  f / force     立即強制結束 (跳過寬限期，直接發送 SIGKILL)\n  p / protected 查看並管理目前已被保護的應用程式清單 (可移出白名單)\n  r / refresh   重新掃描系統幕前應用程式\n  q / quit      取消並安全結束",
            Language::Ja => "コマンドガイド:\n  w [番号...]   指定アプリを設定ファイルホワイトリストに永続追加 (例: w 1, 2 または w 1 3)\n  t [番号...]   このセッションのみ一時的にスキップ (例: t 1)\n  c / clean     標準終了を実行 (SIGTERM送信後、タイムアウトでSIGKILL)\n  f / force     即時強制終了 (猶予期間をスキップし直接SIGKILL送信)\n  p / protected 現在の保護アプリ一覧を表示・管理 (除外解除可能)\n  r / refresh   フォアグラウンドアプリを再スキャン\n  q / quit      キャンセルして安全に終了",
            _ => "Commands Guide:\n  w [no...]     Permanently add specified apps to config whitelist (e.g.: w 1, 2 or w 1 3)\n  t [no...]     Temporarily skip in this session (e.g.: t 1)\n  c / clean     Confirm standard termination (SIGTERM -> SIGKILL escalation on timeout)\n  f / force     Immediate forced termination (skip grace period, send SIGKILL directly)\n  p / protected View & manage protected apps list (allows removing from whitelist)\n  r / refresh   Rescan system foreground applications\n  q / quit      Cancel and safely exit",
        }
    }

    pub fn interactive_refreshing(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[刷新] 正在重新扫描前台应用...",
            Language::ZhHant => "[重新整理] 正在重新掃描幕前應用程式...",
            Language::Ja => "[更新] フォアグラウンドアプリを再スキャン中...",
            _ => "[Refresh] Rescanning foreground applications...",
        }
    }

    pub fn interactive_confirm_clean(count: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("确认执行标准终止 (SIGTERM) 上述 {} 个应用?", count),
            Language::ZhHant => format!("確認執行標準結束 (SIGTERM) 上述 {} 個應用程式?", count),
            Language::Ja => format!("上記 {} 個のアプリに対して標準終了 (SIGTERM) を実行しますか?", count),
            _ => format!("Confirm standard termination (SIGTERM) of {} applications?", count),
        }
    }

    pub fn interactive_starting_clean(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "\n[开始执行标准终止]...",
            Language::ZhHant => "\n[開始執行標準結束]...",
            Language::Ja => "\n[標準終了を開始中]...",
            _ => "\n[Starting standard termination]...",
        }
    }

    pub fn interactive_confirm_force(count: usize, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("警告: 确认直接强制终止 (SIGKILL) 上述 {} 个应用?", count),
            Language::ZhHant => format!("警告: 確認直接強制結束 (SIGKILL) 上述 {} 個應用程式?", count),
            Language::Ja => format!("警告: 上記 {} 個のアプリを直接強制終了 (SIGKILL) しますか?", count),
            _ => format!("Warning: Confirm forced termination (SIGKILL) of {} applications?", count),
        }
    }

    pub fn interactive_starting_force(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "\n[开始执行强制终止]...",
            Language::ZhHant => "\n[開始執行強制結束]...",
            Language::Ja => "\n[強制終了を開始中]...",
            _ => "\n[Starting forced termination]...",
        }
    }

    pub fn interactive_cancelled(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[操作已取消]",
            Language::ZhHant => "[操作已取消]",
            Language::Ja => "[操作はキャンセルされました]",
            _ => "[Operation cancelled]",
        }
    }

    pub fn interactive_invalid_indices(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[提示] 请指定有效序号，例如: w 1 或 w 1, 2",
            Language::ZhHant => "[提示] 請指定有效序號，例如: w 1 或 w 1, 2",
            Language::Ja => "[ヒント] 有効な番号を指定してください (例: w 1 または w 1, 2)",
            _ => "[Hint] Please specify valid indices, e.g.: w 1 or w 1, 2",
        }
    }

    pub fn interactive_config_saved(count: usize, path: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("\n[配置更新成功] 已将 {} 个应用持久化写入白名单: {}", count, path),
            Language::ZhHant => format!("\n[設定更新成功] 已將 {} 個應用程式持久化寫入白名單: {}", count, path),
            Language::Ja => format!("\n[設定更新成功] {} 個のアプリをホワイトリストに保存しました: {}", count, path),
            _ => format!("\n[Config Updated] Successfully persisted {} application(s) to whitelist: {}", count, path),
        }
    }

    pub fn interactive_temp_skip(name: &str, bundle_id: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[临时豁免] 本轮清场将跳过: {} ({})", name, bundle_id),
            Language::ZhHant => format!("[臨時豁免] 本輪清場將略過: {} ({})", name, bundle_id),
            Language::Ja => format!("[一時除外] このクリーンセッションではスキップします: {} ({})", name, bundle_id),
            _ => format!("[Temporary Exemption] Skipping in current session: {} ({})", name, bundle_id),
        }
    }

    pub fn interactive_quick_select(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[快捷选择] 检测到序号输入，将其加入永久白名单:",
            Language::ZhHant => "[快捷選擇] 偵測到序號輸入，將其加入永久白名單:",
            Language::Ja => "[クイック選択] 番号が入力されたため、永続ホワイトリストに追加します:",
            _ => "[Quick Select] Number input detected; adding to permanent whitelist:",
        }
    }

    pub fn interactive_unknown_cmd(cmd: &str, lang: Language) -> String {
        match lang {
            Language::ZhHans => format!("[提示] 未知指令: '{}'。请输入 w / t / c / f / p / r / q", cmd),
            Language::ZhHant => format!("[提示] 未知指令: '{}'。請輸入 w / t / c / f / p / r / q", cmd),
            Language::Ja => format!("[ヒント] 未知のコマンド: '{}'。w / t / c / f / p / r / q を入力してください", cmd),
            _ => format!("[Hint] Unknown command: '{}'. Please enter w / t / c / f / p / r / q", cmd),
        }
    }

    pub fn protected_view_title(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "[当前受保护应用清单]",
            Language::ZhHant => "[目前受保護應用程式清單]",
            Language::Ja => "[現在の保護対象アプリ一覧]",
            _ => "[Currently Protected Applications]",
        }
    }

    pub fn protected_view_cols(lang: Language) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
        match lang {
            Language::ZhHans => ("序号", "PID", "应用名称", "保护层级", "豁免规则"),
            Language::ZhHant => ("序號", "PID", "應用程式名稱", "保護層級", "豁免規則"),
            Language::Ja => ("番号", "PID", "アプリ名", "保護レベル", "除外規則"),
            _ => ("No.", "PID", "App Name", "Protection Tier", "Rule"),
        }
    }

    pub fn protected_view_guide(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "操作指令:\n  u [序号...]   将指定应用移出白名单并加入禁用规则 (例: u 1 或 u 1, 2)\n  按回车键直接返回主菜单",
            Language::ZhHant => "操作指令:\n  u [序號...]   將指定應用程式移出白名單並加入停用規則 (例: u 1 或 u 1, 2)\n  按 Enter 鍵直接返回主選單",
            Language::Ja => "コマンド:\n  u [番号...]   指定アプリをホワイトリストから除外し無効ルールに登録 (例: u 1 または u 1, 2)\n  Enterキーを押すとメインメニューに戻ります",
            _ => "Commands:\n  u [no...]     Remove specified apps from whitelist and record to disabled rules (e.g.: u 1 or u 1, 2)\n  Press Enter to return to main menu",
        }
    }

    pub fn pause_prompt(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "\n按回车键继续...",
            Language::ZhHant => "\n按 Enter 鍵繼續...",
            Language::Ja => "\nEnterキーを押して続行...",
            _ => "\nPress Enter to continue...",
        }
    }

    pub fn purge_running(lang: Language) -> &'static str {
        match lang {
            Language::ZhHans => "\n[内存回收] 正在执行 /usr/sbin/purge...",
            Language::ZhHant => "\n[記憶體釋放] 正在執行 /usr/sbin/purge...",
            Language::Ja => "\n[メモリ解放] /usr/sbin/purge を実行中...",
            _ => "\n[Memory Purge] Running /usr/sbin/purge...",
        }
    }
}
