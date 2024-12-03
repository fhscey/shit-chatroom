extern crate async_std;
extern crate hex;
extern crate serde;
extern crate serde_json;
extern crate std;
use self::serde::{Deserialize, Serialize};
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::Arc;

use druid::Data;
use hex::encode;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream as TcpStreamClient;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageClient {
    pub msg_type: u8, //0:reg,1:login,2:content,3.logout,4.user_list,5.start_receive
    pub s_account: String,
    pub r_account: String,
    pub passwd: String,
    pub content: String,
    pub address: SocketAddr,
    pub hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageServer {
    pub msg_type: u8,    //0:reg,1:login,2:content_to_r,3.content_to_s,4:user_list
    pub reg_state: u8,   //1:success,2:existed
    pub login_state: u8, //1:success,2.wrong passwd,3.no such user
    pub content_state: ContentState,
    pub user_list: Vec<UserList>,
    pub address: SocketAddr,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContentState {
    pub s_account: String,
    pub r_account: String,
    pub content: String,
    pub state: u8,
}
impl ContentState {
    pub fn new() -> Self {
        Self {
            s_account: String::new(),
            r_account: String::new(),
            content: String::new(),
            state: 0, //1.success,2.r offline,3.no such r?(impossible),4.r broken
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct UserList {
    pub account: String,
    pub state: bool,
    pub is_selected: bool,
}
impl UserList {
    pub fn new() -> Self {
        Self {
            account: String::new(),
            state: false,
            is_selected: false,
        }
    }
}
impl MessageClient {
    pub fn write_client(&mut self, addr: SocketAddr) -> Result<TcpStreamClient, Box<dyn Error>> {
        let mut stream = TcpStreamClient::connect(addr)?;
        self.hash = String::new();
        stream.set_read_timeout(Some(Duration::new(2, 0)))?;
        self.address = stream.peer_addr()?;
        let json = serde_json::to_string(self)?;
        let mut hasher = Sha256::new();
        hasher.update(json.clone());
        self.hash = encode(hasher.finalize());
        let json = serde_json::to_string(self)?;
        stream.write(json.as_bytes())?;
        stream.flush()?;
        Ok(stream)
    }
    pub async fn read(mut stream: Arc<TcpStream>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut reader = BufReader::new(&*stream);
        let mut buf = [0; 1024];
        reader.read(&mut buf).await?;
        let mut to_be_deserialized = serde_json::Deserializer::from_slice(&buf);
        let mut message = Self::deserialize(&mut to_be_deserialized)?;
        let code = message.hash;
        message.hash = String::new();
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&message).unwrap());
        match encode(hasher.finalize()) == code {
            true => Ok(message),
            false => Err(format!("Verify failed!").into()),
        }
    }
}
impl MessageServer {
    pub async fn new(address: SocketAddr) -> Self {
        Self {
            msg_type: 0,
            reg_state: 0,
            login_state: 0,
            content_state: ContentState::new(),
            user_list: Vec::new(),
            address: address,
            hash: String::new(),
        }
    }
    pub fn read_client(stream: &mut TcpStreamClient) -> Result<Self, Box<dyn Error>> {
        let mut to_be_deserialized = serde_json::Deserializer::from_reader(stream);
        let mut message = Self::deserialize(&mut to_be_deserialized)?;

        let code = message.hash;
        message.hash = String::new();
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&message).unwrap());
        let compute_hash = encode(hasher.finalize());
        match compute_hash == code {
            true => Ok(message),
            false => Err(format!(
                "Verify failed! send_hash:{}, compute_hash:{}",
                code, compute_hash
            )
            .into()),
        }
    }
    pub async fn write(
        &mut self,
        mut stream: Arc<TcpStream>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.address = stream.peer_addr()?;
        self.hash = String::new();
        let mut stream = &*stream;
        let json = serde_json::to_string(self)?;
        let mut hasher = Sha256::new();
        hasher.update(json.clone());
        self.hash = encode(hasher.finalize());
        let json = serde_json::to_string(self)?;

        if let Err(e) = stream.write(json.as_bytes()).await {
            print!("{}:{}", e, self.address);
        }
        stream.flush().await?;
        Ok(())
    }
}
