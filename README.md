# archipelagio-cli

Command-line interface for the [Archipelag.io](https://archipelag.io) distributed compute network.

**[Full documentation](https://docs.archipelag.io/sdks/cli/)** · **[Quickstart](https://docs.archipelag.io/getting-started/quickstart/cli/)**

The binary is called `archipelagio` and can also be invoked as `aio` for short.

## Install

### From source

```bash
cargo install --git https://github.com/archipelag-io/archipelagio-cli
```

### From crates.io

```bash
cargo install archipelag
```

Both `archipelagio` and `aio` (symlink) are included in release tarballs.

## Quick Start

```bash
# Authenticate with your API key
aio auth login

# Chat with an AI model (streaming)
aio chat "Explain quantum computing in one paragraph"

# Check your account balance
aio account

# List available workloads
aio workloads list

# Submit a custom job
aio jobs submit --workload llm-chat --input '{"prompt": "Hello!"}' --stream

# View Islands on the network
aio hosts list

# Check market rates
aio market rates
```

All examples use `aio` for brevity. `archipelagio` works identically.

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
| `aio auth login` | Save API key |
| `aio auth status` | Show authentication status |
| `aio auth logout` | Remove saved credentials |
| `aio account` | Show account info and credits |
| `aio chat <prompt>` | Chat with an AI model (streaming) |
| `aio jobs list` | List your compute jobs |
| `aio jobs submit` | Submit a new job |
| `aio jobs get <id>` | Get job details |
| `aio jobs stream <id>` | Stream job output |
| `aio jobs cancel <id>` | Cancel a running job |
| `aio workloads list` | List available workloads |
| `aio workloads get <slug>` | Get workload details |
| `aio hosts list` | List online Islands |
| `aio hosts get <id>` | Get Island details |
| `aio api-keys list` | List API keys |
| `aio api-keys create <name>` | Create a new API key |
| `aio api-keys delete <id>` | Delete an API key |
| `aio market rates` | Show current market rates |
| `aio market history <slug>` | Price history for a workload |
| `aio sail subscribe <subject>` | Subscribe to NATS messages |
| `aio completion <shell>` | Generate shell completions |

## Output Formats

All commands support `--format json` for machine-readable output:

```bash
aio jobs list --format json
aio account --format json | jq '.credits'
```

## Chat Options

```bash
# With system prompt
aio chat "Write a haiku" --system "You are a poet"

# Control generation
aio chat "Hello" --max-tokens 100 --temperature 0.5

# Non-streaming (wait for complete response)
aio chat "Hello" --no-stream

# Use a specific workload
aio chat "Hello" --workload llm-chat
```

## Sail (NATS)

Subscribe to live messages on the Archipelag.io message fabric:

```bash
# Watch all heartbeats
aio sail subscribe "host.*.heartbeat"

# Watch job status updates for a specific host
aio sail subscribe "host.abc123.status"

# Limit to 10 messages
aio sail subscribe "host.*.heartbeat" --max 10
```

## Shell Completions

```bash
# Bash
aio completion bash >> ~/.bashrc

# Zsh
aio completion zsh >> ~/.zshrc

# Fish
aio completion fish > ~/.config/fish/completions/archipelagio.fish
```

## License

MIT
