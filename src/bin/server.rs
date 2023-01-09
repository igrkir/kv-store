use std::collections::HashMap;

use keyvaluestore::key_value_store_server::{KeyValueStore, KeyValueStoreServer};
use keyvaluestore::{GetRequest, GetResponse, SetRequest, SetResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

#[derive(Debug)]
pub struct KVStoreService {
    db: HashMap<String, String>,
}

#[tonic::async_trait]
impl KeyValueStore for KVStoreService {
    async fn get_value(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        println!("GetValue: {:?}", request);

        let key = request.into_inner().key;

        if self.db.contains_key(&key) {
            let val = self.db.get(&key).unwrap();
            return Ok(Response::new(GetResponse { value: val.clone() }));
        }

        Ok(Response::new(GetResponse::default()))
    }

    async fn set_value(
        &self,
        _request: Request<SetRequest>,
    ) -> Result<Response<SetResponse>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("Listening on: {}", addr);

    let mut values = HashMap::new();
    values.insert("one".to_string(), "1".to_string());
    values.insert("two".to_string(), "2".to_string());

    let store = KVStoreService { db: values };

    let service = KeyValueStoreServer::new(store);
    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
