#[derive(Debug)]
pub struct RawBlock {
    pub number: u64,
    pub hash: String,
    pub raw_payload: String,
}

#[derive(Debug)]
pub struct DecodedBlock {
    pub number: u64,
    pub hash: String,
    pub tx_count: usize,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub block_number: u64,
    pub summary: String,
}
