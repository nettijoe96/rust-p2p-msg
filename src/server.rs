// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

use tonic::{transport::Server, Request, Response, Status};

use p2p_msg::node_server::{Node, NodeServer};
use p2p_msg::{P2pReply, P2pMsg, Ack};

pub mod p2p_msg {
    tonic::include_proto!("p2pmsg");
}

#[derive(Debug, Default)]
pub struct MyNode {}

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

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let node = MyNode::default();

    Server::builder()
        .add_service(NodeServer::new(node))
        .serve(addr)
        .await?;

    Ok(())
}
