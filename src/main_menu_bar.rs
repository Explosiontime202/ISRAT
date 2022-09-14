use imgui::{Condition, MenuItem, Ui, Window};

use crate::{common::center, screens::buttons, ProgramState};

pub fn draw_main_menu_bar(ui: &Ui, program_state: &mut ProgramState) {
    if let Some(_) = ui.begin_main_menu_bar() {
        if let Some(_) = ui.begin_menu("File") {
            if MenuItem::new("New").build(ui) {
                buttons::new_action(program_state);
            }
            if MenuItem::new("Open").build(ui) {
                buttons::open_action(program_state);
            }
            if MenuItem::new("Save").build(ui) {
                buttons::save_action(program_state);
            }
            if MenuItem::new("Save as").build(ui) {
                buttons::save_as_action(program_state);
            }
        }

        if let Some(_) = ui.begin_menu("Help") {
            if MenuItem::new("About").build(ui) {
                program_state.main_menu_bar_state.about_popup = true;
            }
        }
    }

    about_window(ui, program_state);
}

fn about_window(ui: &Ui, program_state: &mut ProgramState) {
    if program_state.main_menu_bar_state.about_popup {
        ui.open_popup("About");
        /*if let Some(_token) = ui.begin_popup("##about_popup") {
            ui.text("TEST");
        }*/
        /*ui.popup_modal("About").title_bar(true).movable(false).collapsible(false).resizable(false).build(ui, || {
            ui.text("TEST!")
        });*/
        Window::new("About")
            .movable(false)
            .resizable(false)
            .collapsible(false)
            .opened(&mut program_state.main_menu_bar_state.about_popup)
            .position(
                [
                    program_state.size[0] * (0.5 - 0.25),
                    program_state.size[1] * (0.5 - 0.25),
                ],
                Condition::Always,
            )
            .size(
                [program_state.size[0] * 0.5, program_state.size[1] * 0.5],
                Condition::Always,
            )
            .build(ui, || {
                center(ui, "ISRAT by Johannes Maier.");
                center(ui, "Licensed under MIT License, 2022. Free to use.");
                center(
                    ui,
                    "Source code can be found at https://github.com/Explosiontime202/ISRAT.",
                );
            });
    }
}

pub struct MainMenuBarState {
    pub about_popup: bool,
}

impl MainMenuBarState {
    pub fn empty() -> Self {
        Self { about_popup: false }
    }
}
