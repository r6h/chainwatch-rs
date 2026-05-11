use anyhow::Result;
use std::sync::Arc;
use tokio::{
    sync::{Semaphore, mpsc},
    time::{Duration, sleep},
};

use crate::types::RawBlock;

pub async fn fetch_fake_block(
    block_number: u64,
    raw_tx: mpsc::Sender<RawBlock>,
    semaphore: Arc<Semaphore>,
) -> Result<()> {
    let _permit = semaphore.acquire_owned().await?;

    let delay = 50 + (block_number % 5) * 100;
    sleep(Duration::from_millis(delay)).await;

    let raw_block = RawBlock {
        number: block_number,
        hash: format!("0xfakehash{block_number}"),
        raw_payload: format!("fake-json-payload-for-block-{block_number}"),
    };

    raw_tx.send(raw_block).await?;

    Ok(())
}
