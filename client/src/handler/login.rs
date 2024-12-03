use crate::ui::login_view::State;

use druid::EventCtx;

use crate::service::message::{process_login, process_reg};

pub fn handle_login_click(ctx: &mut EventCtx<'_, '_>, state: &mut State) {
    process_login(ctx, state);
}

pub fn handle_reg_click(ctx: &mut EventCtx<'_, '_>, state: &mut State) {
    process_reg(state);
}
