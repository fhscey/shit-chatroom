use crate::ui::login_view::{MessageContent, State, UPDATE_MESSAGE_CONTENT, UPDATE_RESULT};
use crate::ui::main_view::launch_main_window;
use chatroom_net::protocol::{MessageClient, MessageServer};
use druid::{EventCtx, Target};
use std::error::Error;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub fn send_reg_message(
    addr: SocketAddr,
    account: String,
    password: String,
) -> Result<TcpStream, Box<dyn Error>> {
    let mut msg = MessageClient {
        msg_type: 0,
        s_account: account,
        r_account: String::new(),
        passwd: password,
        content: String::new(),
        address: "0.0.0.0:0".parse().unwrap(),
        hash: String::new(),
    };
    Ok(msg.write_client(addr)?)
}

pub fn send_login_message(
    addr: SocketAddr,
    account: String,
    password: String,
) -> Result<TcpStream, Box<dyn Error>> {
    let mut msg = MessageClient {
        msg_type: 1,
        s_account: account,
        r_account: String::new(),
        passwd: password,
        content: String::new(),
        address: "0.0.0.0:0".parse().unwrap(),
        hash: String::new(),
    };
    Ok(msg.write_client(addr)?)
}

pub fn send_content_message(
    addr: SocketAddr,
    s_account: String,
    r_account: String,
    content: String,
) -> Result<TcpStream, Box<dyn Error>> {
    let mut msg = MessageClient {
        msg_type: 2,
        s_account: s_account,
        r_account: r_account,
        passwd: String::new(),
        content: content,
        address: "0.0.0.0:0".parse().unwrap(),
        hash: String::new(),
    };
    //Ok(msg.write_client(addr)?)
    Ok(msg.write_client(addr)?)
}

pub fn send_logout_message(addr: SocketAddr, account: String) -> Result<TcpStream, Box<dyn Error>> {
    let mut msg = MessageClient {
        msg_type: 3,
        s_account: account,
        r_account: String::new(),
        passwd: String::new(),
        content: String::new(),
        address: "0.0.0.0:0".parse().unwrap(),
        hash: String::new(),
    };
    Ok(msg.write_client(addr)?)
}

pub fn send_user_list_request(
    addr: SocketAddr,
    account: String,
) -> Result<TcpStream, Box<dyn Error>> {
    let mut msg = MessageClient {
        msg_type: 4,
        s_account: account,
        r_account: String::new(),
        passwd: String::new(),
        content: String::new(),
        address: "0.0.0.0:0".parse().unwrap(),
        hash: String::new(),
    };
    Ok(msg.write_client(addr)?)
}

pub fn send_start_receive_request(
    addr: SocketAddr,
    account: String,
) -> Result<TcpStream, Box<dyn Error>> {
    let mut msg = MessageClient {
        msg_type: 5,
        s_account: account,
        r_account: String::new(),
        passwd: String::new(),
        content: String::new(),
        address: "0.0.0.0:0".parse().unwrap(),
        hash: String::new(),
    };
    Ok(msg.write_client(addr)?)
}

pub fn process_reg(state: &mut State) -> Result<(), Box<dyn Error>> {
    let mut addr = state.server_address.parse().map_err(|_| {
        state.result = format!("无效的地址");
        Box::<dyn Error>::from("地址解析失败")
    })?;
    let mut stream =
        send_reg_message(addr, state.account.clone(), state.passwd.clone()).map_err(|e| {
            state.result = format!("发送注册请求失败: {}", e);
            Box::<dyn Error>::from("发送注册请求失败")
        })?;
    let mut response = MessageServer::read_client(&mut stream).map_err(|e| {
        state.result = format!("无法收到服务器响应: {}", e);
        Box::<dyn Error>::from("无法收到服务器响应")
    })?;
    match response.msg_type {
        0 => match response.reg_state {
            1 => {
                state.result = format!("注册成功");
            }
            2 => {
                state.result = format!("用户已存在");
            }
            _ => {
                state.result = format!("无效的注册结果");
            }
        },
        _ => {
            state.result = format!("响应代码无效");
        }
    }
    Ok(())
}

pub fn process_login(ctx: &mut EventCtx<'_, '_>, state: &mut State) -> Result<(), Box<dyn Error>> {
    let mut addr = state.server_address.parse().map_err(|_| {
        state.result = format!("无效的地址");
        Box::<dyn Error>::from("地址解析失败")
    })?;
    let mut stream = send_login_message(addr, state.account.clone(), state.passwd.clone())
        .map_err(|e| {
            state.result = format!("发送登录请求失败: {}", e);
            Box::<dyn Error>::from("发送登录请求失败")
        })?;
    let mut response = MessageServer::read_client(&mut stream).map_err(|e| {
        state.result = format!("无法收到服务器响应: {}", e);
        Box::<dyn Error>::from("无法收到服务器响应")
    })?;
    match response.msg_type {
        1 => match response.login_state {
            1 => {
                ctx.window().close();
                launch_main_window(ctx, state);
            }
            2 => {
                state.result = format!("密码错误");
            }
            3 => {
                state.result = format!("用户不存在");
            }
            _ => {
                state.result = format!("无效的登录结果");
            }
        },
        _ => {
            state.result = format!("响应代码无效");
        }
    }
    Ok(())
}

