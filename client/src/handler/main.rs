use druid::widget::prelude::*;

use crate::service::message::{
    flush_user_list, process_logout, process_message_content, process_message_group,
};
use crate::ui::login_view::State;

pub fn handle_main_window(ctx: &mut EventCtx<'_, '_>, state: &mut State) {
    flush_user_list(state).unwrap();
    process_message_content(ctx, state);
}
pub fn handle_logout_click(ctx: &mut EventCtx<'_, '_>, state: &mut State) {
    process_logout(ctx, state);
}
pub fn send_content_group(ctx: &mut EventCtx<'_, '_>, state: &mut State) {
    process_message_group(ctx, state);
}
