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
struct FakeBlock {
    number: u64,
    hash: String,
}

pub async fn blocks(from: u64, to: u64, concurrency: usize) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let (tx, mut rx) = mpsc::channel::<FakeBlock>(1024);

    let consumer = tokio::spawn(async move {
        while let Some(block) = rx.recv().await {
            println!("analyzing fake block: #{}, hash={}", block.number, block.hash);
        }
    });

    let mut handles = Vec::new();

    for block_number in from..=to {
        let tx = tx.clone();
        let semaphore = semaphore.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire_owned().await.unwrap();

            sleep(Duration::from_millis(300)).await;

            let block = FakeBlock {
                number: block_number,
                hash: format!("0xfakehash{block_number}"),
            };

            tx.send(block).await.unwrap();
        });

        handles.push(handle);

        drop(tx);

        for handle in handles {
            handle.await?;
        }

        consumer.await?;
pub async fn tx(hash: String) -> Result<()> {
    info!(hash = hash, "fetching fake transaction");

    sleep(Duration::from_millis(500)).await;

    println!("transaction: {hash}");
    println!("status: fake-success");
    println!("from: 0x1111111111111111111111111111111111111111");
    println!("to:   0x2222222222222222222222222222222222222222");

    Ok(())
}
