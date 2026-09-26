use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

use crate::i18n::CliMessages;
use crate::preview::render_execution_report;
use macos_task_cleaner_core::{
    scan_foreground_apps, tiered_terminate_with_lang, AppTarget, Language, WhitelistManager,
    WhitelistMatch,
};

/// 启动交互式任务向导会话 (-i / --interactive)
pub fn run_interactive_session(
    custom_config_path: Option<&Path>,
    cli_keeps: &[String],
    grace_period: Duration,
    do_purge: bool,
    mut lang: Language,
) {
    let mut in_memory_keeps = cli_keeps.to_vec();

    loop {
        // 1. 初始化/刷新白名单
        let (whitelist, _) = WhitelistManager::new(custom_config_path, &in_memory_keeps);

        // 2. 扫描当前系统前台 GUI 应用
        let all_apps = scan_foreground_apps();

        let mut protected_list = Vec::new();
        let mut target_list: Vec<AppTarget> = Vec::new();

        for app in &all_apps {
            if let Some(matched) = whitelist.check_protection_with_lang(app, lang) {
                protected_list.push((app.clone(), matched));
            } else {
                target_list.push(app.clone());
            }
        }

        println!();
        println!("============================================================");
        println!("{}", CliMessages::interactive_title(lang));
        println!("============================================================");
        println!(
            "{}",
            CliMessages::interactive_overview(all_apps.len(), protected_list.len(), target_list.len(), lang)
        );
        if let Some(ref p) = whitelist.loaded_config_path {
            println!("{}", CliMessages::interactive_config_file(&p.display().to_string(), lang));
        } else {
            println!("{}", CliMessages::interactive_config_default(lang));
        }
        println!();

        if target_list.is_empty() {
            println!("{}", CliMessages::interactive_all_protected(lang));
            println!("------------------------------------------------------------");
            println!("{}", CliMessages::interactive_empty_menu(lang));
            println!("------------------------------------------------------------");
            print!("{}", CliMessages::interactive_prompt(lang));
            let _ = io::stdout().flush();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            let trimmed = input.trim().to_lowercase();
            match trimmed.as_str() {
                "p" | "protected" => {
                    handle_protected_apps(custom_config_path, &protected_list, lang);
                }
                "l" | "lang" => {
                    if let Some(new_lang) = run_language_select_menu(custom_config_path, lang) {
                        lang = new_lang;
                    }
                    continue;
                }
                "r" | "refresh" => {
                    continue;
                }
                "q" | "quit" | "exit" => {
                    println!("{}", CliMessages::interactive_exited(lang));
                    break;
                }
                _ => {
                    println!("{}", CliMessages::interactive_invalid_prompt(lang));
                }
            }
            continue;
        }

        // 显示待清场目标清单 (带 1-indexed 编号)
        println!("{}", CliMessages::interactive_targets_header(lang));
        let (c_no, c_pid, c_name, c_bid) = CliMessages::interactive_targets_cols(lang);
        println!("{:<4} {:<7} {:<20} {}", c_no, c_pid, c_name, c_bid);
        println!("{:-<4} {:-<7} {:-<20} {:-<30}", "", "", "", "");
        for (idx, target) in target_list.iter().enumerate() {
            let display_name = CliMessages::resolve_app_name(&target.bundle_id, &target.name, lang);
            println!(
                "[{:>2}] {:<7} {:<20} {}",
                idx + 1,
                target.pid,
                display_name,
                target.bundle_id
            );
        }

        println!("------------------------------------------------------------");
        println!("{}", CliMessages::interactive_guide(lang));
        println!("------------------------------------------------------------");
        print!("{}", CliMessages::interactive_prompt(lang));
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        // 解析指令
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();

        match cmd.as_str() {
            "q" | "quit" | "exit" => {
                println!("{}", CliMessages::interactive_exited(lang));
                break;
            }
            "r" | "refresh" => {
                println!("{}", CliMessages::interactive_refreshing(lang));
                continue;
            }
            "p" | "protected" => {
                handle_protected_apps(custom_config_path, &protected_list, lang);
                continue;
            }
            "l" | "lang" => {
                if let Some(new_lang) = run_language_select_menu(custom_config_path, lang) {
                    lang = new_lang;
                }
                continue;
            }
            "c" | "clean" => {
                if confirm_action(&CliMessages::interactive_confirm_clean(target_list.len(), lang)) {
                    println!("{}", CliMessages::interactive_starting_clean(lang));
                    let report = tiered_terminate_with_lang(&target_list, grace_period, false, lang);
                    render_execution_report(&report, false, lang);

                    if do_purge {
                        execute_purge(lang);
                    }
                    break;
                } else {
                    println!("{}", CliMessages::interactive_cancelled(lang));
                }
            }
            "f" | "force" => {
                if confirm_action(&CliMessages::interactive_confirm_force(target_list.len(), lang)) {
                    println!("{}", CliMessages::interactive_starting_force(lang));
                    let report = tiered_terminate_with_lang(&target_list, grace_period, true, lang);
                    render_execution_report(&report, false, lang);

                    if do_purge {
                        execute_purge(lang);
                    }
                    break;
                } else {
                    println!("{}", CliMessages::interactive_cancelled(lang));
                }
            }
            "w" | "whitelist" => {
                let indices = parse_indices(&parts[1..], target_list.len());
                if indices.is_empty() {
                    println!("{}", CliMessages::interactive_invalid_indices(lang));
                    continue;
                }

                let mut bundle_ids_to_add = Vec::new();
                let mut names_to_add = Vec::new();

                for idx in indices {
                    let target = &target_list[idx];
                    if !target.bundle_id.is_empty() {
                        bundle_ids_to_add.push(target.bundle_id.clone());
                    } else {
                        names_to_add.push(target.name.clone());
                    }
                }

                match WhitelistManager::append_to_user_config(
                    custom_config_path,
                    &bundle_ids_to_add,
                    &names_to_add,
                ) {
                    Ok((saved_path, added_count)) => {
                        println!(
                            "{}",
                            CliMessages::interactive_config_saved(added_count, &saved_path.display().to_string(), lang)
                        );
                        for bid in &bundle_ids_to_add {
                            println!("  * Bundle ID: {}", bid);
                        }
                        for name in &names_to_add {
                            println!("  * Name: {}", name);
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", CliMessages::config_init_failed(&e.to_string(), lang));
                    }
                }
                pause_prompt(lang);
            }
            "t" | "temp" => {
                let indices = parse_indices(&parts[1..], target_list.len());
                if indices.is_empty() {
                    println!("{}", CliMessages::interactive_invalid_indices(lang));
                    continue;
                }
                for idx in indices {
                    let target = &target_list[idx];
                    in_memory_keeps.push(target.bundle_id.clone());
                    println!(
                        "{}",
                        CliMessages::interactive_temp_skip(&target.name, &target.bundle_id, lang)
                    );
                }
                pause_prompt(lang);
            }
            _ => {
                // 检查用户是否直接输入了纯数字（如 "1" 或 "1, 2"），便捷默认等同于加入白名单
                let indices = parse_indices(&parts[..], target_list.len());
                if !indices.is_empty() {
                    println!("{}", CliMessages::interactive_quick_select(lang));
                    let mut bundle_ids_to_add = Vec::new();
                    let mut names_to_add = Vec::new();

                    for idx in indices {
                        let target = &target_list[idx];
                        if !target.bundle_id.is_empty() {
                            bundle_ids_to_add.push(target.bundle_id.clone());
                        } else {
                            names_to_add.push(target.name.clone());
                        }
                    }

                    match WhitelistManager::append_to_user_config(
                        custom_config_path,
                        &bundle_ids_to_add,
                        &names_to_add,
                    ) {
                        Ok((saved_path, added_count)) => {
                            println!(
                                "{}",
                                CliMessages::interactive_config_saved(added_count, &saved_path.display().to_string(), lang)
                            );
                            for bid in &bundle_ids_to_add {
                                println!("  * Bundle ID: {}", bid);
                            }
                            for name in &names_to_add {
                                println!("  * Name: {}", name);
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", CliMessages::config_init_failed(&e.to_string(), lang));
                        }
                    }
                    pause_prompt(lang);
                } else {
                    println!("{}", CliMessages::interactive_unknown_cmd(trimmed, lang));
                }
            }
        }
    }
}

fn handle_protected_apps(
    custom_config_path: Option<&Path>,
    protected_list: &[(AppTarget, WhitelistMatch)],
    lang: Language,
) {
    println!();
    println!("------------------------------------------------------------");
    println!("{}", CliMessages::protected_view_title(lang));
    println!("------------------------------------------------------------");
    let (c_no, c_pid, c_name, c_tier, c_rule) = CliMessages::protected_view_cols(lang);
    println!("{:<4} {:<7} {:<18} {:<16} {}", c_no, c_pid, c_name, c_tier, c_rule);
    println!("{:-<4} {:-<7} {:-<18} {:-<16} {:-<20}", "", "", "", "", "");
    for (idx, (app, matched)) in protected_list.iter().enumerate() {
        let display_name = CliMessages::resolve_app_name(&app.bundle_id, &app.name, lang);
        println!(
            "[{:>2}] {:<7} {:<18} {:<16} {}",
            idx + 1,
            app.pid,
            display_name,
            matched.tier_label,
            matched.matched_rule
        );
    }
    println!("------------------------------------------------------------");
    println!("{}", CliMessages::protected_view_guide(lang));
    print!("{}", CliMessages::interactive_prompt(lang));
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if !parts.is_empty() && (parts[0].eq_ignore_ascii_case("u") || parts[0].eq_ignore_ascii_case("rm")) {
            let indices = parse_indices(&parts[1..], protected_list.len());
            if indices.is_empty() {
                println!("{}", CliMessages::interactive_invalid_indices(lang));
            } else {
                for idx in indices {
                    let (app, _) = &protected_list[idx];
                    let ident = if !app.bundle_id.is_empty() {
                        &app.bundle_id
                    } else {
                        &app.name
                    };
                    match WhitelistManager::remove_identifier_from_config(custom_config_path, ident) {
                        Ok((path, _)) => {
                            println!(
                                "{}",
                                CliMessages::whitelist_remove_success(
                                    &app.name,
                                    &path.display().to_string(),
                                    lang
                                )
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "{}",
                                CliMessages::whitelist_remove_failed(&app.name, &e.to_string(), lang)
                            );
                        }
                    }
                }
            }
            pause_prompt(lang);
        }
    }
}

fn parse_indices(tokens: &[&str], max_len: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    for token in tokens {
        // 支持逗号分隔，如 "1,2,3"
        for sub in token.split(',') {
            let s = sub.trim();
            if let Ok(num) = s.parse::<usize>() {
                if num >= 1 && num <= max_len {
                    let idx = num - 1;
                    if !indices.contains(&idx) {
                        indices.push(idx);
                    }
                }
            }
        }
    }
    indices
}

fn confirm_action(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim().to_lowercase();
        trimmed == "y" || trimmed == "yes"
    } else {
        false
    }
}

