use crate::models::models::p2p_msg::node_client::NodeClient;
use crate::models::models::p2p_msg::P2pMsg;

pub async fn forward(addr: &String, port: i32, packet: P2pMsg) -> Result<(), Box<dyn std::error::Error>> {
    let ip_str = format!("[{addr}]:{port}");
    let url = format!("https://{ip_str}");
    println!("url: {url}");
    let mut client = NodeClient::connect(url).await?;

    let response = client.recieve_msg(packet).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}