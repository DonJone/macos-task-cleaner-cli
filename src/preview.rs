use serde::{Deserialize, Serialize};

use crate::i18n::CliMessages;
use macos_task_cleaner_core::{
    is_process_alive, AppTarget, Language, TerminationReport, WhitelistMatch,
};

/// 预检模式完整视图数据结构 (支持终端渲染与 JSON 格式化输出)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunSummary {
    pub scanned_total: usize,
    pub protected_count: usize,
    pub target_count: usize,
    pub scan_duration_ms: f64,
    pub config_source: String,
    pub protected_apps: Vec<ProtectedAppEntry>,
    pub targets: Vec<TargetAppEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedAppEntry {
    pub pid: i32,
    pub name: String,
    pub bundle_id: String,
    pub tier: String,
    #[serde(default)]
    pub tier_id: String,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetAppEntry {
    pub pid: i32,
    pub name: String,
    pub bundle_id: String,
    pub is_alive: bool,
}

/// 渲染并输出 Dry-Run 预检报表
pub fn render_dry_run_preview(
    scanned_apps: &[AppTarget],
    protected: &[(AppTarget, WhitelistMatch)],
    targets: &[AppTarget],
    scan_duration_ms: f64,
    config_path: Option<&std::path::Path>,
    as_json: bool,
    lang: Language,
) {
    let config_source = match config_path {
        Some(p) => CliMessages::config_source_loaded(&p.display().to_string(), lang),
        None => CliMessages::config_source_default(lang).to_string(),
    };

    let mut protected_entries = Vec::new();
    for (app, matched) in protected {
        protected_entries.push(ProtectedAppEntry {
            pid: app.pid,
            name: app.name.clone(),
            bundle_id: app.bundle_id.clone(),
            tier: matched.tier_label.clone(),
            tier_id: matched.tier_id.clone(),
            rule: matched.matched_rule.clone(),
        });
    }

    let mut target_entries = Vec::new();
    for app in targets {
        let alive = is_process_alive(app.pid);
        target_entries.push(TargetAppEntry {
            pid: app.pid,
            name: app.name.clone(),
            bundle_id: app.bundle_id.clone(),
            is_alive: alive,
        });
    }

    let summary = DryRunSummary {
        scanned_total: scanned_apps.len(),
        protected_count: protected.len(),
        target_count: targets.len(),
        scan_duration_ms,
        config_source: config_source.clone(),
        protected_apps: protected_entries,
        targets: target_entries,
    };

    if as_json {
        if let Ok(json_str) = serde_json::to_string_pretty(&summary) {
            println!("{}", json_str);
        }
        return;
    }

    // 纯文本友好表格排版 (无 Emoji)
    println!("============================================================");
    println!("{}", CliMessages::dry_run_title(lang));
    println!("============================================================");
    println!(
        "{}",
        CliMessages::dry_run_stats(summary.scanned_total, summary.protected_count, summary.target_count, lang)
    );
    match lang {
        Language::ZhHans => println!("[配置来源] {}", summary.config_source),
        Language::ZhHant => println!("[設定來源] {}", summary.config_source),
        Language::Ja => println!("[設定ソース] {}", summary.config_source),
        _ => println!("[Config Source] {}", summary.config_source),
    }
    println!();

    if !summary.protected_apps.is_empty() {
        let (c1, c2, c3, c4) = CliMessages::dry_run_protected_cols(lang);
        println!("------------------------------------------------------------");
        println!("{}", CliMessages::dry_run_protected_banner(lang));
        println!("------------------------------------------------------------");
        println!("{:<7} {:<18} {:<16} {}", c1, c2, c3, c4);
        println!("{:-<7} {:-<18} {:-<16} {:-<20}", "", "", "", "");
        for item in &summary.protected_apps {
            println!(
                "{:<7} {:<18} {:<16} {}",
                item.pid, item.name, item.tier, item.rule
            );
        }
        println!();
    }

    println!("------------------------------------------------------------");
    println!("{}", CliMessages::dry_run_targets_banner(lang));
    println!("------------------------------------------------------------");
    if summary.targets.is_empty() {
        println!("{}", CliMessages::dry_run_targets_empty(lang));
    } else {
        let (c1, c2, c3, c4) = CliMessages::dry_run_targets_cols(lang);
        println!("{:<7} {:<18} {:<30} {}", c1, c2, c3, c4);
        println!("{:-<7} {:-<18} {:-<30} {:-<14}", "", "", "", "");
        for target in &summary.targets {
            let status_str = if target.is_alive {
                CliMessages::status_alive(lang)
            } else {
                CliMessages::status_suspended(lang)
            };
            println!(
                "{:<7} {:<18} {:<30} {}",
                target.pid, target.name, target.bundle_id, status_str
            );
        }
    }
    println!("------------------------------------------------------------");
    println!("{}", CliMessages::dry_run_hint(lang));
    println!("{}", CliMessages::dry_run_duration(scan_duration_ms, lang));
    println!("============================================================");
}

/// 渲染清场执行报告
pub fn render_execution_report(report: &TerminationReport, as_json: bool, lang: Language) {
    if as_json {
        if let Ok(json_str) = serde_json::to_string_pretty(report) {
            println!("{}", json_str);
        }
        return;
    }

    println!();
    println!("============================================================");
    println!("{}", CliMessages::report_title(lang));
    println!("============================================================");
    println!("{}", CliMessages::report_total_targets(report.total_targets, lang));
    println!("{}", CliMessages::report_sigterm(report.terminated_sigterm, lang));
    println!("{}", CliMessages::report_sigkill(report.terminated_sigkill, lang));
    println!("{}", CliMessages::report_failed(report.failed, lang));
    println!("{}", CliMessages::report_duration(report.duration_ms, lang));
    println!("------------------------------------------------------------");

    if !report.records.is_empty() {
        let (c1, c2, c3, c4) = CliMessages::report_cols(lang);
        println!("{:<7} {:<20} {:<18} {}", c1, c2, c3, c4);
        println!("{:-<7} {:-<20} {:-<18} {:-<8}", "", "", "", "");
        for rec in &report.records {
            println!(
                "{:<7} {:<20} {:<18} {}",
                rec.app.pid,
                rec.app.name,
                rec.status,
                rec.exit_signal.as_deref().unwrap_or("-")
            );
        }
    }
    println!("============================================================");
}
