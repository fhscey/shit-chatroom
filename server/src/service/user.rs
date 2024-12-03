extern crate chatroom_net;

use self::chatroom_net::protocol::{MessageClient, UserList};
use async_std::net::TcpStream;
use async_std::sync::Arc;
use std::collections::HashMap;
use std::{error::Error, net::SocketAddr};

pub struct User {
    pub account: String,
    passwd: String,
    pub state: bool,
    pub address: SocketAddr,
    pub stream: Arc<TcpStream>,
}
pub async fn update_user_reg(
    users: &mut HashMap<String, User>,
    message: MessageClient,
    mut stream: Arc<TcpStream>,
) -> Result<u8, Box<dyn Error + Send + Sync>> {
    match users.get_mut(&message.s_account) {
        Some(user) => {
            Ok(2) //existed
        }
        None => {
            users.insert(
                message.s_account.clone(),
                User {
                    account: message.s_account.clone(),
                    passwd: message.passwd,
                    state: false,
                    address: message.address,
                    stream: stream,
                },
            );
            println!("user: {} registered!", message.s_account);
            Ok(1)
        }
    }
}
pub async fn update_user_login(
    users: &mut HashMap<String, User>,
    message: MessageClient,
) -> Result<u8, Box<dyn Error + Send + Sync>> {
    match users.get_mut(&message.s_account) {
        Some(user) => {
            if message.passwd == user.passwd {
                user.state = true;
                user.address = message.address;
                println!("user: {} login!", user.account);
                Ok(1)
            } else {
                Ok(2)
                //wrong passwd
            }
        }
        None => {
            Ok(3)
            //no such user
        }
    }
}
pub async fn update_user_logout(
    users: &mut HashMap<String, User>,
    message: MessageClient,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match users.get_mut(&message.s_account) {
        Some(user) => {
            user.state = false;
            println!("user: {} logout!", user.account);
            Ok(())
        }
        None => Err(format!(
            "User {} logout but is not registered on the server",
            message.s_account
        )
        .into()),
    }
}
pub fn get_user_list(users: &mut HashMap<String, User>) -> Vec<UserList> {
    users
        .into_iter()
        .map(|(account, user)| UserList {
            account: account.to_string(),
            state: user.state,
            is_selected: false,
        })
        .collect()
}
