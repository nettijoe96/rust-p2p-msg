use p2p_msg::node_client::NodeClient;
use p2p_msg::P2pMsg;

pub mod p2p_msg {
    tonic::include_proto!("p2pmsg");
}

pub async fn forward(addr: &String, port: i32, msg: String) -> Result<(), Box<dyn std::error::Error>> {
    let ip_str = format!("[{addr}]:{port}");
    let url = format!("https://{ip_str}");
    println!("url: {url}");
    let mut client = NodeClient::connect(url).await?;

    let request = tonic::Request::new(P2pMsg {
        msg: msg.into(),
    });

    let response = client.recieve_msg(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}