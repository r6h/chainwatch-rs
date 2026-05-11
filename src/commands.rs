use anyhow::Result;
use std::sync::Arc;
use tokio::{
    sync::{Semaphore, mpsc},
    time::{Duration, sleep},
};
use tracing::info;

use crate::pipeline::{analyze::run_analyzer, decode::run_decoder, fetch::fetch_fake_block};
use crate::types::{DecodedBlock, RawBlock};

pub async fn watch() -> Result<()> {
    info!("Starting fake watcher");

    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        println!("New fake block received: #{i}");
    }

    Ok(())
}

pub async fn blocks(from: u64, to: u64, concurrency: usize) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(concurrency));

    let (raw_tx, raw_rx) = mpsc::channel::<RawBlock>(1024);
    let (decoded_tx, decoded_rx) = mpsc::channel::<DecodedBlock>(1024);

    let decoder = tokio::spawn(async move { run_decoder(raw_rx, decoded_tx).await });

    let analyzer = tokio::spawn(async move { run_analyzer(decoded_rx).await });

    let mut handles = Vec::new();

    for block_number in from..=to {
        let raw_tx = raw_tx.clone();
        let semaphore = semaphore.clone();

        let handle =
            tokio::spawn(async move { fetch_fake_block(block_number, raw_tx, semaphore).await });

        handles.push(handle);
    }

    drop(raw_tx);

    for handle in handles {
        handle.await??;
    }

    decoder.await??;
    analyzer.await??;

    Ok(())
}

pub async fn tx(hash: String) -> Result<()> {
    info!(hash = hash, "fetching fake transaction");

    sleep(Duration::from_millis(500)).await;

    println!("transaction: {hash}");
    println!("transaction: fake-success");
    println!("from: 0x1111111111111111111111111111111111111111");
    println!("to:   0x2222222222222222222222222222222222222222");

    Ok(())
}
