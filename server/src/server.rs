use crate::service::message::handle_connect;
use crate::service::user::User;
use async_std::net::TcpListener;
use async_std::sync::{Arc, RwLock};
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::error::Error;

const LOCAL_HOST: &str = "127.0.0.1:12345";

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind(LOCAL_HOST)
        .await
        .expect("Could not bind socket");
    let users_rw: Arc<RwLock<HashMap<String, User>>> = Arc::new(RwLock::new(HashMap::new()));

    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| {
            let users_rw = Arc::clone(&users_rw);
            async move {
                let stream = Arc::new(stream.unwrap());
                println!("Client {}: CONNECTED", stream.peer_addr().unwrap());
                spawn(handle_connect(stream, users_rw));
            }
        })
        .await;

    Ok(())
}
