// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

pub mod peer;
use std::fmt::Debug;

use clap::Parser;
pub mod models;
pub mod crypto;
use serde::{Serialize, Deserialize};
use k256::ecdsa::recoverable::Signature;
use models::models::p2p_msg::{P2pMsg};
use crypto::sig;
use bincode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    addr: String,
    #[arg(short, long)]
    port: i32,
    #[arg(short, long)]
    msg: String,
    #[arg(short, long)]
    keyfile: String,
}

// calls recieve_msg gRPC function on some server running on ip:port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{:?}", args);

    let p2pmsg = P2pMsg {
        msg: args.msg.to_string(),
        sig: vec![],
        pubkey: vec![]
    };

    let r: () = peer::forward::forward(&args.addr, args.port, p2pmsg).await?;
    Ok(r)
}