pub fn flush_user_list(state: &mut State) -> Result<(), Box<dyn Error>> {
    let mut addr = state.server_address.parse().map_err(|_| {
        state.result = format!("无效的地址");
        Box::<dyn Error>::from("地址解析失败")
    })?;
    let mut stream = send_user_list_request(addr, state.account.clone()).map_err(|e| {
        state.result = format!("发送用户列表请求失败: {}", e);
        Box::<dyn Error>::from("发送用户列表请求失败")
    })?;
    let mut response = MessageServer::read_client(&mut stream).map_err(|e| {
        state.result = format!("无法收到服务器响应: {}", e);
        Box::<dyn Error>::from("无法收到服务器响应")
    })?;
    match response.msg_type {
        4 => {
            state.users = Arc::new(response.user_list);
            state.result = format!("用户列表已刷新");
        }
        _ => {
            state.result = format!("响应代码无效");
        }
    }
    Ok(())
}

pub fn process_logout(ctx: &mut EventCtx<'_, '_>, state: &mut State) -> Result<(), Box<dyn Error>> {
    let mut addr = state.server_address.parse().map_err(|_| {
        state.result = format!("无效的地址");
        Box::<dyn Error>::from("地址解析失败")
    })?;
    let mut stream = send_logout_message(addr, state.account.clone()).map_err(|e| {
        state.result = format!("发送注销请求失败: {}", e);
        Box::<dyn Error>::from("发送注销请求失败")
    })?;
    ctx.window().close();
    Ok(())
}

pub fn process_message_group(
    ctx: &mut EventCtx<'_, '_>,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    let event_sink = ctx.get_external_handle();
    let mut addr = state.server_address.parse().map_err(|_| {
        state.result = format!("无效的地址");
        Box::<dyn Error>::from("地址解析失败")
    })?;
    for user in Arc::make_mut(&mut state.selected_users).iter_mut() {
        let mut stream = send_content_message(
            addr,
            state.account.to_string(),
            user.to_string(),
            state.content_to_send.to_string(),
        )
        .map_err(|e| {
            state.result = format!("{}：发送失败: {}", user.to_string(), e);
            Box::<dyn Error>::from("发送失败")
        })?;
        let mut response = MessageServer::read_client(&mut stream).map_err(|e| {
            state.result = format!("无法收到服务器响应: {}", e);
            Box::<dyn Error>::from("无法收到服务器响应")
        })?;

        match response.msg_type {
            3 => match response.content_state.state {
                1 => {
                    let content = MessageContent {
                        s_account: response.content_state.s_account.clone(),
                        r_content: response.content_state.r_account.clone(),
                        content: response.content_state.content.clone(),
                        state: format!("(已发送)"),
                    };
                    event_sink
                        .submit_command(UPDATE_MESSAGE_CONTENT, content, Target::Global)
                        .unwrap();
                }
                2 => {
                    event_sink
                        .submit_command(
                            UPDATE_RESULT,
                            format!("用户：{} 已离线", response.content_state.r_account),
                            Target::Global,
                        )
                        .unwrap();
                }
                4 => {
                    event_sink
                        .submit_command(
                            UPDATE_RESULT,
                            format!("用户：{} 已断开", response.content_state.r_account),
                            Target::Global,
                        )
                        .unwrap();
                }
                _ => {
                    event_sink
                        .submit_command(
                            UPDATE_RESULT,
                            format!("用户：{} 无效的状态", response.content_state.r_account),
                            Target::Global,
                        )
                        .unwrap();
                }
            },
            _ => {
                event_sink
                    .submit_command(UPDATE_RESULT, format!("无效的消息代码"), Target::Global)
                    .unwrap();
            }
        }
    }
    Ok(())
}

pub fn process_message_content(
    ctx: &mut EventCtx<'_, '_>,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    let event_sink = ctx.get_external_handle();
    let mut addr: SocketAddr = state.server_address.parse().map_err(|_| {
        state.result = format!("无效的地址");
        Box::<dyn Error>::from("地址解析失败")
    })?;
    let mut stream = send_start_receive_request(addr, state.account.clone()).map_err(|e| {
        state.result = format!("发送接受消息请求失败: {}", e);
        Box::<dyn Error>::from("发送接受消息请求失败")
    })?;
    thread::spawn(move || loop {
        match MessageServer::read_client(&mut stream) {
            Ok(message) => {
                println!("{}", stream.local_addr().unwrap());
                match message.msg_type {
                    2 => {
                        let content = MessageContent {
                            s_account: message.content_state.s_account.clone(),
                            r_content: message.content_state.r_account.clone(),
                            content: message.content_state.content.clone(),
                            state: String::new(),
                        };

                        event_sink
                            .submit_command(UPDATE_MESSAGE_CONTENT, content, Target::Global)
                            .unwrap();
                    }
                    _ => {
                        event_sink
                            .submit_command(
                                UPDATE_RESULT,
                                format!("无效的消息代码"),
                                Target::Global,
                            )
                            .unwrap();
                    }
                }
            }
            Err(_) => {}
        }
        thread::sleep(Duration::from_millis(100));
    });
    Ok(())
}
