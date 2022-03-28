use imgui::Condition;
use imgui::StyleColor;
use imgui::Ui;
use imgui::Window;

use crate::ProgramStage;
use crate::ProgramState;

mod my_input_text;
pub mod new_screen;
pub mod start_screen;
pub mod erg_screen;
pub mod add_next_games_screen;

pub fn build(ui: &Ui, program_state: &mut ProgramState) {
    let menu_bar_height = program_state.size[1] / 8.0;

    // TODO: Extend for more screens
    match program_state.stage {
        ProgramStage::StartScreenStage => start_screen::build(ui, program_state, menu_bar_height),
        ProgramStage::NewScreenStage => new_screen::build(ui, program_state, menu_bar_height),
        ProgramStage::CurrentErgViewStage => erg_screen::build(ui, program_state, menu_bar_height),
        ProgramStage::AddNextGamesStage => add_next_games_screen::build(ui, program_state, menu_bar_height),
    };

    bottom_buttons(ui, program_state, menu_bar_height);
}

fn bottom_buttons(ui: &Ui, program_state: &mut ProgramState, height: f32) {
    let child_bg_color_token = ui.push_style_color(StyleColor::ChildBg, [0.2, 0.2, 0.2, 1.0]);
    let window_bg_color_token = ui.push_style_color(StyleColor::WindowBg, [0.2, 0.2, 0.2, 1.0]);
    let menu_bar_bg_color_token = ui.push_style_color(StyleColor::MenuBarBg, [0.2, 0.2, 0.2, 1.0]);
    Window::new("bottom_buttons")
        .size([program_state.size[0], height], Condition::Always)
        .position([0.0, program_state.size[1] - height], Condition::Always)
        .no_decoration()
        .movable(false)
        .focus_on_appearing(true)
        .menu_bar(true)
        .no_nav()
        .build(ui, || {
            ui.menu_bar(|| {
                // TODO: Replace with icon buttons (first create them^^ :D)
                // TODO: Extract functionality in order to use it with upper menu buttons
                if ui.button("New") {
                    program_state.switch_to_stage(ProgramStage::NewScreenStage);
                }
                if ui.button("Open") {
                    // TODO: Implement open saved data
                }
                if ui.button("Save") {
                    // TODO: Implement save data, same as "Save as" if no file to save is specified
                }
                if ui.button("Save as") {
                    // TODO: Implement save data as file (specify file)
                }
                if ui.button("Edit") {
                    // TODO: Implement edit competition button, only visible if competition already exists
                }
            });
        });
    menu_bar_bg_color_token.pop();
    window_bg_color_token.pop();
    child_bg_color_token.pop();
}

