mod block;
mod blockchain;
mod network;

use blockchain::Blockchain;
use network::start_peer;
use std::sync::{Arc, Mutex};
use tokio;
use clap::Parser;
use tokio::time::{self, Duration};

#[derive(Parser)]
#[command(name = "Blockchain Node")]
#[command(about = "A lightweight blockchain node client", long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    #[arg(long)]
    peer_port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let blockchain_clone = blockchain.clone();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        loop {
            interval.tick().await;

            let transactions = vec![]; // Placeholder for transactions
            blockchain_clone.lock().unwrap().add_block(transactions);

            println!("New block added. Current chain length: {}", blockchain_clone.lock().unwrap().blocks.len());
        }
    });

    start_peer(blockchain, cli.port, cli.peer_port).await;
}
