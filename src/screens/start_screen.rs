use imgui::{ChildWindow, Condition, StyleColor, Ui, Window};

use crate::{ProgramState, ProgramStage};

pub fn build(ui: &Ui, program_state: &ProgramState, menu_bar_height: f32) {
    let child_bg_color = ui.push_style_color(StyleColor::ChildBg, [0.0, 0.0, 0.0, 1.0]);
    let window_bg_color = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]);

    let text = "Open a file or create a new one!";
    let _text_size = ui.calc_text_size(text);
    let text_len = _text_size[0];
    let text_height = _text_size[1];
    ChildWindow::new("Main screen")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_inputs()
        .focus_on_appearing(true)
        .build(ui, || {
            Window::new("Text_window")
                .no_decoration()
                .no_inputs()
                .bring_to_front_on_focus(true)
                .position(
                    [
                        (program_state.size[0] - text_len) / 2.0,
                        (program_state.size[1] - menu_bar_height) / 2.0,
                    ],
                    Condition::Always,
                )
                .size([text_len + 20.0, text_height + 20.0], Condition::Always)
                .build(ui, || ui.text(text))
        });

    window_bg_color.pop();
    child_bg_color.pop();
}

pub fn bottom_buttons(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("New") {
        program_state.switch_to_stage(ProgramStage::NewScreenStage);
    }

    if ui.button("Open") {
        todo!();
    }
}
