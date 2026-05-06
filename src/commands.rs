use anyhow::Result;
use std::sync::Arc;
use tokio::{
    sync::Semaphore,
    sync::mpsc,
    time::{Duration, sleep},
};
use tracing::info;
pub async fn watch() -> Result<()> {
    info!("Starting fake watcher");

    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        println!("New fake block received: #{i}");
    }

    Ok(())
}

#[derive(Debug)]
struct RawBlock {
    number: u64,
    hash: String,
    raw_payload: String,
}

#[derive(Debug)]
struct DecodedBlock {
    number: u64,
    hash: String,
    tx_count: usize,
}

#[derive(Debug)]
struct AnalysisResult {
    block_number: u64,
    summary: String,
}

pub async fn blocks(from: u64, to: u64, concurrency: usize) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(concurrency));

    let (raw_tx, raw_rx) = mpsc::channel::<RawBlock>(1024);
    let (decoded_tx, decoded_rx) = mpsc::channel::<DecodedBlock>(1024);

    let decoder = tokio::spawn(async move {
        let mut raw_rx = raw_rx;
        let decoded_tx = decoded_tx;

        while let Some(raw_block) = raw_rx.recv().await {
            sleep(Duration::from_millis(100)).await;

            let decoded = DecodeBlock {
                number: raw_block.number,
                hash: raw_block.hash,
                tx_count: raw_block.raw_payload.len() % 20,
            };

            decoded_tx.send(decoded).await.unwrap();
        }
    });

    let analyzer = tokio::spawn(async move {
        let mut decoded_rx = decoded_rx;

        while let Some(block) = decoded_rx.recv().await {
            let result = AnalysisResult {
                block_number: block.number,
                summary: format!(
                    "block {} has {} fake transactions",
                    block.number,
                    block.tx_count
                ),
            };

            println!("analysis result: {:?}", result);
        }
    });

    let mut handles = Vec::new();

    for block_number in from..=to {
        let raw_tx = raw_tx.clone();
        let semaphore = semaphore.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire_owned().await.unwrap();

            sleep(Duration::from_millis(300)).await;

            let raw_block = RawBlock {
                number: block_number,
                hash: format!("0xfakehash{block_number}"),
                raw_payload: format!("fake-json-payload-for-block-{block_number}"),
            };

            raw_tx.send(block).await.unwrap();
        });

        handles.push(handle);
    }

    drop(raw_tx);

    for handle in handles {
        handle.await?;
    }

    decoder.await?;
    analyzer.await?;

    Ok(())
}

pub async fn tx(hash: String) -> Result<()> {
    info!(hash = hash, "fetching fake transaction");

    sleep(Duration::from_millis(500)).await;

    println!("transaction: {hash}");
    println!("status: fake-success");
    println!("from: 0x1111111111111111111111111111111111111111");
    println!("to:   0x2222222222222222222222222222222222222222");

    Ok(())
}
