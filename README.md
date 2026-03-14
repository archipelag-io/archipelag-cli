# archipelag-cli

Command-line interface for the [Archipelag.io](https://archipelag.io) distributed compute network.

**[Full documentation](https://docs.archipelag.io/sdks/cli/)** · **[Quickstart](https://docs.archipelag.io/getting-started/quickstart/cli/)**

## Install

### From source

```bash
cargo install --git https://github.com/archipelag-io/archipelag-cli
```

### From crates.io

```bash
cargo install archipelag
```

## Quick Start

```bash
# Authenticate with your API key
archipelag auth login

# Chat with an AI model (streaming)
archipelag chat "Explain quantum computing in one paragraph"

# Check your account balance
archipelag account

# List available workloads
archipelag workloads list

# Submit a custom job
archipelag jobs submit --workload llm-chat --input '{"prompt": "Hello!"}' --stream

# View Islands on the network
archipelag hosts list

# Check market rates
archipelag market rates
```

## Configuration

Config is stored at `~/.config/archipelag/config.toml`:

```toml
api_key = "ak_your_key_here"
```

You can also use environment variables:

```bash
export ARCHIPELAG_API_KEY="ak_your_key_here"
export ARCHIPELAG_API_URL="https://api.archipelag.io"    # default
export ARCHIPELAG_NATS_URL="nats://sail.archipelag.io:4222"  # default
```

Precedence: CLI flag > environment variable > config file.

## Commands

| Command | Description |
|---------|-------------|
| `archipelag auth login` | Save API key |
| `archipelag auth status` | Show authentication status |
| `archipelag auth logout` | Remove saved credentials |
| `archipelag account` | Show account info and credits |
| `archipelag chat <prompt>` | Chat with an AI model (streaming) |
| `archipelag jobs list` | List your compute jobs |
| `archipelag jobs submit` | Submit a new job |
| `archipelag jobs get <id>` | Get job details |
| `archipelag jobs stream <id>` | Stream job output |
| `archipelag jobs cancel <id>` | Cancel a running job |
| `archipelag workloads list` | List available workloads |
| `archipelag workloads get <slug>` | Get workload details |
| `archipelag hosts list` | List online Islands |
| `archipelag hosts get <id>` | Get Island details |
| `archipelag api-keys list` | List API keys |
| `archipelag api-keys create <name>` | Create a new API key |
| `archipelag api-keys delete <id>` | Delete an API key |
| `archipelag market rates` | Show current market rates |
| `archipelag market history <slug>` | Price history for a workload |
| `archipelag nats subscribe <subject>` | Subscribe to NATS messages |
| `archipelag completion <shell>` | Generate shell completions |

## Output Formats

All commands support `--format json` for machine-readable output:

```bash
archipelag jobs list --format json
archipelag account --format json | jq '.credits'
```

## Chat Options

```bash
# With system prompt
archipelag chat "Write a haiku" --system "You are a poet"

# Control generation
archipelag chat "Hello" --max-tokens 100 --temperature 0.5

# Non-streaming (wait for complete response)
archipelag chat "Hello" --no-stream

# Use a specific workload
archipelag chat "Hello" --workload llm-chat
```

## NATS (Advanced)

Subscribe to live messages on the Archipelag.io message fabric:

```bash
# Watch all heartbeats
archipelag nats subscribe "host.*.heartbeat"

# Watch job status updates for a specific host
archipelag nats subscribe "host.abc123.status"

# Limit to 10 messages
archipelag nats subscribe "host.*.heartbeat" --max 10
```

## Shell Completions

```bash
# Bash
archipelag completion bash >> ~/.bashrc

# Zsh
archipelag completion zsh >> ~/.zshrc

# Fish
archipelag completion fish > ~/.config/fish/completions/archipelag.fish
```

## License

MIT
