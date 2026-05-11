use anyhow::Result;
use tokio::{
    sync::mpsc,
    time::{Duration, sleep},
};

use crate::types::{DecodedBlock, RawBlock};

pub async fn run_decoder(
    mut raw_rx: mpsc::Receiver<RawBlock>,
    decoded_tx: mpsc::Sender<DecodedBlock>,
) -> Result<()> {
    while let Some(raw_block) = raw_rx.recv().await {
        sleep(Duration::from_millis(100)).await;

        let decoded = DecodedBlock {
            number: raw_block.number,
            hash: raw_block.hash,
            tx_count: raw_block.raw_payload.len() % 20,
        };

        decoded_tx.send(decoded).await?;
    }

    Ok(())
}