fn pause_prompt(lang: Language) {
    print!("{}", CliMessages::pause_prompt(lang));
    let _ = io::stdout().flush();
    let mut dummy = String::new();
    let _ = io::stdin().read_line(&mut dummy);
}

fn execute_purge(lang: Language) {
    println!("{}", CliMessages::purge_running(lang));
    match macos_task_cleaner_core::purge_system_cache() {
        Ok(()) => println!("{}", CliMessages::purge_success(lang)),
        Err(e) => eprintln!("{}: {}", CliMessages::purge_warn(lang), e),
    }
}

/// 交互式语言切换向导菜单，支持保存至用户配置文件
pub fn run_language_select_menu(
    custom_config_path: Option<&Path>,
    current_lang: Language,
) -> Option<Language> {
    println!();
    println!("============================================================");
    println!("{}", CliMessages::lang_menu_banner(current_lang));
    println!("============================================================");
    let detected = Language::detect_system();
    println!("[Current / 当前语言]    {:?} ({})", current_lang, current_lang.code());
    println!("[Terminal / 终端探测]   {:?} ({})", detected, detected.code());
    println!();
    println!("Available Languages / 可选语言:");
    println!("  [ 0] Auto (Terminal & System Auto-Detect / 自动探测)");
    println!("  [ 1] English (en)               [13] Nederlands (nl)");
    println!("  [ 2] 简体中文 (zh-Hans)          [14] Polski (pl)");
    println!("  [ 3] 繁體中文 (zh-Hant)          [15] Türkçe (tr)");
    println!("  [ 4] 日本語 (ja)                 [16] العربية (ar)");
    println!("  [ 5] 한국어 (ko)                 [17] ไทย (th)");
    println!("  [ 6] Français (fr)              [18] Tiếng Việt (vi)");
    println!("  [ 7] Deutsch (de)               [19] Bahasa Indonesia (id)");
    println!("  [ 8] Español (es)               [20] Svenska (sv)");
    println!("  [ 9] Português (pt)             [21] Dansk (da)");
    println!("  [10] Italiano (it)              [22] Norsk Bokmål (nb)");
    println!("  [11] Русский (ru)               [23] Suomi (fi)");
    println!("  [12] Čeština (cs)               [24] Українська (uk)");
    println!("------------------------------------------------------------");
    print!("{}", CliMessages::lang_menu_prompt(current_lang));
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }

    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("q") || trimmed.eq_ignore_ascii_case("quit") {
        return None;
    }

    let (chosen_lang, code_to_save, is_auto) = match trimmed {
        "0" | "auto" | "a" => (detected, "auto", true),
        "1" => (Language::En, "en", false),
        "2" => (Language::ZhHans, "zh-Hans", false),
        "3" => (Language::ZhHant, "zh-Hant", false),
        "4" => (Language::Ja, "ja", false),
        "5" => (Language::Ko, "ko", false),
        "6" => (Language::Fr, "fr", false),
        "7" => (Language::De, "de", false),
        "8" => (Language::Es, "es", false),
        "9" => (Language::Pt, "pt", false),
        "10" => (Language::It, "it", false),
        "11" => (Language::Ru, "ru", false),
        "12" => (Language::Cs, "cs", false),
        "13" => (Language::Nl, "nl", false),
        "14" => (Language::Pl, "pl", false),
        "15" => (Language::Tr, "tr", false),
        "16" => (Language::Ar, "ar", false),
        "17" => (Language::Th, "th", false),
        "18" => (Language::Vi, "vi", false),
        "19" => (Language::Id, "id", false),
        "20" => (Language::Sv, "sv", false),
        "21" => (Language::Da, "da", false),
        "22" => (Language::Nb, "nb", false),
        "23" => (Language::Fi, "fi", false),
        "24" => (Language::Uk, "uk", false),
        custom => {
            let parsed = Language::from_locale_str(custom);
            (parsed, parsed.code(), false)
        }
    };

    match WhitelistManager::set_language_in_config(custom_config_path, code_to_save) {
        Ok((saved_path, _)) => {
            let path_str = saved_path.display().to_string();
            if is_auto {
                println!(
                    "{}",
                    CliMessages::lang_auto_saved_notice(&path_str, detected.code(), chosen_lang)
                );
            } else {
                let target_name = format!("{:?} ({})", chosen_lang, chosen_lang.code());
                println!(
                    "{}",
                    CliMessages::lang_saved_notice(&target_name, &path_str, chosen_lang)
                );
            }
            Some(chosen_lang)
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            None
        }
    }
}

