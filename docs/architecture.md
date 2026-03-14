# Architecture

## Overview

The CLI is a single Rust binary that communicates with two backend services:

- **HTTP API** (`api.archipelag.io`) — REST endpoints for account, jobs, workloads, hosts, API keys, and market data
- **NATS** (`sail.archipelag.io:4222`) — Pub/sub message fabric for real-time events (heartbeats, job status, output streaming)

```
┌──────────────┐
│ archipelagio  │  (binary, alias: aio)
│    CLI        │
├──────┬───────┤
│ HTTP │ NATS  │
│Client│Client │
└──┬───┴───┬───┘
   │       │
   ▼       ▼
 api.    sail.
archipelag.io
```

## Source Layout

```
src/
├── main.rs       Entry point — parses CLI args, calls commands::run()
├── cli.rs        Command and argument definitions (clap derive)
├── client.rs     HTTP API client (reqwest) with SSE streaming
├── commands.rs   Command handler implementations
├── config.rs     Config file loading/saving (~/.config/archipelag/config.toml)
├── models.rs     API request/response types (serde)
└── output.rs     Terminal formatters (text tables, colored status, JSON)
```

## Module Responsibilities

### `cli.rs`

Defines the full command tree using clap's derive API. Every command, subcommand, argument, and flag lives here. Global options (`--api-url`, `--api-key`, `--nats-url`, `--format`) are propagated to all subcommands.

The `OutputFormat` enum (`Text` | `Json`) controls how all output is rendered.

### `client.rs`

`ApiClient` wraps `reqwest::Client` with:

- Default `Authorization: Bearer` header from the API key
- `User-Agent: archipelag-cli/{version}` header
- `check_error()` that maps HTTP status codes to human-readable `anyhow` errors (401 → auth, 402 → credits, 429 → rate limit, etc.)
- SSE streaming via `eventsource-stream` for `stream_job()` and `chat_stream()`, returned as `Pin<Box<dyn Stream>>` for ergonomic consumption

Every API method returns `Result<T>` using `anyhow`. The caller never sees raw HTTP.

### `commands.rs`

The `run()` function is the main dispatch. It splits into two branches:

1. **No-auth commands** — `auth`, `completion` (don't need an API client)
2. **Authenticated commands** — everything else (resolves API key, creates `ApiClient`, dispatches)

Each command group has its own `run_*` function. Streaming commands (`chat`, `jobs stream`, `jobs submit --stream`) consume the `Stream` with `StreamExt::next()` and flush stdout on each token.

The `sail` command uses `async-nats` directly — connect, subscribe, print messages.

### `config.rs`

Config file at `~/.config/archipelag/config.toml` (via the `dirs` crate). Three fields: `api_key`, `api_url`, `nats_url`.

Key resolution order: CLI flag → env var (handled by clap's `env` attribute) → config file.

### `models.rs`

Flat serde structs for every API response shape. All fields that might be absent are `Option<T>`. Response structs with a `data` wrapper (e.g., `JobsResponse { data: Vec<Job> }`) are unwrapped in `client.rs` so callers get the inner type directly.

Types used in `--format json` output derive both `Serialize` and `Deserialize`.

### `output.rs`

One `print_*` function per resource type. Each takes a format argument and branches:

- `Text` — colored, aligned table or key-value output using the `colored` crate
- `Json` — `serde_json::to_string_pretty()` to stdout

Status values are color-coded: green (completed/online), red (failed/offline), cyan (running), yellow (pending), dimmed (cancelled).

## Data Flow

### Authenticated command (e.g., `aio jobs list`)

```
main() → Cli::parse()
       → commands::run()
       → config::resolve_api_key()  // CLI flag > env > config file
       → ApiClient::new()           // builds reqwest client with auth header
       → client.list_jobs()         // GET /api/v1/jobs
       → check_error()              // maps HTTP errors to anyhow
       → output::print_jobs()       // text table or JSON
```

### Streaming command (e.g., `aio chat "hello"`)

```
main() → commands::run()
       → client.chat_stream()       // POST /api/v1/chat/completions (stream=true)
       → reqwest response stream
       → eventsource_stream         // parse SSE events
       → filter_map (skip empty/[DONE], parse JSON)
       → Pin<Box<dyn Stream>>       // returned to caller
       → while stream.next().await  // print delta.content, flush stdout
       → final newline + token count to stderr
```

### NATS command (e.g., `aio sail subscribe "host.*.heartbeat"`)

```
main() → commands::run()
       → async_nats::connect()      // TLS connection to sail.archipelag.io:4222
       → client.subscribe()         // subscribe to subject pattern
       → while subscriber.next()    // print [seq] subject payload (pretty JSON)
       → stop at --max count
```

## Error Handling

All errors flow through `anyhow::Result`. The HTTP client maps status codes to contextual messages:

| Status | Error message |
|--------|---------------|
| 401 | "Authentication failed: {msg}. Check your API key." |
| 402 | "Insufficient credits: {msg}" |
| 403 | "Access denied: {msg}. Your API key may lack the required scope." |
| 404 | "Not found: {msg}" |
| 422 | "Validation error: {msg}" |
| 429 | "Rate limited: {msg}. Please wait and retry." |
| other | "API error ({status}): {msg}" |

The error body is parsed as `ApiError { error, message }` when possible, falling back to the raw response body.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` + `clap_complete` | CLI parsing and shell completions |
| `tokio` | Async runtime |
| `reqwest` (rustls) | HTTP client (no OpenSSL dependency) |
| `async-nats` | NATS client |
| `serde` + `serde_json` | Serialization |
| `toml` | Config file parsing |
| `eventsource-stream` | SSE (Server-Sent Events) parsing |
| `futures` | `Stream` trait and `StreamExt` |
| `colored` | Terminal colors |
| `dirs` | XDG config directory resolution |
| `anyhow` | Error handling |
| `chrono` | Timestamp types |
