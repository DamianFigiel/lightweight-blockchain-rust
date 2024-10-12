# Lightweight Blockchain Node Client
A lightweight blockchain node client built in Rust. This client can participate in consensus, validate transactions, relay data between peers, and synchronize with other nodes on the network. 

## Features
- **Blockchain Basics**: Implements a simple blockchain with transaction validation and block production.
- **Peer-to-Peer Networking**: Nodes communicate using a peer-to-peer network to share and synchronize the blockchain.
- **Blockchain Synchronization**: Newly connected nodes synchronize with peers to ensure they have the latest blockchain state.
- **Rust**: Leveraging Rust's performance and memory safety features for high efficiency.

## Prerequisites
- **Rust**: Make sure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).

## Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/DamianFigiel/lightweight-blockchain-rust.git
    cd blockchain_node_client
2. Build the project:
    ```bash
    cargo build --release
## Usage
To start a node, specify the port and an optional peer port. The first node can be started without a peer, and subsequent nodes can connect to it.

### Start Node 1 (No Peer)

    cargo run -- --port 3000

This starts a blockchain node on port 3000.

### Start Node 2 (Connect to Node 1)

    cargo run -- --port 3001 --peer-port 3000

This starts a second node on port 3001 and connects it to the node on port 3000. Node 2 will synchronize its blockchain with Node 1 upon startup.

### CLI Options
- `--port <PORT>`: Specifies the port to bind the node to. Defaults to 3000.
- `--peer-port <PEER_PORT>`: Specifies an optional peer port for connecting to another node. If provided, the node will synchronize with the peer.

## Project Structure
    .
    ├── src
    │   ├── blockchain.rs      # Blockchain logic and Proof of Work consensus
    │   ├── network.rs         # Networking and peer-to-peer communication
    │   ├── block.rs           # Core block data structures (Block, Transaction)
    │   └── main.rs            # Entry point and CLI argument parsing
    └── Cargo.toml             # Project dependencies

## How It Works
1. **Blockchain**: Each node maintains a blockchain with transactions. Blocks are mined using a simple PoW algorithm, where the difficulty is based on finding a hash with a specific number of leading zeros.

2. **Networking**: Nodes communicate using TcpStream and TcpListener from the tokio asynchronous runtime. Each node can connect to peers and request their blockchain for synchronization.

3. **Synchronization**: When a new node connects to a peer, it requests the current blockchain. If the peer’s blockchain is longer and valid, the new node replaces its local blockchain with the synchronized one.

## Testing and Demo
1. **Run Node 1**: Start the first node and let it produce a few blocks.
2. **Run Node 2**: Start a second node with --peer-port to connect it to Node 1.
3. **Observe Synchronization**: Node 2 will synchronize with Node 1, adopting the longer blockchain.

## Future Enhancements
- **Transaction Pool**: Implement a pool for handling multiple pending transactions.
- **Enhanced Consensus Mechanism**: Upgrade from Proof of Work to other consensus algorithms, like Proof of Stake.
- **Improved Error Handling**: Add more robust error handling and reconnection logic for networking.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

