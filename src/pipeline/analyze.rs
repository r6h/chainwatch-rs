use anyhow::Result;
use tokio::sync::mpsc;

use crate::types::{AnalysisResult, DecodedBlock};

pub async fn run_analyzer(mut decoded_rx: mpsc::Receiver<DecodedBlock>) -> Result<()> {
    while let Some(block) = decoded_rx.recv().await {
        let result = AnalysisResult {
            block_number: block.number,
            summary: format!(
                "block {} with hash {} has {} fake transactions",
                block.number, block.hash, block.tx_count
            ),
        };

        println!(
            "analysis result: block={}, summary={}",
            result.block_number, result.summary
        );
    }

    Ok(())
}
