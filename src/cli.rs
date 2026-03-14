use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// CLI for the Archipelag.io distributed compute network
#[derive(Parser)]
#[command(name = "archipelag", version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    /// API base URL
    #[arg(
        long,
        global = true,
        env = "ARCHIPELAG_API_URL",
        default_value = "https://api.archipelag.io"
    )]
    pub api_url: String,

    /// API key (overrides config file)
    #[arg(long, global = true, env = "ARCHIPELAG_API_KEY")]
    pub api_key: Option<String>,

    /// NATS server URL
    #[arg(
        long,
        global = true,
        env = "ARCHIPELAG_NATS_URL",
        default_value = "nats://sail.archipelag.io:4222"
    )]
    pub nats_url: String,

    /// Output format
    #[arg(long, global = true, default_value = "text")]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authenticate with Archipelag.io
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },

    /// Show account info and credit balance
    Account,

    /// Chat with an AI model (streaming)
    Chat {
        /// The prompt to send
        prompt: String,

        /// System prompt
        #[arg(short, long)]
        system: Option<String>,

        /// Maximum tokens to generate
        #[arg(short, long)]
        max_tokens: Option<u32>,

        /// Sampling temperature (0.0-2.0)
        #[arg(short, long)]
        temperature: Option<f32>,

        /// Workload slug to use
        #[arg(short, long, default_value = "llm-chat")]
        workload: String,

        /// Disable streaming (wait for full response)
        #[arg(long)]
        no_stream: bool,
    },

    /// Manage compute jobs
    Jobs {
        #[command(subcommand)]
        command: JobsCommand,
    },

    /// Browse available workloads
    Workloads {
        #[command(subcommand)]
        command: WorkloadsCommand,
    },

    /// View Islands (compute hosts) on the network
    Hosts {
        #[command(subcommand)]
        command: HostsCommand,
    },

    /// Manage API keys
    #[command(name = "api-keys")]
    ApiKeys {
        #[command(subcommand)]
        command: ApiKeysCommand,
    },

    /// View market rates and pricing
    Market {
        #[command(subcommand)]
        command: MarketCommand,
    },

    /// Subscribe to NATS subjects (advanced)
    Nats {
        #[command(subcommand)]
        command: NatsCommand,
    },

    /// Generate shell completions
    Completion {
        /// Shell to generate completions for
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Save your API key
    Login {
        /// API key (or enter interactively)
        #[arg(long)]
        key: Option<String>,
    },
    /// Show current auth status
    Status,
    /// Remove saved credentials
    Logout,
}

#[derive(Subcommand)]
pub enum JobsCommand {
    /// List your jobs
    List {
        /// Maximum number of jobs to show
        #[arg(short, long, default_value = "20")]
        limit: u32,

        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: u32,
    },
    /// Submit a new job
    Submit {
        /// Workload slug (e.g., "llm-chat", "sdxl")
        #[arg(short, long)]
        workload: String,

        /// Job input as JSON string
        #[arg(short, long)]
        input: String,

        /// Stream output in real-time
        #[arg(short, long)]
        stream: bool,
    },
    /// Get job details
    Get {
        /// Job ID (UUID)
        id: String,
    },
    /// Stream job output in real-time
    Stream {
        /// Job ID (UUID)
        id: String,
    },
    /// Cancel a running job
    Cancel {
        /// Job ID (UUID)
        id: String,
    },
}

#[derive(Subcommand)]
pub enum WorkloadsCommand {
    /// List available workloads
    List,
    /// Get workload details
    Get {
        /// Workload slug
        slug: String,
    },
}

#[derive(Subcommand)]
pub enum HostsCommand {
    /// List online Islands
    List,
    /// Get Island details
    Get {
        /// Host ID (UUID)
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ApiKeysCommand {
    /// List your API keys
    List,
    /// Create a new API key
    Create {
        /// Name for the API key
        name: String,
    },
    /// Delete an API key
    Delete {
        /// API key ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum MarketCommand {
    /// Show current market rates
    Rates {
        /// Filter by workload slug
        workload: Option<String>,
    },
    /// Show price history for a workload
    History {
        /// Workload slug
        slug: String,
    },
}

#[derive(Subcommand)]
pub enum NatsCommand {
    /// Subscribe to a NATS subject and print messages
    Subscribe {
        /// Subject to subscribe to (e.g., "host.*.heartbeat")
        subject: String,

        /// Maximum messages to receive (0 = unlimited)
        #[arg(short, long, default_value = "0")]
        max: u64,
    },
}
