mod i18n;
mod interactive;
mod preview;

use std::env;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use i18n::CliMessages;
use interactive::{run_interactive_session, run_language_select_menu};
use macos_task_cleaner_core::{
    scan_foreground_apps, terminate_with_mode_with_lang, AppTarget, Language, TerminationMode,
    WhitelistManager,
};
use preview::{render_dry_run_preview, render_execution_report};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Debug, Clone)]
struct CliArgs {
    dry_run: Option<bool>,
    force: bool,
    interactive: bool,
    purge: bool,
    json: bool,
    config_path: Option<PathBuf>,
    lang: Option<Language>,
    set_lang_code: Option<String>,
    select_lang_menu: bool,
    cli_keeps: Vec<String>,
    add_whitelist: Vec<String>,
    remove_whitelist: Vec<String>,
    terminate_pids: Vec<i32>,
    init_config: bool,
    show_help: bool,
    show_version: bool,
    has_explicit_action: bool,
}

fn parse_cli_args_from<I, T>(args_iter: I) -> Result<CliArgs, String>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut args = args_iter.into_iter().map(Into::into).peekable();
    let mut cli = CliArgs::default();
    let mut active_lang = Language::detect_system();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" | "--dry-run" => {
                cli.dry_run = Some(true);
                cli.has_explicit_action = true;
            }
            "-e" | "--execute" => {
                cli.dry_run = Some(false);
                cli.has_explicit_action = true;
            }
            "-i" | "--interactive" => {
                cli.interactive = true;
                cli.has_explicit_action = true;
            }
            "-f" | "--force" => {
                cli.force = true;
                cli.dry_run = Some(false);
                cli.has_explicit_action = true;
            }
            "-p" | "--purge" => {
                cli.purge = true;
                cli.has_explicit_action = true;
            }
            "--json" => {
                cli.json = true;
            }
            "--init-config" => {
                cli.init_config = true;
                cli.has_explicit_action = true;
            }
            "-l" | "--lang" => {
                let next_arg = args.peek().cloned();
                match next_arg {
                    None => {
                        cli.select_lang_menu = true;
                    }
                    Some(ref val) if val.starts_with('-') => {
                        cli.select_lang_menu = true;
                    }
                    Some(ref val)
                        if val.eq_ignore_ascii_case("menu")
                            || val.eq_ignore_ascii_case("select")
                            || val.eq_ignore_ascii_case("wizard")
                            || val.eq_ignore_ascii_case("list") =>
                    {
                        args.next();
                        cli.select_lang_menu = true;
                    }
                    Some(val) => {
                        args.next();
                        let parsed = Language::from_locale_str(&val);
                        active_lang = parsed;
                        cli.lang = Some(parsed);
                        cli.set_lang_code = Some(val);
                    }
                }
            }
            "-c" | "--config" => {
                if let Some(val) = args.next() {
                    cli.config_path = Some(PathBuf::from(val));
                } else {
                    return Err(CliMessages::err_missing_value("--config", active_lang));
                }
            }
            "-k" | "--keep" => {
                if let Some(val) = args.next() {
                    cli.cli_keeps.push(val);
                } else {
                    return Err(CliMessages::err_missing_value("--keep", active_lang));
                }
            }
            "-a" | "--add-whitelist" => {
                if let Some(val) = args.next() {
                    cli.add_whitelist.push(val);
                    cli.has_explicit_action = true;
                } else {
                    return Err(CliMessages::err_missing_value("--add-whitelist", active_lang));
                }
            }
            "-r" | "--remove-whitelist" | "--unprotect" => {
                if let Some(val) = args.next() {
                    cli.remove_whitelist.push(val);
                    cli.has_explicit_action = true;
                } else {
                    return Err(CliMessages::err_missing_value("--remove-whitelist", active_lang));
                }
            }
            "-t" | "--terminate-pid" | "--kill-pid" => {
                if let Some(val) = args.next() {
                    if let Ok(pid) = val.parse::<i32>() {
                        cli.terminate_pids.push(pid);
                        cli.has_explicit_action = true;
                    } else {
                        return Err(CliMessages::err_invalid_pid(&val, active_lang));
                    }
                } else {
                    return Err(CliMessages::err_missing_value("--terminate-pid", active_lang));
                }
            }
            "-h" | "--help" => {
                cli.show_help = true;
            }
            "-v" | "--version" => {
                cli.show_version = true;
            }
            unknown => {
                return Err(CliMessages::err_unknown_arg(unknown, active_lang));
            }
        }
    }

    Ok(cli)
}

