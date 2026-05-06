use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "chainwatch")]
#[command(about = "Async blockchain monitoring CLI - learning version")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Watch,

    Blocks {
        #[arg(long)]
        from: u64,

        #[arg(long)]
        to: u64,

        #[arg(long, default_value_t = 4)]
        concurrency: usize,
    },

    Tx {
        hash: String,
    },
}
