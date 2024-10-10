use libp2p::{
    PeerId, Swarm, NetworkBehaviour,
    tcp::TokioTcpConfig,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::SwarmBuilder,
}

#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    mdns: Mdns,
}

impl NodeBehaviour {
    pub async fn new() -> Self {
        let mdns =  Mdns::new(MdnsConfig::default()).await.unwrap();
        NodeBehaviour { mdns }
    }
}

pub async fn run_node() {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let transport = TokioTcpConfig::new();
    let behaviour = NodeBehaviour::new().await;
    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id).build();
    println!("Node running with PeerId: {:?}", local_peer_id);
    
    loop {
        match swarm.next().await.unwrap() {
            MdnsEvent::Discovered(peers) => {
                for (peer, _) in peers {
                    println!("Discovered peer: {:?}", peer);
                }
            }
            Mdns::Expired(expired) => {
                for (peer, _) in peers {
                    println!("Expired peer: {:?}", peer);
                }
            }
        }
    }
}