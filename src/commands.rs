use std::io::{self, Write};

use anyhow::{Context, Result};
use colored::Colorize;
use futures::StreamExt;

use crate::cli::*;
use crate::client::ApiClient;
use crate::config::{self, Config};
use crate::models::*;
use crate::output;

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Auth { command } => run_auth(command).await,
        Command::Completion { shell } => {
            let mut cmd = <Cli as clap::CommandFactory>::command();
            clap_complete::generate(shell, &mut cmd, "archipelag", &mut io::stdout());
            Ok(())
        }
        _ => {
            // All other commands need an API client
            let api_key = config::resolve_api_key(&cli.api_key)?;
            let client = ApiClient::new(&cli.api_url, &api_key)?;
            let format = cli.format;

            match cli.command {
                Command::Account => run_account(&client, format).await,
                Command::Chat {
                    prompt,
                    system,
                    max_tokens,
                    temperature,
                    workload,
                    no_stream,
                } => {
                    run_chat(&client, &prompt, system, max_tokens, temperature, &workload, no_stream)
                        .await
                }
                Command::Jobs { command } => run_jobs(&client, command, format).await,
                Command::Workloads { command } => run_workloads(&client, command, format).await,
                Command::Hosts { command } => run_hosts(&client, command, format).await,
                Command::ApiKeys { command } => run_api_keys(&client, command, format).await,
                Command::Market { command } => run_market(&client, command, format).await,
                Command::Nats { command } => run_nats(command, &cli.nats_url).await,
                Command::Auth { .. } | Command::Completion { .. } => unreachable!(),
            }
        }
    }
}

// --- Auth ---

async fn run_auth(command: AuthCommand) -> Result<()> {
    match command {
        AuthCommand::Login { key } => {
            let api_key = match key {
                Some(k) => k,
                None => {
                    eprint!("Enter your API key: ");
                    io::stderr().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    input.trim().to_string()
                }
            };

            if !api_key.starts_with("ak_") {
                anyhow::bail!("API key should start with 'ak_'. Got: {}...", &api_key[..api_key.len().min(6)]);
            }

            // Verify the key works
            let client = ApiClient::new("https://api.archipelag.io", &api_key)?;
            let account = client.get_account().await
                .context("Failed to verify API key. Is it valid?")?;

            let mut config = Config::load().unwrap_or_default();
            config.api_key = Some(api_key);
            config.save()?;

            println!("{} Logged in as {} ({})", "✓".green(), account.email, format!("{:.2} credits", account.credits));
            println!("  Config saved to {}", Config::path()?.display());
            Ok(())
        }
        AuthCommand::Status => {
            let config = Config::load()?;
            match config.api_key {
                Some(ref key) => {
                    let prefix = &key[..key.len().min(8)];
                    println!("Authenticated: {}…", prefix);
                    println!("Config: {}", Config::path()?.display());

                    // Try to verify
                    match ApiClient::new("https://api.archipelag.io", key) {
                        Ok(client) => match client.get_account().await {
                            Ok(account) => {
                                println!("Account: {} ({:.2} credits)", account.email, account.credits);
                            }
                            Err(e) => {
                                println!("{} Key may be invalid: {e}", "⚠".yellow());
                            }
                        },
                        Err(e) => println!("{} {e}", "⚠".yellow()),
                    }
                }
                None => {
                    println!("Not authenticated. Run `archipelag auth login` to get started.");
                }
            }
            Ok(())
        }
        AuthCommand::Logout => {
            let mut config = Config::load().unwrap_or_default();
            config.api_key = None;
            config.save()?;
            println!("{} Logged out. API key removed from config.", "✓".green());
            Ok(())
        }
    }
}

// --- Account ---

async fn run_account(client: &ApiClient, format: OutputFormat) -> Result<()> {
    let account = client.get_account().await?;
    output::print_account(&account, format);
    Ok(())
}

// --- Chat ---

async fn run_chat(
    client: &ApiClient,
    prompt: &str,
    system: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    _workload: &str,
    no_stream: bool,
) -> Result<()> {
    let mut messages = Vec::new();
    if let Some(sys) = system {
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: sys,
        });
    }
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: prompt.to_string(),
    });

    let request = ChatRequest {
        model: "default".to_string(),
        messages,
        stream: !no_stream,
        max_tokens,
        temperature,
    };

    if no_stream {
        let response = client.chat(&request).await?;
        if let Some(choice) = response.choices.first() {
            if let Some(ref msg) = choice.message {
                if let Some(ref content) = msg.content {
                    println!("{content}");
                }
            }
        }
        if let Some(ref usage) = response.usage {
            eprintln!(
                "\n{}",
                format!(
                    "[{} tokens]",
                    usage.total_tokens.unwrap_or(0)
                )
                .dimmed()
            );
        }
    } else {
        let mut stream = client.chat_stream(&request).await?;
        let mut total_tokens = 0u64;

        while let Some(result) = stream.next().await {
            match result {
                Ok(chunk) => {
                    for choice in &chunk.choices {
                        if let Some(ref delta) = choice.delta {
                            if let Some(ref content) = delta.content {
                                print!("{content}");
                                io::stdout().flush()?;
                            }
                        }
                    }
                    if let Some(ref usage) = chunk.usage {
                        total_tokens = usage.total_tokens.unwrap_or(total_tokens);
                    }
                }
                Err(e) => {
                    eprintln!("\n{} Stream error: {e}", "✗".red());
                    break;
                }
            }
        }

        println!(); // Final newline
        if total_tokens > 0 {
            eprintln!("{}", format!("[{total_tokens} tokens]").dimmed());
        }
    }

    Ok(())
}

// --- Jobs ---

