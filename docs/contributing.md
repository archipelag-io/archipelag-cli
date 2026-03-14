# Contributing

## Prerequisites

- Rust stable (1.75+)
- An Archipelag.io account with an API key (for manual testing)

## Setup

```bash
git clone https://github.com/archipelag-io/archipelagio-cli.git
cd archipelagio-cli
cargo build
```

## Development Workflow

```bash
# Build
cargo build

# Run
cargo run -- --help
cargo run -- account --api-key ak_xxx  # runs as archipelagio

# Check everything CI checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Format code
cargo fmt
```

## Adding a New Command

1. **Define the command** in `src/cli.rs`:

   Add a variant to the appropriate `enum` (`Command` for top-level, or a sub-enum like `JobsCommand`). Use clap's derive attributes for arguments.

   ```rust
   #[derive(Subcommand)]
   pub enum FooCommand {
       /// Short description shown in --help
       Bar {
           /// Argument description
           #[arg(short, long)]
           name: String,
       },
   }
   ```

2. **Add the API method** to `src/client.rs` (if it needs a new endpoint):

   Follow the existing pattern — make the request, call `check_error()`, deserialize, return the inner `data` field.

3. **Add response types** to `src/models.rs` (if the endpoint returns a new shape):

   Use `#[derive(Debug, Serialize, Deserialize)]` for types shown in `--format json` output. Use `#[derive(Debug, Deserialize)]` for internal-only types.

4. **Add a formatter** to `src/output.rs`:

   Write a `print_foo()` function that handles both `OutputFormat::Text` and `OutputFormat::Json`.

5. **Wire it up** in `src/commands.rs`:

   Add a `run_foo()` function and call it from the main `run()` dispatch.

## Adding a New API Endpoint

The pattern is always the same:

```rust
// In client.rs
pub async fn do_thing(&self, id: &str) -> Result<Thing> {
    let resp = self.http.get(self.url(&format!("/api/v1/things/{id}"))).send().await?;
    let resp = self.check_error(resp).await?;
    let body: ThingResponse = resp.json().await?;
    Ok(body.data)
}
```

For mutations, use `.post()` / `.delete()` / `.put()` and pass `.json(&body)`.

## Streaming Endpoints

For SSE streaming endpoints, follow the `stream_job()` / `chat_stream()` pattern:

1. Make the request
2. Get `resp.bytes_stream()`
3. Wrap with `.eventsource()`
4. `filter_map` to parse events, skip empty/`[DONE]`
5. Return `Pin<Box<dyn Stream<Item = Result<T>> + Send>>`

The caller consumes with `while let Some(result) = stream.next().await`.

## Output Conventions

- **Text format**: Use `colored` for status colors. Align columns with fixed widths in `format!`. Print to stdout.
- **JSON format**: `serde_json::to_string_pretty()` to stdout. Must be valid JSON that can be piped to `jq`.
- **Status messages** (connecting, done, errors): Print to stderr with `eprintln!` so they don't pollute piped JSON output.
- **Streaming tokens**: Print to stdout with `print!` + `io::stdout().flush()` (no newline until stream ends).

## Conventions

- All user-visible errors go through `anyhow::bail!` or `.context()` — never `unwrap()` or `panic!` in command handlers.
- API key is never logged or printed in full. `auth status` shows only the first 8 characters.
- Use `"✓".green()` for success, `"✗".red()` for errors, `"⚠".yellow()` for warnings, `"→".cyan()` for progress.
- Config file operations are fallible — always use `Config::load().unwrap_or_default()` when the file might not exist.

## CI

CI runs on every push and PR to `main`:

- `cargo check --all-targets`
- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`

All four must pass. See `.github/workflows/ci.yml`.

## Releasing

Releases are automated via GitHub Actions. To release a new version:

1. Update `version` in `Cargo.toml`
2. Commit: `git commit -am "Bump version to X.Y.Z"`
3. Tag: `git tag vX.Y.Z`
4. Push: `git push origin main --tags`

The release workflow (`.github/workflows/release.yml`) will:

- Build binaries for 6 targets (linux amd64/arm64/musl, macOS amd64/arm64, Windows)
- Create a GitHub Release with all binaries and SHA256 checksums
- Build and push a multi-arch OCI container to `ghcr.io/archipelag-io/archipelagio-cli`

Tags are automatically versioned: `v0.1.0` → `ghcr.io/archipelag-io/archipelagio-cli:0.1.0`, `:0.1`, `:0`, `:latest`.
