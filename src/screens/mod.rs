use imgui::{ChildWindow, StyleColor, Ui};

use crate::colors::{BORDER, ELEVATED_BACKGROUND, TEXT};
use crate::constants::BORDER_THICKNESS;
use crate::constants::{
    BOTTOM_MENU_HEIGHT, BOTTOM_MENU_WIDTH, NAVIGATION_BAR_HEIGHT, NAVIGATION_BAR_WIDTH,
    TOP_MENU_HEIGHT, TOP_MENU_WIDTH,
};
use crate::ProgramStage;
use crate::ProgramState;

mod my_input_text;

pub mod buttons;
pub mod erg_screen;
pub mod home_screen;
pub mod new_screen;
pub mod start_screen;

pub fn build(ui: &Ui, program_state: &mut ProgramState) {
    let text_token = ui.push_style_color(StyleColor::Text, TEXT);
    build_top_menu(ui, program_state);
    build_navigation_bar(ui, program_state);
    build_bottom_menu(ui, program_state);
    build_selected_screen(ui, program_state);
    text_token.pop();
}

fn build_top_menu(ui: &Ui, program_state: &mut ProgramState) {
    let bg_token = ui.push_style_color(StyleColor::ChildBg, ELEVATED_BACKGROUND);
    ChildWindow::new("##top_menu")
        .size([
            program_state.size[0] * TOP_MENU_WIDTH,
            program_state.size[1] * TOP_MENU_HEIGHT,
        ])
        .bring_to_front_on_focus(false)
        .movable(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(ui, || {
            let window_size = ui.window_size();
            // draw border on bottom side
            ui.get_window_draw_list()
                .add_line(
                    [0.0, window_size[1] - BORDER_THICKNESS],
                    [window_size[0], window_size[1] - BORDER_THICKNESS],
                    BORDER,
                )
                .thickness(BORDER_THICKNESS)
                .build();
        });
    bg_token.pop();
}

fn build_navigation_bar(ui: &Ui, program_state: &mut ProgramState) {
    let bg_token = ui.push_style_color(StyleColor::ChildBg, ELEVATED_BACKGROUND);
    let y_offset = program_state.size[1] * TOP_MENU_HEIGHT;
    ui.set_cursor_pos([0.0, y_offset]);
    ChildWindow::new("##navigation_bar")
        .size([
            program_state.size[0] * NAVIGATION_BAR_WIDTH,
            program_state.size[1] * NAVIGATION_BAR_HEIGHT,
        ])
        .bring_to_front_on_focus(false)
        .movable(false)
        .scroll_bar(true)
        .scrollable(true)
        .build(ui, || {
            let window_size = ui.window_size();
            // draw border on right side
            ui.get_window_draw_list()
                .add_line(
                    [window_size[0] - BORDER_THICKNESS, y_offset],
                    [window_size[0] - BORDER_THICKNESS, y_offset + window_size[1]],
                    BORDER,
                )
                .thickness(BORDER_THICKNESS)
                .build();
        });
    bg_token.pop();
}

fn build_bottom_menu(ui: &Ui, program_state: &mut ProgramState) {
    let bg_token = ui.push_style_color(StyleColor::ChildBg, ELEVATED_BACKGROUND);
    let y_offset = program_state.size[1] * (TOP_MENU_HEIGHT + NAVIGATION_BAR_HEIGHT);
    ui.set_cursor_pos([0.0, y_offset]);
    ChildWindow::new("##bottom_menu")
        .size([
            program_state.size[0] * BOTTOM_MENU_WIDTH,
            program_state.size[1] * BOTTOM_MENU_HEIGHT,
        ])
        .bring_to_front_on_focus(false)
        .movable(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(ui, || {
            let window_size = ui.window_size();
            // draw border on top side
            ui.get_window_draw_list()
                .add_line([0.0, y_offset], [window_size[0], y_offset], BORDER)
                .thickness(BORDER_THICKNESS)
                .build();
        });
    bg_token.pop();
}

fn build_selected_screen(ui: &Ui, program_state: &mut ProgramState) {
    ui.set_cursor_pos([
        program_state.size[0] * NAVIGATION_BAR_WIDTH,
        program_state.size[1] * TOP_MENU_HEIGHT,
    ]);
    match program_state.stage {
        ProgramStage::HomeStage => home_screen::build(ui, program_state),
        _ => todo!(),
    }
}
