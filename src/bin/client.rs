use keyvaluestore::key_value_store_client::KeyValueStoreClient;
use keyvaluestore::GetRequest;
use tonic::Request;

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KeyValueStoreClient::connect("http://[::1]:10000").await?;

    let response = client
        .get_value(Request::new(GetRequest { key: "two".to_string() }))
        .await?;
    println!("response: {:?}", response);

    Ok(())
}