fn parse_cli_args() -> Result<CliArgs, String> {
    parse_cli_args_from(env::args().skip(1))
}

fn print_help(lang: Language) {
    println!("{}", CliMessages::help_text(VERSION, lang));
}

fn main() {
    let cli = match parse_cli_args() {
        Ok(args) => args,
        Err(e) => {
            let lang = Language::detect_system();
            eprintln!("[ERROR] {}", e);
            eprintln!("{}", CliMessages::err_hint_help(lang));
            std::process::exit(1);
        }
    };

    // 1. 初始化并加载基础配置及白名单
    let (whitelist, config) =
        WhitelistManager::new(cli.config_path.as_deref(), &cli.cli_keeps);

    // 2. 语言决策优先级: CLI 显式参数 > 配置文件设定 > 终端环境变量 (LC_ALL/LC_MESSAGES/LANG) > macOS 全局偏好
    let lang = cli
        .lang
        .or_else(|| config.general.language.as_deref().map(Language::from_locale_str))
        .unwrap_or_else(Language::detect_system);

    if cli.show_help {
        print_help(lang);
        return;
    }

    if cli.show_version {
        println!("{}", CliMessages::version_text(VERSION, lang));
        return;
    }

    // 3. 语言切换向导菜单 (-l / --lang / --lang menu)
    if cli.select_lang_menu {
        run_language_select_menu(cli.config_path.as_deref(), lang);
        return;
    }

    // 3.5 独立设置语言配置 (--lang <CODE> 且未指定具体执行动作)
    // 安全防护：防止用户只想设定语言偏好时意外触发进程清理！
    if let Some(ref code) = cli.set_lang_code {
        if !cli.has_explicit_action {
            match WhitelistManager::set_language_in_config(cli.config_path.as_deref(), code) {
                Ok((saved_path, _)) => {
                    let path_str = saved_path.display().to_string();
                    let target_lang = Language::from_locale_str(code);
                    if code.eq_ignore_ascii_case("auto") {
                        let detected = Language::detect_system();
                        println!(
                            "{}",
                            CliMessages::lang_auto_saved_notice(&path_str, detected.code(), target_lang)
                        );
                    } else {
                        let name = format!("{:?} ({})", target_lang, target_lang.code());
                        println!(
                            "{}",
                            CliMessages::lang_saved_notice(&name, &path_str, target_lang)
                        );
                    }
                    return;
                }
                Err(e) => {
                    eprintln!("[ERROR] {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // 4. 处理配置文件初始化 (--init-config)
    if cli.init_config {
        match WhitelistManager::generate_default_config_file(cli.config_path.as_deref()) {
            Ok(path) => {
                println!("{}", CliMessages::config_init_success(&path.display().to_string(), lang));
                return;
            }
            Err(e) => {
                eprintln!("{}", CliMessages::config_init_failed(&e.to_string(), lang));
                std::process::exit(1);
            }
        }
    }

    // 5. 处理直接追加白名单 (-a / --add-whitelist)
    if !cli.add_whitelist.is_empty() {
        for ident in &cli.add_whitelist {
            match WhitelistManager::add_identifier_to_config(cli.config_path.as_deref(), ident) {
                Ok((saved_path, is_new)) => {
                    let path_str = saved_path.display().to_string();
                    if is_new {
                        println!("{}", CliMessages::whitelist_add_success(ident, &path_str, lang));
                    } else {
                        println!("{}", CliMessages::whitelist_add_exists(ident, &path_str, lang));
                    }
                }
                Err(e) => {
                    eprintln!("{}", CliMessages::whitelist_add_failed(ident, &e.to_string(), lang));
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    // 6. 处理移除白名单 (-r / --remove-whitelist)
    if !cli.remove_whitelist.is_empty() {
        for ident in &cli.remove_whitelist {
            match WhitelistManager::remove_identifier_from_config(cli.config_path.as_deref(), ident) {
                Ok((saved_path, is_modified)) => {
                    let path_str = saved_path.display().to_string();
                    if is_modified {
                        println!("{}", CliMessages::whitelist_remove_success(ident, &path_str, lang));
                    } else {
                        println!("{}", CliMessages::whitelist_remove_unchanged(ident, &path_str, lang));
                    }
                }
                Err(e) => {
                    eprintln!("{}", CliMessages::whitelist_remove_failed(ident, &e.to_string(), lang));
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    let grace_period = Duration::from_millis(config.general.grace_period_ms);

    // 3.5 处理指定 PID 单独终止 (-t / --terminate-pid)
    if !cli.terminate_pids.is_empty() {
        let all_apps = scan_foreground_apps();
        let mut targets_to_kill = Vec::new();
        for &pid in &cli.terminate_pids {
            if let Some(app) = all_apps.iter().find(|a| a.pid == pid) {
                targets_to_kill.push(app.clone());
            } else {
                targets_to_kill.push(AppTarget {
                    pid,
                    name: format!("PID:{}", pid),
                    bundle_id: String::new(),
                });
            }
        }
        let mode = if cli.force {
            TerminationMode::ForceImmediate
        } else {
            TerminationMode::Standard
        };
        let report = terminate_with_mode_with_lang(&targets_to_kill, grace_period, mode, lang);
        if cli.json {
            println!("{}", serde_json::to_string(&report).unwrap_or_default());
        } else {
            for rec in &report.records {
                if rec.error_msg.is_none() {
                    println!("{}", CliMessages::term_pid_success(&rec.app.name, rec.app.pid, lang));
                } else {
                    eprintln!(
                        "{}",
                        CliMessages::term_pid_failed(
                            &rec.app.name,
                            rec.app.pid,
                            rec.error_msg.as_deref().unwrap_or("-"),
                            lang
                        )
                    );
                }
            }
        }
        if report.failed > 0 {
            std::process::exit(1);
        }
        return;
    }

    // 4. 交互式模式 (-i / --interactive)
    if cli.interactive {
        run_interactive_session(
            cli.config_path.as_deref(),
            &cli.cli_keeps,
            grace_period,
            cli.purge,
            lang,
        );
        return;
    }

    // 5. 非交互式流程
    let scan_start = Instant::now();

    // 确定运行模式：CLI 显式指定 > 配置文件指定 > 默认 true
    let is_dry_run = cli
        .dry_run
        .unwrap_or(config.general.default_dry_run);

    // 扫描前台 GUI 应用
    let scanned_apps = scan_foreground_apps();

    // 应用白名单多级过滤网 (传入当前语言以获得本地化标签)
    let mut protected_list = Vec::new();
    let mut target_list = Vec::new();

    for app in scanned_apps.iter() {
        if let Some(matched) = whitelist.check_protection_with_lang(app, lang) {
            protected_list.push((app.clone(), matched));
        } else {
            target_list.push(app.clone());
        }
    }

    let scan_duration_ms = scan_start.elapsed().as_secs_f64() * 1000.0;

    // 预检模式 (Dry-Run)
    if is_dry_run {
        render_dry_run_preview(
            &scanned_apps,
            &protected_list,
            &target_list,
            scan_duration_ms,
            whitelist.loaded_config_path.as_deref(),
            cli.json,
            lang,
        );
        return;
    }

    // 实质执行终止
    let mode = if cli.force {
        TerminationMode::ForceImmediate
    } else if cli.purge {
        TerminationMode::StandardWithPurge
    } else {
        TerminationMode::Standard
    };
    let report = terminate_with_mode_with_lang(&target_list, grace_period, mode, lang);
    render_execution_report(&report, cli.json, lang);

    // 可选内存整理反馈 (--purge)
    if cli.purge && !cli.json {
        if report.cache_purged {
            println!("\n{}", CliMessages::purge_success(lang));
        } else {
            eprintln!("\n{}", CliMessages::purge_warn(lang));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cli_args_lang_flag() {
        let args = vec!["--lang", "en", "-n"];
        let cli = parse_cli_args_from(args).expect("parse should succeed");
        assert_eq!(cli.lang, Some(Language::En));
        assert_eq!(cli.dry_run, Some(true));
        assert!(cli.has_explicit_action);

        let args_zh = vec!["-l", "zh-Hant", "-e"];
        let cli_zh = parse_cli_args_from(args_zh).expect("parse should succeed");
        assert_eq!(cli_zh.lang, Some(Language::ZhHant));
        assert_eq!(cli_zh.dry_run, Some(false));
        assert!(cli_zh.has_explicit_action);

        let args_ja = vec!["--lang", "ja", "--interactive"];
        let cli_ja = parse_cli_args_from(args_ja).expect("parse should succeed");
        assert_eq!(cli_ja.lang, Some(Language::Ja));
        assert!(cli_ja.interactive);
        assert!(cli_ja.has_explicit_action);
    }

    #[test]
    fn test_parse_cli_args_lang_menu() {
        let args_empty = vec!["--lang"];
        let cli_empty = parse_cli_args_from(args_empty).expect("parse should succeed");
        assert!(cli_empty.select_lang_menu);

        let args_short = vec!["-l"];
        let cli_short = parse_cli_args_from(args_short).expect("parse should succeed");
        assert!(cli_short.select_lang_menu);

        let args_menu = vec!["--lang", "menu"];
        let cli_menu = parse_cli_args_from(args_menu).expect("parse should succeed");
        assert!(cli_menu.select_lang_menu);
    }

    #[test]
    fn test_parse_cli_args_lang_standalone() {
        let args = vec!["--lang", "en"];
        let cli = parse_cli_args_from(args).expect("parse should succeed");
        assert_eq!(cli.lang, Some(Language::En));
        assert_eq!(cli.set_lang_code, Some("en".to_string()));
        assert!(!cli.has_explicit_action);
    }

    #[test]
    fn test_help_text_multilingual() {
        let help_en = CliMessages::help_text(VERSION, Language::En);
        assert!(help_en.contains("Usage:"));
        assert!(help_en.contains("Whitelist Defense Matrix:"));
        assert!(help_en.contains("--lang <LANG>"));

        let help_zh = CliMessages::help_text(VERSION, Language::ZhHans);
        assert!(help_zh.contains("用法:"));
        assert!(help_zh.contains("白名单分级体系:"));

        let help_ja = CliMessages::help_text(VERSION, Language::Ja);
        assert!(help_ja.contains("使用方法:"));
        assert!(help_ja.contains("ホワイトリスト階層システム:"));

        let help_zht = CliMessages::help_text(VERSION, Language::ZhHant);
        assert!(help_zht.contains("用法:"));
        assert!(help_zht.contains("白名單分級體系:"));
    }

    #[test]
    fn test_version_multilingual() {
        let ver_en = CliMessages::version_text(VERSION, Language::En);
        assert!(ver_en.contains("All rights reserved"));

        let ver_zh = CliMessages::version_text(VERSION, Language::ZhHans);
        assert!(ver_zh.contains("保留所有权利"));
    }

    #[test]
    fn test_error_messages_multilingual() {
        let err_en = CliMessages::err_missing_value("--config", Language::En);
        assert_eq!(err_en, "Missing value for argument --config");

        let err_zh = CliMessages::err_missing_value("--config", Language::ZhHans);
        assert_eq!(err_zh, "缺少 --config 参数值");

        let pid_err_en = CliMessages::err_invalid_pid("abc", Language::En);
        assert_eq!(pid_err_en, "Invalid PID: abc");

        let pid_err_ja = CliMessages::err_invalid_pid("abc", Language::Ja);
        assert_eq!(pid_err_ja, "無効な PID です: abc");
    }

    #[test]
    fn test_dry_run_strings_multilingual() {
        let stats_en = CliMessages::dry_run_stats(5, 2, 3, Language::En);
        assert!(stats_en.contains("Scanned Foreground Apps: 5"));

        let stats_zh = CliMessages::dry_run_stats(5, 2, 3, Language::ZhHans);
        assert!(stats_zh.contains("发现前台图形应用: 5 个"));

        let (c1, _, _, _) = CliMessages::dry_run_targets_cols(Language::Ja);
        assert_eq!(c1, "PID");
    }
}

