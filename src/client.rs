// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

use std::env;
mod forward;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("must provide filename as first command line arg. File must be in directory 'nodes'");
    }
    let msg = &args[1];

    let r: () = forward::forward(&"::1".to_string(), 50051, msg.to_string()).await?;
    Ok(r)
}