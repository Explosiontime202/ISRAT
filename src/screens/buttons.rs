use std::{sync::mpsc, thread};

use crate::ProgramState;
use imgui::Ui;
use native_dialog::FileDialog;

// this crate defines functions to draw common buttons and their windows & popups

pub fn new_button(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("New") {
        new_action(program_state);
    }
}

pub fn save_button(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("Save") {
        save_action(program_state);
    }
}

// draws an save as button for a given ProgramState on a given UI
pub fn save_as_button(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("Save as") {
        save_as_action(program_state);
    }
}

// draws an open button for a given ProgramState on a given UI
pub fn open_button(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("Open") {
        open_action(program_state);
    }
}

pub fn new_action(program_state: &mut ProgramState) {
    todo!();
}

pub fn save_action(program_state: &mut ProgramState) {
    if let Some(file_path) = program_state.competition.absolute_file_path.as_ref() {
        let file_path = file_path.clone();
        // save to file if file_name is known, e.g. if file was opened or previously saved to this file_path
        let export_res = program_state
            .competition
            .handle_save_file(file_path.to_path_buf());

        match export_res {
            Ok(_) => (),
            Err(msg) => program_state.button_state.export_err_msg = Some(msg),
        }
    } else {
        // else open the window to the let the user determine where to store the file
        save_as_action(program_state);
    }
}

pub fn save_as_action(program_state: &mut ProgramState) {
    debug_assert!(program_state.competition.data.is_some());

    let (tx, rx) = mpsc::channel();

    // clone values to be able to use them safely inside the thread
    let competition_name = program_state
        .competition
        .data
        .as_ref()
        .unwrap()
        .name
        .clone();
    let absolute_dir_path = program_state.competition.absolute_dir_path.clone();

    // open os save as file dialog in separate thread in order to not stop the GUI rendering
    thread::Builder::new()
        .name(String::from("Save as dialog thread"))
        .spawn(move || {
            let mut dialog = FileDialog::new();

            let filename_suggestion =
                format!("{}.json", competition_name.replace(" ", "_").to_lowercase());
            dialog = dialog.set_filename(filename_suggestion.as_str());

            // set path to directory iff available
            if let Some(dir_path) = absolute_dir_path.as_ref() {
                dialog = dialog.set_location(dir_path);
            }

            let dialog_res = dialog.show_save_single_file();

            // inform main (GUI) Thread about closed dialog
            tx.send(dialog_res)
                .expect("Channel is closed, this is not expected");
        })
        .expect("This shouldn't happen!");

    program_state.threads.save_channels.push(rx);
}

pub fn open_action(program_state: &mut ProgramState) {
    let (tx, rx) = mpsc::channel();
    let absolute_dir_path = program_state.competition.absolute_dir_path.clone();

    // open os open file dialog in separate thread in order to not stop the GUI rendering
    thread::Builder::new()
        .name(String::from("Open dialog thread"))
        .spawn(move || {
            let mut open_dialog =
                FileDialog::new().add_filter("ISRAT Data Files", &["json", "isra"]);

            if let Some(dir_path) = absolute_dir_path.as_ref() {
                open_dialog = open_dialog.set_location(dir_path);
            }

            let dialog_res = open_dialog.show_open_single_file();

            // inform main (GUI) Thread about closed dialog
            tx.send(dialog_res)
                .expect("Channel is closed, this is not expected");
        })
        .expect("This shouldn't happen!");

    program_state.threads.open_channels.push(rx);
}

pub struct ButtonState {
    pub export_err_msg: Option<String>,
}

impl ButtonState {
    pub fn empty() -> Self {
        ButtonState {
            export_err_msg: None,
        }
    }
}
