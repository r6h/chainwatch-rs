# Chainwatch-RS Roadmap

Async Rust CLI for Ethereum monitoring, analysis, and data extraction.

---

## 1. Overview

`chainwatch-rs` is a production-oriented CLI tool that connects to Ethereum-compatible nodes (via HTTP and WebSocket), processes blockchain data in real time and historically, and applies analysis rules.

The project is designed to demonstrate:
- idiomatic Rust
- async/concurrent systems with Tokio
- real-world crypto infrastructure patterns
- data pipelines and fault tolerance

---

## 2. Core Features (MVP)

### Commands

- **`watch`** — Subscribe to live blockchain events via WebSocket.
- **`blocks`** — Fetch and process block ranges concurrently.
- **`tx`** — Fetch and decode a transaction.
- **`address`** — Analyze activity for an address.

### Example Usage

```bash
chainwatch watch --ws wss://eth-mainnet.g.alchemy.com/v2/API_KEY

chainwatch blocks --from 22000000 --to 22001000 --concurrency 32

chainwatch tx 0xabc123...

chainwatch address 0xabc123...
```

---

## 3. Architecture

### High-level

```
CLI
 ├── config
 ├── rpc
 │    ├── http client
 │    └── ws client
 ├── pipeline
 │    ├── fetchers
 │    ├── decoders
 │    └── analyzers
 ├── storage
 └── output
```

### Async Data Flow

```
[ RPC Fetch (HTTP / WS) ]
        ↓
   mpsc channel
        ↓
 [ Decoder Workers ]
        ↓
   mpsc channel
        ↓
 [ Analyzer Workers ]
        ↓
 [ Storage / Output ]
```

---

## 4. Project Structure

```
chainwatch-rs/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   └── commands.rs
│   ├── config/
│   │   └── mod.rs
│   ├── rpc/
│   │   ├── mod.rs
│   │   ├── http.rs
│   │   └── ws.rs
│   ├── pipeline/
│   │   ├── mod.rs
│   │   ├── fetch.rs
│   │   ├── decode.rs
│   │   └── analyze.rs
│   ├── types/
│   │   └── mod.rs
│   ├── storage/
│   │   └── mod.rs
│   └── utils/
│       └── mod.rs
└── abis/
    └── *.json
```

---

## 5. Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
alloy = "0.9"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
```

---

## 6. CLI Layer

Use clap derive macros.

Example:

```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Watch {
        #[arg(long)]
        ws: String,
    },
    Blocks {
        #[arg(long)]
        from: u64,
        #[arg(long)]
        to: u64,
        #[arg(long, default_value = "16")]
        concurrency: usize,
    },
    Tx {
        hash: String,
    },
}
```

---

## 7. RPC Layer

### HTTP (JSON-RPC)

Used for:
- block fetching
- transaction queries

### WebSocket

Used for:
- new blocks
- logs
- pending transactions

---

## 8. Async & Concurrency Patterns

### 8.1 Bounded Concurrency

Avoid overwhelming RPC providers.

```rust
let semaphore = Arc::new(Semaphore::new(concurrency));

for block in range {
    let permit = semaphore.clone().acquire_owned().await?;

    tokio::spawn(async move {
        let _permit = permit;

        // fetch block
        // process
    });
}
```

### 8.2 Channels (Pipeline Separation)

```rust
let (tx, mut rx) = mpsc::channel(1024);

// producer
tokio::spawn(async move {
    tx.send(item).await?;
});

// consumer
tokio::spawn(async move {
    while let Some(item) = rx.recv().await {
        process(item);
    }
});
```

### 8.3 Timeouts

```rust
tokio::time::timeout(
    Duration::from_secs(5),
    rpc_call()
).await;
```

### 8.4 Graceful Shutdown

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        println!("shutdown signal received");
    }
    _ = run_pipeline() => {}
}
```

---

## 9. Data Types

Define strong internal types:

```rust
pub struct BlockData {
    pub number: u64,
    pub hash: String,
    pub transactions: Vec<TransactionData>,
}

pub struct TransactionData {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub input: Vec<u8>,
}
```

Avoid loose JSON handling in core logic.

---

## 10. Decoding Layer

Responsibilities:

- decode calldata using ABI
- decode logs/events
- map raw bytes → structured types

Optional:

- fallback to 4-byte signature matching

---

## 11. Analyzer Layer

Add simple rules:

- large transfers
- repeated failed calls
- contract interactions
- unusual activity patterns

Example:

```
IF tx.value > threshold AND to is contract
→ flag as "large contract interaction"
```

---

## 12. Storage Layer

Start with SQLite via sqlx.

Store:

- blocks
- transactions
- events
- analysis results

Schema example:

```sql
CREATE TABLE blocks (
    number INTEGER PRIMARY KEY,
    hash TEXT
);

CREATE TABLE transactions (
    hash TEXT PRIMARY KEY,
    block_number INTEGER,
    from_addr TEXT,
    to_addr TEXT
);
```

---

## 13. Config System

`contracts.toml`:

```toml
[[contracts]]
name = "ERC20"
address = "0x..."
abi = "./abis/erc20.json"
```

Load at startup.

---

## 14. Logging

Use tracing:

```rust
tracing::info!("processing block {}", number);
tracing::error!("rpc failed: {:?}", err);
```

Enable with:

```bash
RUST_LOG=info chainwatch ...
```

---

## 15. Stretch Features

### 15.1 Mempool Monitoring

- subscribe to pending transactions
- decode swaps / approvals

### 15.2 Reorg Detection

Track parent hash:

```
if new_block.parent_hash != last_block.hash:
    → reorg detected
```

### 15.3 ABI Signature Resolution

Map selectors:

```
0xa9059cbb → transfer(address,uint256)
```

### 15.4 Export Modes

- JSON
- CSV

---

## 16. Development Roadmap

- **Week 1** — CLI + basic RPC; fetch blocks/tx
- **Week 2** — concurrency + pipeline; decoding basics
- **Week 3** — storage (SQLite); logging + error handling
- **Week 4** — mempool watcher; basic analysis rules

---

## 17. What to Emphasize (for employers)

- async pipeline design
- bounded concurrency
- real RPC interaction
- clean Rust architecture
- error handling and resilience
- performance considerations

---

## 18. Positioning

Describe the project as:

> "Async Rust Ethereum monitoring pipeline with WebSocket subscriptions, bounded concurrency, ABI decoding, and real-time analysis."

Not as a simple CLI tool.

---

## 19. Future Extensions

- multi-chain support (Arbitrum, Optimism)
- distributed workers
- integration with your knowledge graph (Desmodus)
- custom rule engine
- API server on top of the pipeline
