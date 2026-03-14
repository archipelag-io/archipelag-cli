use colored::Colorize;

use crate::cli::OutputFormat;
use crate::models::*;

pub fn print_account(account: &Account, format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(account).unwrap()),
        OutputFormat::Text => {
            println!("{}", "Account".bold());
            println!("  Email:   {}", account.email);
            println!("  ID:      {}", account.id);
            println!("  Credits: {}", format_credits(account.credits));
            if let Some(ref created) = account.created_at {
                println!("  Created: {}", created);
            }
        }
    }
}

pub fn print_jobs(jobs: &[Job], format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(jobs).unwrap()),
        OutputFormat::Text => {
            if jobs.is_empty() {
                println!("{}", "No jobs found.".dimmed());
                return;
            }
            println!(
                "{:<38} {:<12} {:<16} {:<10} {}",
                "ID".bold(),
                "STATUS".bold(),
                "WORKLOAD".bold(),
                "DURATION".bold(),
                "CREATED".bold()
            );
            for job in jobs {
                println!(
                    "{:<38} {:<12} {:<16} {:<10} {}",
                    job.id,
                    format_status(&job.status),
                    job.workload_slug.as_deref().unwrap_or("-"),
                    format_duration(job.duration_ms),
                    format_time(&job.created_at),
                );
            }
        }
    }
}

pub fn print_job(job: &Job, format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(job).unwrap()),
        OutputFormat::Text => {
            println!("{}", "Job".bold());
            println!("  ID:       {}", job.id);
            println!("  Status:   {}", format_status(&job.status));
            if let Some(ref slug) = job.workload_slug {
                println!("  Workload: {}", slug);
            }
            if let Some(ref input) = job.input {
                println!(
                    "  Input:    {}",
                    serde_json::to_string(input).unwrap_or_default()
                );
            }
            if let Some(ref output) = job.output {
                println!("  Output:   {}", output);
            }
            if let Some(ref error) = job.error {
                println!("  Error:    {}", error.red());
            }
            println!("  Duration: {}", format_duration(job.duration_ms));
            println!("  Created:  {}", format_time(&job.created_at));
            if let Some(ref started) = job.started_at {
                println!("  Started:  {}", started);
            }
            if let Some(ref completed) = job.completed_at {
                println!("  Completed: {}", completed);
            }
            if let Some(ref usage) = job.usage {
                print_usage(usage);
            }
        }
    }
}

pub fn print_workloads(workloads: &[Workload], format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(workloads).unwrap()),
        OutputFormat::Text => {
            if workloads.is_empty() {
                println!("{}", "No workloads available.".dimmed());
                return;
            }
            println!(
                "{:<20} {:<30} {:<12} {:<10} {}",
                "SLUG".bold(),
                "NAME".bold(),
                "RUNTIME".bold(),
                "PRICE".bold(),
                "ENABLED".bold()
            );
            for w in workloads {
                println!(
                    "{:<20} {:<30} {:<12} {:<10} {}",
                    w.slug,
                    truncate(&w.name, 28),
                    w.runtime_type.as_deref().unwrap_or("-"),
                    w.price_per_job
                        .map(|p| format!("{:.4}", p))
                        .unwrap_or_else(|| "-".to_string()),
                    if w.is_enabled.unwrap_or(false) {
                        "yes".green().to_string()
                    } else {
                        "no".red().to_string()
                    },
                );
            }
        }
    }
}

pub fn print_workload(workload: &Workload, format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(workload).unwrap()),
        OutputFormat::Text => {
            println!("{}", "Workload".bold());
            println!("  Name:      {}", workload.name);
            println!("  Slug:      {}", workload.slug);
            println!("  ID:        {}", workload.id);
            if let Some(ref desc) = workload.description {
                println!("  Desc:      {}", desc);
            }
            if let Some(ref rt) = workload.runtime_type {
                println!("  Runtime:   {}", rt);
            }
            if let Some(vram) = workload.required_vram_mb {
                println!("  VRAM:      {} MB", vram);
            }
            if let Some(ram) = workload.required_ram_mb {
                println!("  RAM:       {} MB", ram);
            }
            if let Some(price) = workload.price_per_job {
                println!("  Price:     {} credits/job", price);
            }
            println!(
                "  Enabled:   {}",
                if workload.is_enabled.unwrap_or(false) {
                    "yes".green()
                } else {
                    "no".red()
                }
            );
        }
    }
}

