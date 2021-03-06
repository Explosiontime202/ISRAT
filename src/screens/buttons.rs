use crate::{data::import_export::save_to_file, ProgramState, ProgramStage};
use imgui::{Condition, Ui, Window};
use native_dialog::{FileDialog, MessageType};

// this crate defines functions to draw common buttons and their windows & popups

pub fn draw_button_windows_and_popups(ui: &Ui, program_state: &mut ProgramState) {
    let button_state = &mut program_state.button_state;

    if button_state.save_as_window {
        Window::new("Save as")
            .movable(true)
            .resizable(false)
            .collapsible(false)
            .opened(&mut button_state.save_as_window)
            .size(
                [program_state.size[0] / 3.0, program_state.size[1] / 3.0],
                Condition::Always,
            )
            .build(ui, || {
                ui.text("Save as?");
                ui.text("todo!");
            });
    }
}

pub fn save_button(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("Save") {
        if let Some(file_path) = program_state.competition.absolute_file_path.as_ref() {
            // save to file if file_name is known, e.g. if file was opened or previously saved to this file_path
            let data = program_state.competition.data.as_ref().unwrap();
            let export_result = save_to_file(String::from(file_path.to_str().unwrap()), data)
                .join()
                .expect("Join on export thread thrown an unexpected error!");

            match export_result {
                Ok(_) => (),
                Err(msg) => program_state.button_state.export_err_msg = Some(msg),
            }
        } else {
            // else open the window to the let the user determine where to store the file
            program_state.button_state.save_as_window = true;
        }
    }
}

// draws an save as button for a given ProgramState on a given UI
pub fn save_as_button(ui: &Ui, program_state: &mut ProgramState) {
    if ui.button("Save as") {
        debug_assert!(program_state.competition.data.is_some());
        // constructs and shows the os save as dialog
        let mut dialog =  FileDialog::new();

        // set competition name as filename suggestion
        let data = program_state.competition.data.as_ref().unwrap();
        let filename_suggestion = format!("{}.json", data.name.replace(" ", "_").to_lowercase());
        dialog = dialog.set_filename(filename_suggestion.as_str());

        if let Some(dir_path) = &program_state.competition.absolute_dir_path {

            dialog = dialog.set_location(dir_path.as_path());
        }

        let save_res = dialog.show_save_single_file();

        // check for error and else save the data to the given path
        match save_res {
            Ok(path_opt) => match path_opt {
                Some(path) => match program_state.competition.handle_save_file(path) {
                    Ok(_) => (),
                    Err(msg) => show_error_message("save_as_button", msg)
                }
                None => eprintln!("[save_as_button]: FileDialog returned None path!"),
            },
            Err(msg) => show_error_message("save_as_button", msg.to_string()),
        }
    
    }
}

// draws an open button for a given ProgramState on a given UI
pub fn open_button(ui: &Ui, program_state: &mut ProgramState) {
    // draw open button
    if ui.button("Open") {
        // opens the os file open dialog
        let path_res = FileDialog::new()
            .add_filter("ISRAT Data Files", &["json", "isra"])
            .show_open_single_file();

        // check for errors and else load the data from the file and switch to the CurrentErgViewStage
        match path_res {
            Ok(path_opt) => match path_opt {
                Some(path) => match 
                    program_state.competition.handle_open_file(path) {
                        Ok(_) => program_state.switch_to_stage(ProgramStage::CurrentErgViewStage),
                        Err(msg) => show_error_message("open_button", msg),
                },
                None => eprintln!("[open_button]: FileDialog returned None path!"),
            },
            Err(msg) => show_error_message("open_button", msg.to_string()),
        }


    }
}

// helper for showing error messages
fn show_error_message(function: &str, msg: String) {
    match native_dialog::MessageDialog::new().set_type(MessageType::Error).set_title("Error occurred!").set_text(format!("An error occurred during opening the file open dialog. Please try again.\nError Message:\n{msg}").as_str()).show_alert() {
        Ok(_) => (),
        Err(_) => eprintln!("[{function}]: Could not open message dialog! Error message: {msg}"),
    }
}

pub struct ButtonState {
    pub save_as_window: bool,
    pub export_err_msg: Option<String>,
}

impl ButtonState {
    pub fn empty() -> Self {
        ButtonState {
            save_as_window: false,
            export_err_msg: None,
        }
    }
}
