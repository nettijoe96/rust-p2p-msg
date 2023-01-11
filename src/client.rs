// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

use p2p_msg::node_client::NodeClient;
use p2p_msg::P2pMsg;

pub mod p2p_msg {
    tonic::include_proto!("p2pmsg");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(P2pMsg {
        msg: "Tonic".into(),
    });

    let response = client.recieve_msg(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}