// from https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

use std::env;
mod forward;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("must provide filename as first command line arg. File must be in directory 'nodes'");
    }
    let addr = &args[1];
    let port = args[2].parse::<i32>()?;
    let msg = &args[3];

    let r: () = forward::forward(&addr.to_string(), port, msg.to_string()).await?;
    Ok(r)
}