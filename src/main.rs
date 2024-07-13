use std::error::Error;

use tracing_subscriber::EnvFilter;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Identities in libp2p are handled via a public/private key pair,
    // Nodes identify each other via their PeerId which is derived from their public key
    let mut swarm = libp2p::SwarmBuilder::with_new_identity();

    Ok(())
}
