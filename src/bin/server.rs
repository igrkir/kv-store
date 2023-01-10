use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use keyvaluestore::key_value_store_server::{KeyValueStore, KeyValueStoreServer};
use keyvaluestore::{GetRequest, GetResponse, SetRequest, SetResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

type Db = Arc<Mutex<HashMap<String, String>>>;

#[derive(Debug)]
pub struct KVStoreService {
    store: Db,
}

#[tonic::async_trait]
impl KeyValueStore for KVStoreService {
    async fn get_value(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        println!("GetValue from: {:?}", request.remote_addr());

        let key = request.into_inner().key;

        let store = self.store.lock().unwrap();
        if let Some(val) = store.get(&key) {
            return Ok(Response::new(GetResponse { value: val.clone() }));
        }

        // Ok(Response::new(GetResponse::default()))
        Ok(Response::new(GetResponse { value: "(nil)".to_string() }))
    }

    async fn set_value(
        &self,
        request: Request<SetRequest>,
    ) -> Result<Response<SetResponse>, Status> {
        println!("SetValue from: {:?}", request.remote_addr());

        let req = request.into_inner();

        let mut store = self.store.lock().unwrap();
        store.insert(req.key, req.value);

        Ok(Response::new(SetResponse { successed: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("Listening on: {}", addr);

    let db = Arc::new(Mutex::new(HashMap::new()));

    let store = KVStoreService { store: db };

    let svc = KeyValueStoreServer::new(store);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
