use imgui::{ChildWindow, StyleColor, Ui};

use crate::{
    colors::{BACKGROUND, BLUE, ELEVATED_BACKGROUND, GREEN, RED, TEXT},
    constants::{SELECTED_SCREEN_HEIGHT, SELECTED_SCREEN_WIDTH},
    ProgramState,
};

pub fn build(ui: &Ui, program_state: &mut ProgramState) {
    let text_token = ui.push_style_color(StyleColor::Text, TEXT);
    let bg_token = ui.push_style_color(StyleColor::ChildBg, BACKGROUND);
    ChildWindow::new("##home_screen")
        .size([
            program_state.size[0] * SELECTED_SCREEN_WIDTH,
            program_state.size[1] * SELECTED_SCREEN_HEIGHT,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(ui, || {});
    bg_token.pop();
    text_token.pop();
}
