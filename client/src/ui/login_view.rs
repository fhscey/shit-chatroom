use chatroom_net::protocol::UserList;
use druid::{
    widget::{Align, Button, Flex, Label, TextBox},
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled, Lens, LocalizedString,
    Selector, Target, Widget, WidgetExt, WindowDesc,
};

use crate::handler::login::{handle_login_click, handle_reg_click};
use std::collections::VecDeque;
use std::sync::Arc;

pub const ADD_USER_TO_GROUP: Selector<String> = Selector::new("add_user_to_group");
pub const UPDATE_MESSAGE_CONTENT: Selector<MessageContent> =
    Selector::new("update_message_content");
pub const UPDATE_RESULT: Selector<String> = Selector::new("update_result");

pub struct MainDelegate;

impl AppDelegate<State> for MainDelegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        target: Target,
        cmd: &Command,
        data: &mut State,
        env: &Env,
    ) -> Handled {
        if let Some(account) = cmd.get(ADD_USER_TO_GROUP) {
            Arc::make_mut(&mut data.selected_users).push(account.to_string());
        }
        if let Some(message) = cmd.get(UPDATE_MESSAGE_CONTENT) {
            Arc::make_mut(&mut data.content).push_back(message.clone());
        }
        if let Some(results) = cmd.get(UPDATE_RESULT) {
            data.result = results.to_string();
        }

        Handled::No
    }
}

#[derive(Clone, Data, Lens)]
pub struct State {
    pub account: String,
    pub passwd: String,
    pub server_address: String,
    pub result: String,
    pub users: Arc<Vec<UserList>>,
    pub content: Arc<VecDeque<MessageContent>>,
    pub selected_users: Arc<Vec<String>>,
    pub content_to_send: String,
}

#[derive(Clone, Data, Lens)]
pub struct MessageContent {
    pub s_account: String,
    pub r_content: String,
    pub content: String,
    pub state: String,
}

pub fn launch_login_window() {
    let login_window = WindowDesc::new(build_login_window())
        .title(LocalizedString::new("chatroom-login"))
        .window_size((400.0, 400.0));

    let initial_state = State {
        account: String::new(),
        passwd: String::new(),
        server_address: "127.0.0.1:12345".into(),
        result: String::new(),
        users: Arc::new(Vec::new()),
        content: Arc::new(VecDeque::new()),
        selected_users: Arc::new(Vec::new()),
        content_to_send: String::new(),
    };

    let launcher = AppLauncher::with_window(login_window);

    launcher
        .delegate(MainDelegate)
        .launch(initial_state)
        .expect_err("启动失败");
}

fn build_login_window() -> impl Widget<State> {
    let textbox_account = TextBox::new()
        .with_placeholder("账号")
        .fix_width(200.0)
        .lens(State::account);
    let textbox_passwd = TextBox::new()
        .with_placeholder("密码")
        .fix_width(200.0)
        .lens(State::passwd);
    let label_server_address: Label<State> = Label::new("服务器地址:");
    let textbox_server_address = TextBox::new()
        .with_placeholder("服务器地址")
        .fix_width(200.0)
        .lens(State::server_address);
    let msg = Label::new(|state: &State, _env: &Env| state.result.to_string());
    let button_login =
        Button::new("登录").on_click(|_ctx, state: &mut State, _| handle_login_click(_ctx, state));
    let button_reg =
        Button::new("注册").on_click(|_ctx, state: &mut State, _| handle_reg_click(_ctx, state));
    let layout = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_server_address)
                .with_child(textbox_server_address),
        )
        .with_spacer(20.0)
        .with_child(textbox_account)
        .with_child(textbox_passwd)
        .with_spacer(5.0)
        .with_child(
            Flex::row()
                .with_child(button_login)
                .with_spacer(40.0)
                .with_child(button_reg),
        )
        .with_child(msg);

    Align::centered(layout)
}
