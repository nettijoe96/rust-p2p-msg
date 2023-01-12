// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

use tonic::{transport::Server, Request, Response, Status};

use p2p_msg::node_server::{Node, NodeServer};
use p2p_msg::{P2pReply, P2pMsg, Ack};
use serde::{Deserialize};
use std::fs;
use std::env;
mod forward;

pub mod p2p_msg {
    tonic::include_proto!("p2pmsg");
}

#[derive(Deserialize, Debug, Default)]
pub struct IP {
    addr: String,
    port: i32,
}

#[derive(Deserialize, Debug, Default)]
pub struct MyNode {
    id: i32,
    ip: IP,
    peers: Vec<IP>
}

#[tonic::async_trait]
impl Node for MyNode {
    async fn recieve_msg(
        &self,
        request: Request<P2pMsg>,
    ) -> Result<Response<P2pReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = p2p_msg::P2pReply {
            ack: Ack::Success.into(),
        };

        let msg = &request.into_inner().msg;
        for peer in &self.peers {
            let addr = &peer.addr;
            let port = peer.port;
            let r = forward::forward(addr, port, msg.to_string()).await;
        }

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("must provide filename as first command line arg. File must be in directory 'nodes'");
    }

    let filename = &args[1];
    let path = format!("nodes/{filename}");
    let contents = fs::read_to_string(path).expect("Couldn't find or load that file.");
    let n: MyNode = serde_json::from_str(&contents)?;

    let addr = &n.ip.addr;
    let port = n.ip.port;
    let ip_str = format!("[{addr}]:{port}").parse()?;

    Server::builder()
        .add_service(NodeServer::new(n))
        .serve(ip_str)
        .await?;

    Ok(())
}
