// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

use tonic::{transport::Server, Request, Response, Status};

pub mod models;
use models::models::p2p_msg::node_server::{Node, NodeServer};
use models::models::p2p_msg::{P2pReply, P2pMsg, Ack};
use serde::{Deserialize};
use std::fs;
use std::env;
pub mod peer;


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
        let p2pmsg: P2pMsg = request.into_inner();
        println!("Got a request: {:?}", p2pmsg);

        let reply = P2pReply {
            ack: Ack::Success.into(),
        };

        for peer in &self.peers {
            let c = p2pmsg.clone();
            let addr = &peer.addr;
            let port = peer.port;
            //
            let r = peer::forward::forward(addr, port, c).await;
            if r.is_err() {
                // do not crash if node can't be reached
                println!("peer [{addr}]:{port} cannot be reached");
            }
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