async fn run_jobs(client: &ApiClient, command: JobsCommand, format: OutputFormat) -> Result<()> {
    match command {
        JobsCommand::List { limit, offset } => {
            let jobs = client.list_jobs(limit, offset).await?;
            output::print_jobs(&jobs, format);
        }
        JobsCommand::Submit {
            workload,
            input,
            stream,
        } => {
            let input: serde_json::Value =
                serde_json::from_str(&input).context("Invalid JSON input")?;
            let job = client.submit_job(&workload, input).await?;
            println!("{} Job submitted: {}", "✓".green(), job.id);

            if stream {
                println!("{}", "Streaming output…".dimmed());
                let mut event_stream = client.stream_job(&job.id).await?;
                while let Some(result) = event_stream.next().await {
                    match result {
                        Ok(event) => {
                            if let Some(ref content) = event.content {
                                print!("{content}");
                                io::stdout().flush()?;
                            }
                            if let Some(ref error) = event.error {
                                eprintln!("\n{} {error}", "✗".red());
                            }
                        }
                        Err(e) => {
                            eprintln!("\n{} Stream error: {e}", "✗".red());
                            break;
                        }
                    }
                }
                println!();
            } else {
                output::print_job(&job, format);
            }
        }
        JobsCommand::Get { id } => {
            let job = client.get_job(&id).await?;
            output::print_job(&job, format);
        }
        JobsCommand::Stream { id } => {
            let mut stream = client.stream_job(&id).await?;
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => {
                        if let Some(ref content) = event.content {
                            print!("{content}");
                            io::stdout().flush()?;
                        }
                        if let Some(ref error) = event.error {
                            eprintln!("\n{} {error}", "✗".red());
                        }
                    }
                    Err(e) => {
                        eprintln!("\n{} Stream error: {e}", "✗".red());
                        break;
                    }
                }
            }
            println!();
        }
        JobsCommand::Cancel { id } => {
            client.cancel_job(&id).await?;
            println!("{} Job {} cancelled.", "✓".green(), id);
        }
    }
    Ok(())
}

// --- Workloads ---

async fn run_workloads(
    client: &ApiClient,
    command: WorkloadsCommand,
    format: OutputFormat,
) -> Result<()> {
    match command {
        WorkloadsCommand::List => {
            let workloads = client.list_workloads().await?;
            output::print_workloads(&workloads, format);
        }
        WorkloadsCommand::Get { slug } => {
            let workload = client.get_workload(&slug).await?;
            output::print_workload(&workload, format);
        }
    }
    Ok(())
}

// --- Hosts ---

async fn run_hosts(client: &ApiClient, command: HostsCommand, format: OutputFormat) -> Result<()> {
    match command {
        HostsCommand::List => {
            let hosts = client.list_hosts().await?;
            output::print_hosts(&hosts, format);
        }
        HostsCommand::Get { id } => {
            let host = client.get_host(&id).await?;
            output::print_host(&host, format);
        }
    }
    Ok(())
}

// --- API Keys ---

async fn run_api_keys(
    client: &ApiClient,
    command: ApiKeysCommand,
    format: OutputFormat,
) -> Result<()> {
    match command {
        ApiKeysCommand::List => {
            let keys = client.list_api_keys().await?;
            output::print_api_keys(&keys, format);
        }
        ApiKeysCommand::Create { name } => {
            let created = client.create_api_key(&name).await?;
            println!("{} API key created:", "✓".green());
            println!("  Name: {}", created.api_key.name);
            println!("  ID:   {}", created.api_key.id);
            println!();
            println!("  {}", "Secret key (save this — it won't be shown again):".yellow());
            println!("  {}", created.key.bold());
        }
        ApiKeysCommand::Delete { id } => {
            client.delete_api_key(&id).await?;
            println!("{} API key {} deleted.", "✓".green(), id);
        }
    }
    Ok(())
}

// --- Market ---

async fn run_market(
    client: &ApiClient,
    command: MarketCommand,
    format: OutputFormat,
) -> Result<()> {
    match command {
        MarketCommand::Rates { workload } => {
            let rates = client.get_market_rates(workload.as_deref()).await?;
            output::print_market_rates(&rates, format);
        }
        MarketCommand::History { slug } => {
            let history = client.get_market_history(&slug).await?;
            output::print_market_rates(&history, format);
        }
    }
    Ok(())
}

// --- NATS ---

async fn run_nats(command: NatsCommand, nats_url: &str) -> Result<()> {
    match command {
        NatsCommand::Subscribe { subject, max } => {
            eprintln!(
                "{} Connecting to {}…",
                "→".cyan(),
                nats_url
            );

            let client = async_nats::connect(nats_url)
                .await
                .with_context(|| format!("Failed to connect to NATS at {nats_url}"))?;

            eprintln!(
                "{} Subscribed to \"{}\"{}",
                "✓".green(),
                subject,
                if max > 0 {
                    format!(" (max {max} messages)")
                } else {
                    String::new()
                }
            );

            let mut subscriber = client
                .subscribe(subject.clone())
                .await
                .context("Failed to subscribe")?;

            let mut count = 0u64;
            while let Some(msg) = subscriber.next().await {
                count += 1;

                let subject = msg.subject.as_str();
                let payload = String::from_utf8_lossy(&msg.payload);

                // Try to pretty-print JSON, fall back to raw
                let display = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&payload)
                {
                    serde_json::to_string_pretty(&json).unwrap_or_else(|_| payload.to_string())
                } else {
                    payload.to_string()
                };

                println!(
                    "{} {} {}",
                    format!("[{count}]").dimmed(),
                    subject.cyan(),
                    display
                );

                if max > 0 && count >= max {
                    eprintln!("{} Received {max} messages, stopping.", "✓".green());
                    break;
                }
            }
        }
    }
    Ok(())
}
