use crate::block::{Transaction, Block};
use crate::blockchain::Blockchain;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub async fn start_peer(blockchain: Arc<Mutex<Blockchain>>, port: u16, peer_port: Option<u16>) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    println!("Node listening on port: {}", port);

    if let Some(peer_port) = peer_port {
        let _ = connect_to_peer(peer_port, blockchain.clone()).await;
    }

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let blockchain = blockchain.clone();
        tokio::spawn(async move {
            handle_connection(socket, blockchain).await;
        });
    }
}

async fn connect_to_peer(peer_port: u16, blockchain: Arc<Mutex<Blockchain>>) {
    if let Ok(mut stream) = TcpStream::connect(format!("127.0.0.1:{}", peer_port)).await { // Hardcoded to localhost for now
        println!("Connected to peer on port: {}", peer_port);

        let request = json!({ "type": "sync_request" }).to_string();
        let _ = stream.write_all(request.as_bytes()).await;
        
        let mut buf = vec![0; 1024];
        let mut response = String::new();
        
        while let Ok(n) = stream.read(&mut buf).await {
            if n == 0 { break; }
            response.push_str(&String::from_utf8_lossy(&buf[..n]));

            if let Ok(msg) = serde_json::from_str::<Value>(&response) {
                if msg["type"] == "sync_response" {
                    let blocks: Vec<Block> = serde_json::from_str(&msg["data"].as_str().unwrap()).unwrap();
                    let mut local_chain = blockchain.lock().unwrap();
                    if blocks.len() > local_chain.blocks.len() && validate_chain(&blocks) {
                        println!("Replacing local blockchain with synchronized chain from peer.");
                        local_chain.blocks = blocks;
                    }
                }
                break; 
            }
        }
    }
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buf = vec![0; 1024];
    let mut request = String::new();

    while let Ok(n) = socket.read(&mut buf).await {
        if n == 0 {
            return;
        }

        request.push_str(&String::from_utf8_lossy(&buf[..n]));

        if let Ok(msg) = serde_json::from_str::<Value>(&request) {
            if msg["type"] == "sync_request" {
                let blockchain_data = {
                    let blockchain = blockchain.lock().unwrap();
                    serde_json::to_string(&blockchain.blocks).unwrap()
                };
                let response = json!({ "type": "sync_response", "data": blockchain_data }).to_string();
                let _ = socket.write_all(response.as_bytes()).await;
            } else if msg["type"] == "transaction" {
                let tx: Transaction = serde_json::from_value(msg["data"].clone()).unwrap();
                if blockchain.lock().unwrap().validate_transaction(&tx) {
                    println!("Transaction validated and added: {:?}", tx);
                }
            }
            break;
        }
    }
}

fn validate_chain(blocks: &[Block]) -> bool {
    for i in 1..blocks.len() {
        let prev_block = &blocks[i - 1];
        let current_block = &blocks[i];
        if current_block.previous_hash != prev_block.hash() {
            return false;
        }
    }
    true
}
