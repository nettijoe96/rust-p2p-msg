fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tonic_build::compile_protos("proto/p2p.proto")?;
    tonic_build::configure()
        .type_attribute("p2p.P2pMsg", "#[derive(Copy)]")
        .compile(&["proto/p2p/p2p.proto"], &["proto"])
        .unwrap();
    Ok(())
}