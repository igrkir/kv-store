use keyvaluestore::key_value_store_client::KeyValueStoreClient;
use keyvaluestore::{GetRequest, GetResponse, SetRequest, SetResponse};
use tokio::sync::{mpsc, oneshot};
use tonic::{Request, Response};

pub mod keyvaluestore {
    tonic::include_proto!("keyvaluestore");
}

type ResponseResult<T> = Result<Response<T>, tonic::Status>;
type Responder<T> = oneshot::Sender<ResponseResult<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<GetResponse>,
    },
    Set {
        key: String,
        value: String,
        resp: Responder<SetResponse>,
    },
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // task to manage connection resource
    let mng = tokio::spawn(async move {
        let mut client = KeyValueStoreClient::connect("http://[::1]:10000")
            .await
            .unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let request = Request::new(GetRequest { key: key });
                    let res = client.get_value(request).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, value, resp } => {
                    let request = Request::new(SetRequest {
                        key: key,
                        value: value,
                    });
                    let res = client.set_value(request).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    // concurrent processing task
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let res = resp_rx.await.unwrap();

        match res {
            Ok(response) => println!("c1: GET_RESPONSE: {:?}", response.into_inner()),
            Err(e) => println!("ERROR: {:?}", e),
        }
    });

    // concurrent processing task
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".to_string(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await.unwrap();

        match res {
            Ok(response) => println!("c2: SET_RESPONSE: {:?}", response.into_inner()),
            Err(e) => println!("ERROR: {:?}", e),
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    mng.await.unwrap();

    Ok(())
}