pub fn print_hosts(hosts: &[Host], format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(hosts).unwrap()),
        OutputFormat::Text => {
            if hosts.is_empty() {
                println!("{}", "No Islands online.".dimmed());
                return;
            }
            println!(
                "{:<38} {:<16} {:<10} {:<10} {}",
                "ID".bold(),
                "NAME".bold(),
                "STATUS".bold(),
                "REGION".bold(),
                "KARMA".bold()
            );
            for h in hosts {
                println!(
                    "{:<38} {:<16} {:<10} {:<10} {}",
                    h.id,
                    truncate(h.name.as_deref().unwrap_or("-"), 14),
                    format_status(h.status.as_deref().unwrap_or("-")),
                    h.region.as_deref().unwrap_or("-"),
                    h.karma_score
                        .map(|k| format!("{:.1}", k))
                        .unwrap_or_else(|| "-".to_string()),
                );
            }
        }
    }
}

pub fn print_host(host: &Host, format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(host).unwrap()),
        OutputFormat::Text => {
            println!("{}", "Island".bold());
            println!("  ID:        {}", host.id);
            if let Some(ref name) = host.name {
                println!("  Name:      {}", name);
            }
            if let Some(ref status) = host.status {
                println!("  Status:    {}", format_status(status));
            }
            if let Some(ref region) = host.region {
                println!("  Region:    {}", region);
            }
            if let Some(karma) = host.karma_score {
                println!("  Karma:     {:.1}", karma);
            }
            if let Some(ref hb) = host.last_heartbeat_at {
                println!("  Heartbeat: {}", hb);
            }
            if let Some(ref caps) = host.capabilities {
                println!(
                    "  Caps:      {}",
                    serde_json::to_string_pretty(caps).unwrap_or_default()
                );
            }
        }
    }
}

pub fn print_api_keys(keys: &[ApiKey], format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(keys).unwrap()),
        OutputFormat::Text => {
            if keys.is_empty() {
                println!("{}", "No API keys.".dimmed());
                return;
            }
            println!(
                "{:<38} {:<20} {:<12} {}",
                "ID".bold(),
                "NAME".bold(),
                "PREFIX".bold(),
                "CREATED".bold()
            );
            for k in keys {
                println!(
                    "{:<38} {:<20} {:<12} {}",
                    k.id,
                    truncate(&k.name, 18),
                    k.prefix.as_deref().unwrap_or("-"),
                    format_time(&k.created_at),
                );
            }
        }
    }
}

pub fn print_market_rates(rates: &[MarketRate], format: OutputFormat) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(rates).unwrap()),
        OutputFormat::Text => {
            if rates.is_empty() {
                println!("{}", "No market data available.".dimmed());
                return;
            }
            println!(
                "{:<20} {:<12} {:<12} {:<12} {}",
                "WORKLOAD".bold(),
                "AVG".bold(),
                "MIN".bold(),
                "MAX".bold(),
                "HOSTS".bold()
            );
            for r in rates {
                println!(
                    "{:<20} {:<12} {:<12} {:<12} {}",
                    r.workload_slug,
                    r.avg_price
                        .map(|p| format!("{:.4}", p))
                        .unwrap_or_else(|| "-".to_string()),
                    r.min_price
                        .map(|p| format!("{:.4}", p))
                        .unwrap_or_else(|| "-".to_string()),
                    r.max_price
                        .map(|p| format!("{:.4}", p))
                        .unwrap_or_else(|| "-".to_string()),
                    r.num_hosts.unwrap_or(0),
                );
            }
        }
    }
}

fn print_usage(usage: &Usage) {
    println!("  Usage:");
    if let Some(pt) = usage.prompt_tokens {
        println!("    Prompt tokens:     {}", pt);
    }
    if let Some(ct) = usage.completion_tokens {
        println!("    Completion tokens: {}", ct);
    }
    if let Some(tt) = usage.total_tokens {
        println!("    Total tokens:      {}", tt);
    }
    if let Some(cr) = usage.credits_used {
        println!("    Credits used:      {}", format_credits(cr));
    }
}

fn format_status(status: &str) -> String {
    match status {
        "completed" | "succeeded" | "online" | "approved" => status.green().to_string(),
        "failed" | "error" | "offline" | "suspended" => status.red().to_string(),
        "running" | "streaming" | "assigned" => status.cyan().to_string(),
        "pending" | "queued" | "submitted" => status.yellow().to_string(),
        "cancelled" | "timeout" => status.dimmed().to_string(),
        _ => status.to_string(),
    }
}

fn format_credits(credits: f64) -> String {
    format!("{:.2}", credits)
}

fn format_duration(ms: Option<u64>) -> String {
    match ms {
        Some(ms) if ms < 1000 => format!("{}ms", ms),
        Some(ms) if ms < 60_000 => format!("{:.1}s", ms as f64 / 1000.0),
        Some(ms) => format!("{:.1}m", ms as f64 / 60_000.0),
        None => "-".to_string(),
    }
}

fn format_time(t: &Option<String>) -> String {
    t.as_deref().unwrap_or("-").to_string()
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}
