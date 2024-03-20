use ethnum::AsU256;
use tiny_blockchain::{
    is_epoch, pow, retarget, Block, BlockHeader, Chain, Hash, TinyBlockchainParams, Transaction,
};

fn mine_block(
    chain: &Chain,
    transactions: Vec<Transaction>,
    params: &TinyBlockchainParams,
) -> Block {
    let prev_block = chain.previous_block().expect("Previous block");
    let mut bits = prev_block.header.bits;

    if is_epoch(prev_block.height, chain, params) {
        // TODO: create a constant for 2016 blocks
        let epoch_start_block = chain
            .get_block(chain.len() - params.blocks_in_epoch)
            .unwrap();
        bits = retarget(epoch_start_block.header.bits, chain, params);
    }

    let proof = pow(bits);

    // TODO: add reward transaction
    // let reward_tx = Transaction::new(1);
    let block_header = BlockHeader {
        version: 1,
        timestamp: proof.timestamp,
        prev: prev_block.header_hash.clone(),
        merkle_root: 0.as_u256(),
        bits,
        nonce: proof.nonce,
    };

    Block::new(
        chain.len() + 1,
        block_header.hash(),
        block_header,
        transactions,
    )
}

// fn main() {
//     // init mempool
//     let mempool = Vec::<Transaction>::new();
//     // connect to node discovery or direclty to a node

//     // get actual blockchain

//     // listen new transaction

//     // take transactions from mempool

//     // mine a new block

//     // let new_bits = blockchain.retarget(0x1d00ffff);
//     // let mined_block = blockchain.mine_block(vec![]);

//     // blockchain.add_block(mined_block);

//     println!("Miner node");
// }

mod network;

use async_std::task::spawn;
use clap::Parser;

use futures::prelude::*;
use futures::StreamExt;
use libp2p::{core::Multiaddr, multiaddr::Protocol};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let opt = Opt::parse();

    let (mut network_client, mut network_events, network_event_loop) =
        network::new(opt.secret_key_seed).await?;

    // Spawn the network task for it to run in the background.
    spawn(network_event_loop.run());

    // In case a listen address was provided use it, otherwise listen on any
    // address.
    match opt.listen_address {
        Some(addr) => network_client
            .start_listening(addr)
            .await
            .expect("Listening not to fail."),
        None => network_client
            .start_listening("/ip4/0.0.0.0/tcp/0".parse()?)
            .await
            .expect("Listening not to fail."),
    };

    // In case the user provided an address of a peer on the CLI, dial it.
    if let Some(addr) = opt.peer {
        let Some(Protocol::P2p(peer_id)) = addr.iter().last() else {
            return Err("Expect peer multiaddr to contain peer ID.".into());
        };
        network_client
            .dial(peer_id, addr)
            .await
            .expect("Dial to succeed");
    }

    match opt.argument {
        // Providing a file.
        CliArgument::Provide { path, name } => {
            // Advertise oneself as a provider of the file on the DHT.
            network_client.start_providing(name.clone()).await;

            loop {
                match network_events.next().await {
                    // Reply with the content of the file on incoming requests.
                    Some(network::Event::InboundRequest { request, channel }) => {
                        if request == name {
                            network_client
                                .respond_file(std::fs::read(&path)?, channel)
                                .await;
                        }
                    }
                    e => todo!("{:?}", e),
                }
            }
        }
        // Locating and getting a file.
        CliArgument::Get { name } => {
            // Locate all nodes providing the file.
            let providers = network_client.get_providers(name.clone()).await;
            if providers.is_empty() {
                return Err(format!("Could not find provider for file {name}.").into());
            }

            // Request the content of the file from each node.
            let requests = providers.into_iter().map(|p| {
                let mut network_client = network_client.clone();
                let name = name.clone();
                async move { network_client.request_file(p, name).await }.boxed()
            });

            // Await the requests, ignore the remaining once a single one succeeds.
            let file_content = futures::future::select_ok(requests)
                .await
                .map_err(|_| "None of the providers returned file.")?
                .0;

            std::io::stdout().write_all(&file_content)?;
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(name = "libp2p file sharing example")]
struct Opt {
    /// Fixed value to generate deterministic peer ID.
    #[clap(long)]
    secret_key_seed: Option<u8>,

    #[clap(long)]
    peer: Option<Multiaddr>,

    #[clap(long)]
    listen_address: Option<Multiaddr>,

    #[clap(subcommand)]
    argument: CliArgument,
}

#[derive(Debug, Parser)]
enum CliArgument {
    Provide {
        #[clap(long)]
        path: PathBuf,
        #[clap(long)]
        name: String,
    },
    Get {
        #[clap(long)]
        name: String,
    },
}
