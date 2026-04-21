use std::sync::Arc;

use anchor_client::{ Client, Cluster, Program, solana_client::nonblocking::rpc_client::RpcClient };
use anyhow::Result;
use solana_sdk::{ commitment_config::CommitmentConfig, signature::{ Keypair, read_keypair_file }, signer::Signer };
use std::env;

use crate::example;

pub fn instance() -> Result<(RpcClient, Client<Arc<Keypair>>, Program<Arc<Keypair>>)> {
    let wallet_path = env
        ::var("ANCHOR_WALLET")
        .unwrap_or_else(|_| { format!("{}/.config/solana/id.json", env::var("HOME").unwrap()) });

    let payer = read_keypair_file(&wallet_path).map_err(|e|
        anyhow::anyhow!("Keypair file read failed: {}", e)
    )?;

    let payer = Arc::new(payer);

    println!("Payer pubkey: {:?}",  payer.pubkey());

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed()
    );

    let provider = Client::new_with_options(
        Cluster::Devnet,
        payer.clone(),
        CommitmentConfig::confirmed()
    );

    let instances = provider.program(example::ID)?;

    Ok((connection, provider, instances))
}
