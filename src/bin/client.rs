use keyvaluestore::key_value_store_client::KeyValueStoreClient;
use keyvaluestore::{GetRequest, SetRequest};
use tonic::Request;

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KeyValueStoreClient::connect("http://[::1]:10000").await?;

    let _ = client
        .set_value(Request::new(SetRequest {
            key: "foo".to_string(),
            value: "bar".to_string(),
        }))
        .await?;

    let response = client
        .get_value(Request::new(GetRequest {
            key: "foo".to_string(),
        }))
        .await?;
    println!("get response: {:?}", response);

    Ok(())
}
