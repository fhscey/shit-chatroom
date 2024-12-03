use crate::handler::main::{handle_logout_click, handle_main_window, send_content_group};
use crate::service::message::flush_user_list;
use crate::ui::login_view::{MessageContent, State, ADD_USER_TO_GROUP};
use chatroom_net::protocol::UserList;
use druid::{
    widget::{prelude::*, Button, CrossAxisAlignment, Flex, Label, List, Scroll, TextBox},
    Command, Target, WidgetExt, WindowDesc,
};

use std::sync::Arc;

pub fn launch_main_window(ctx: &mut EventCtx<'_, '_>, state: &mut State) {
    let new = WindowDesc::new(build_main_window())
        .window_size((900.0, 600.0))
        .title("chatroom");
    ctx.new_window(new);

    handle_main_window(ctx, state);
}

pub fn build_main_window() -> impl Widget<State> {
    let mut header = Flex::row()
        .with_child(
            Button::new("注销")
                .fix_width(60.0)
                .on_click(|_ctx, state: &mut State, _| handle_logout_click(_ctx, state)),
        )
        .with_default_spacer()
        .with_child(Button::new("刷新用户列表").fix_width(180.0).on_click(
            |_ctx, state: &mut State, _| {
                flush_user_list(state);
            },
        ))
        .with_default_spacer()
        .with_child(
            Button::new("刷新选中")
                .fix_width(120.0)
                .on_click(|_ctx, state: &mut State, _| flush_selected(state)),
        );
    Flex::row()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(header)
                .with_default_spacer()
                .with_flex_child(
                    Scroll::new(List::new(list_user).lens(State::users)).vertical(),
                    1.0,
                ),
        )
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row()
                        .with_child(
                            TextBox::new()
                                .with_placeholder("输入消息")
                                .fix_width(460.0)
                                .lens(State::content_to_send),
                        )
                        .with_child(Button::new("发送").on_click(|_ctx, state: &mut State, _| {
                            send_content_group(_ctx, state)
                        })),
                )
                .with_child(Label::dynamic(|state: &State, _| state.result.clone()))
                .with_default_spacer()
                .with_flex_child(
                    Scroll::new(List::new(list_content).lens(State::content)).vertical(),
                    1.0,
                ),
        )
}

fn list_user() -> impl Widget<UserList> {
    Flex::row()
        .with_child(
            Button::dynamic(|d: &UserList, _| d.account.clone())
                .on_click(|_ctx, user: &mut UserList, _| select(_ctx, user))
                .disabled_if(|user: &UserList, _| user.is_selected),
        )
        .fix_width(360.0)
}

fn select(ctx: &mut EventCtx<'_, '_>, user: &mut UserList) {
    user.is_selected = !user.is_selected;
    user.state = !user.state;
    ctx.submit_command(Command::new(
        ADD_USER_TO_GROUP,
        user.account.clone(),
        Target::Auto,
    ));
}

fn flush_selected(state: &mut State) {
    for user in Arc::make_mut(&mut state.users) {
        user.is_selected = false;
    }
    state.selected_users = Arc::new(Vec::new())
}

fn list_content() -> impl Widget<MessageContent> {
    Flex::row()
        .with_child(Label::dynamic(|d: &MessageContent, _| {
            format!(
                "({})->({}):{} {}",
                d.s_account, d.r_content, d.content, d.state
            )
        }))
        .fix_width(650.0)
}
