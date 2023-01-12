use keyvaluestore::key_value_store_client::KeyValueStoreClient;
use keyvaluestore::{GetRequest, SetRequest};
use tonic::Request;

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KeyValueStoreClient::connect("http://[::1]:10000").await?;

    let set_res = client
        .set_value(Request::new(SetRequest {
            key: "foo".to_string(),
            value: "bar".to_string(),
        }))
        .await?;

    println!("c: SET_RESPONSE: {:?}", set_res.into_inner());

    let response = client
        .get_value(Request::new(GetRequest {
            key: "foo".to_string(),
        }))
        .await?;

    println!("c: GET_RESPONSE: {:?}", response.into_inner());

    Ok(())
}
