use clap::{Parser, Subcommand};
use keyvaluestore::key_value_store_client::KeyValueStoreClient;
use keyvaluestore::{GetRequest, SetRequest};
use tonic::Request;

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// get_value command
    Get {
        /// key arg
        key: String,
    },
    /// set_value command
    Set {
        /// key arg
        key: String,

        /// value arg
        value: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parsing arguments
    let cli = Cli::parse();

    let mut client = KeyValueStoreClient::connect("http://[::1]:10000").await?;

    match cli.command {
        Command::Get { key } => {
            let request = Request::new(GetRequest { key });
            let response = client.get_value(request).await?;
            println!("GET RESPONSE={:?}", response.into_inner().value);
        }
        Command::Set { key, value } => {
            let request = Request::new(SetRequest { key, value });
            match client.set_value(request).await {
                Ok(response) => println!("RES: {:?}", response.into_inner()),
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
    }

    Ok(())
}
