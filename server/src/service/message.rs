use async_std::net::{SocketAddr, TcpStream};
use async_std::sync::{Arc, RwLock};
use chatroom_net::protocol::{MessageClient, MessageServer};
use std::collections::HashMap;
use std::error::Error;

use super::user::{get_user_list, update_user_login, update_user_logout, update_user_reg, User};

pub async fn handle_connect(
    stream: Arc<TcpStream>,
    users_rw: Arc<RwLock<HashMap<String, User>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let message_client = MessageClient::read(stream.clone()).await?;
    let address = message_client.address;

    let mut users_w = users_rw.write().await;
    users_w
        .iter_mut()
        .for_each(|(_, user)| match user.stream.peer_addr() {
            Ok(_) => {}
            Err(e) => {
                println!("Client: {} DISCONNECTED: {}", user.address, e);
                user.state = false;
            }
        });
    process_message(message_client, &mut users_w, stream, address).await?;
    Ok(())
}
pub async fn process_message_reg(
    message_client: MessageClient,
    mut message_server: MessageServer,
    users: &mut HashMap<String, User>,
    mut stream: Arc<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    message_server.reg_state = update_user_reg(users, message_client, stream.clone()).await?;
    Ok(message_server.write(stream).await?)
}
pub async fn process_message_login(
    message_client: MessageClient,
    mut message_server: MessageServer,
    users: &mut HashMap<String, User>,
    mut stream: Arc<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    message_server.msg_type = 1;
    message_server.login_state = update_user_login(users, message_client).await?;
    Ok(message_server.write(stream).await?)
}
pub async fn process_message_content(
    mut message_client: MessageClient,
    mut message_server: MessageServer,
    users: &mut HashMap<String, User>,
    mut stream: Arc<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    message_server.msg_type = 2;
    message_server.content_state.r_account = message_client.r_account.clone();
    message_server.content_state.s_account = message_client.s_account.clone();
    message_server.content_state.content = message_client.content.clone();
    match users.get_mut(&message_client.r_account) {
        Some(r) => {
            if r.state == true {
                if let Err(_) = message_server.write(r.stream.clone()).await {
                    //try send to r
                    message_server.content_state.state = 4; //r broken
                    r.state = false;
                    println!("Client: {} DISCONNECTED!", r.address);
                    message_server.msg_type = 3; //send back to s
                    message_server.write(stream.clone()).await?
                } else {
                    message_server.content_state.state = 1; //success
                    message_server.msg_type = 3; //send back to s
                    message_server.write(stream.clone()).await?
                }
            } else {
                message_server.content_state.state = 2; //r offline
                message_server.msg_type = 3; //send back to s
                message_server.write(stream.clone()).await?
            }
        }
        None => {
            message_server.content_state.state = 3; //no such r
            message_server.msg_type = 3; //send back to s
            message_server.write(stream.clone()).await?
        }
    }
    Ok(())
}
pub async fn process_message_logout(
    message_client: MessageClient,
    users: &mut HashMap<String, User>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    update_user_logout(users, message_client).await?;
    Ok(())
}
pub async fn process_message_user_list(
    mut message_server: MessageServer,
    users: &mut HashMap<String, User>,
    mut stream: Arc<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    message_server.msg_type = 4;
    message_server.user_list = get_user_list(users);
    message_server.write(stream.into()).await?;

    Ok(())
}
pub async fn process_receive_request(
    message_client: MessageClient,
    mut stream: Arc<TcpStream>,
    users: &mut HashMap<String, User>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    //tcpmap.insert(address, stream.clone());
    match users.get_mut(&message_client.s_account) {
        Some(user) => {
            user.stream = stream;
            user.address = message_client.address
        }
        None => {
            eprintln!("unknown user: {} !", message_client.s_account)
        }
    }
    Ok(())
}
pub async fn process_message(
    message_client: MessageClient,
    users: &mut HashMap<String, User>,
    mut stream: Arc<TcpStream>,
    address: SocketAddr,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let message_server = MessageServer::new(address).await;
    match message_client.msg_type {
        0 => {
            //Reg
            process_message_reg(message_client, message_server, users, stream).await?
        }
        1 => {
            //login
            process_message_login(message_client, message_server, users, stream).await?
        }
        2 => {
            //content
            process_message_content(message_client, message_server, users, stream).await?
        }
        3 => {
            //logout
            process_message_logout(message_client, users).await?
        }
        4 => {
            //user list
            process_message_user_list(message_server, users, stream).await?
        }
        5 => {
            process_receive_request(message_client, stream, users).await?;
        }
        err_type => {
            eprintln!("undefined message type {}!", err_type)
        }
    }
    Ok(())
}
