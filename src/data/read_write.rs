use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{self, TryRecvError},
};

use chrono::{Duration, Local};
use timer::Timer;

use crate::{ProgramStage, ProgramState};

#[cfg(test)]
use super::Team;

use super::CompetitionData;

pub fn save_to_file(file_path: PathBuf, data: &CompetitionData) -> Result<(), String> {
    let json = data.get_as_json_string();
    if let Some(parent) = file_path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(_) => return Err(String::from("Creation of parents dir failed!")),
        };
    }
    match fs::write(file_path, json) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Write to file failed!")),
    }
}

pub fn read_from_file(path: PathBuf) -> Result<CompetitionData, String> {
    let json_string = &fs::read_to_string(&path);
    let json_string = match json_string {
        Ok(bytes) => bytes,
        Err(_) => return Err(format!("Error whilst reading file: {}", path.display())),
    };

    dbg!("{}", json_string);

    match serde_json::from_str(json_string) {
        Ok(competition_data) => Ok(competition_data),
        Err(_) => Err(String::from("JSON was not well-formatted")),
    }
}

#[cfg(test)]
#[test]
fn test_read_write() {
    let mut data = CompetitionData {
        name: String::from("Mustermeisterschaft"),
        date_string: String::from("01.01.2022"),
        place: String::from("Musterstadt"),
        executor: String::from("SV Musterverein"),
        organizer: String::from("Musterverband"),
        referee: String::from("Max Muterschiedsrichter"),
        competition_manager: String::from("Erika Musterwettbewerbsleiter"),
        clerk: String::from("Max Musterschriftführer"),
        additional_text : String::from("Der SV Musterverein bedankt sich für die Teilnahme\nund wünscht ein sichere Heimreise!"),
        count_teams: 20,
        team_distribution: [2, 10],
        teams: Some(vec![
            vec![
                Team {
                    name: String::from("Musterteam A"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername A.1")),
                        Some(String::from("Mustername A.2")),
                        Some(String::from("Mustername A.3")),
                        Some(String::from("Mustername A.4")),
                        None,
                        None,
                        //Some(String::from("Mustername A.5")),
                        //Some(String::from("Mustername A.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam B"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername B.1")),
                        Some(String::from("Mustername B.2")),
                        Some(String::from("Mustername B.3")),
                        Some(String::from("Mustername B.4")),
                        Some(String::from("Mustername B.5")),
                        Some(String::from("Mustername B.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam C"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername C.1")),
                        Some(String::from("Mustername C.2")),
                        Some(String::from("Mustername C.3")),
                        Some(String::from("Mustername C.4")),
                        Some(String::from("Mustername C.5")),
                        Some(String::from("Mustername C.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam D"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername D.1")),
                        Some(String::from("Mustername D.2")),
                        Some(String::from("Mustername D.3")),
                        Some(String::from("Mustername D.4")),
                        Some(String::from("Mustername D.5")),
                        Some(String::from("Mustername D.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam E"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername E.1")),
                        Some(String::from("Mustername E.2")),
                        Some(String::from("Mustername E.3")),
                        Some(String::from("Mustername E.4")),
                        Some(String::from("Mustername E.5")),
                        Some(String::from("Mustername E.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam F"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername F.1")),
                        Some(String::from("Mustername F.2")),
                        Some(String::from("Mustername F.3")),
                        Some(String::from("Mustername F.4")),
                        Some(String::from("Mustername F.5")),
                        Some(String::from("Mustername F.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam G"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername G.1")),
                        Some(String::from("Mustername G.2")),
                        Some(String::from("Mustername G.3")),
                        Some(String::from("Mustername G.4")),
                        Some(String::from("Mustername G.5")),
                        Some(String::from("Mustername G.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam H"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername H.1")),
                        Some(String::from("Mustername H.2")),
                        Some(String::from("Mustername H.3")),
                        Some(String::from("Mustername H.4")),
                        Some(String::from("Mustername H.5")),
                        Some(String::from("Mustername H.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam I"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername I.1")),
                        Some(String::from("Mustername I.2")),
                        Some(String::from("Mustername I.3")),
                        Some(String::from("Mustername I.4")),
                        Some(String::from("Mustername I.5")),
                        Some(String::from("Mustername I.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam J"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername J.1")),
                        Some(String::from("Mustername J.2")),
                        Some(String::from("Mustername J.3")),
                        Some(String::from("Mustername J.4")),
                        Some(String::from("Mustername J.5")),
                        Some(String::from("Mustername J.6")),
                    ],
                },
                /*Team {
                    name: String::from("Musterteam K"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername K.1")),
                        Some(String::from("Mustername K.2")),
                        Some(String::from("Mustername K.3")),
                        Some(String::from("Mustername K.4")),
                        Some(String::from("Mustername K.5")),
                        Some(String::from("Mustername K.6")),
                    ],
                },*/
            ],
            vec![
                Team {
                    name: String::from("Musterteam N"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername N.1")),
                        Some(String::from("Mustername N.2")),
                        Some(String::from("Mustername N.3")),
                        Some(String::from("Mustername N.4")),
                        Some(String::from("Mustername N.5")),
                        Some(String::from("Mustername N.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam O"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername O.1")),
                        Some(String::from("Mustername O.2")),
                        Some(String::from("Mustername O.3")),
                        Some(String::from("Mustername O.4")),
                        Some(String::from("Mustername O.5")),
                        Some(String::from("Mustername O.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam P"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername P.1")),
                        Some(String::from("Mustername P.2")),
                        Some(String::from("Mustername P.3")),
                        Some(String::from("Mustername P.4")),
                        Some(String::from("Mustername P.5")),
                        Some(String::from("Mustername P.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam Q"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername Q.1")),
                        Some(String::from("Mustername Q.2")),
                        Some(String::from("Mustername Q.3")),
                        Some(String::from("Mustername Q.4")),
                        Some(String::from("Mustername Q.5")),
                        Some(String::from("Mustername Q.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam R"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername R.1")),
                        Some(String::from("Mustername R.2")),
                        Some(String::from("Mustername R.3")),
                        Some(String::from("Mustername R.4")),
                        Some(String::from("Mustername R.5")),
                        Some(String::from("Mustername R.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam S"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername S.1")),
                        Some(String::from("Mustername S.2")),
                        Some(String::from("Mustername S.3")),
                        Some(String::from("Mustername S.4")),
                        Some(String::from("Mustername S.5")),
                        Some(String::from("Mustername S.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam T"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername T.1")),
                        Some(String::from("Mustername T.2")),
                        Some(String::from("Mustername T.3")),
                        Some(String::from("Mustername T.4")),
                        Some(String::from("Mustername T.5")),
                        Some(String::from("Mustername T.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam U"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername U.1")),
                        Some(String::from("Mustername U.2")),
                        Some(String::from("Mustername U.3")),
                        Some(String::from("Mustername U.4")),
                        Some(String::from("Mustername U.5")),
                        Some(String::from("Mustername U.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam V"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername V.1")),
                        Some(String::from("Mustername V.2")),
                        Some(String::from("Mustername V.3")),
                        Some(String::from("Mustername V.4")),
                        Some(String::from("Mustername V.5")),
                        Some(String::from("Mustername V.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam W"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername W.1")),
                        Some(String::from("Mustername W.2")),
                        Some(String::from("Mustername W.3")),
                        Some(String::from("Mustername W.4")),
                        Some(String::from("Mustername W.5")),
                        Some(String::from("Mustername W.6")),
                    ],
                },
                /*Team {
                    name: String::from("Musterteam X"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername X.1")),
                        Some(String::from("Mustername X.2")),
                        Some(String::from("Mustername X.3")),
                        Some(String::from("Mustername X.4")),
                        Some(String::from("Mustername X.5")),
                        Some(String::from("Mustername X.6")),
                    ],
                },*/
            ],
        ]),
        group_names: Some(vec![
            String::from("Gruppe BLAU"),
            String::from("Gruppe ROT"),
        ]),
        matches: vec![],
        current_batch: vec![1, 0],
        with_break: true,
    };

    data.generate_matches();

    let export_result = save_to_file(PathBuf::from("./tmp/documents/save.json"), &data);

    assert!(export_result.is_ok());

    let read_data = read_from_file(PathBuf::from("./tmp/documents/save.json").to_path_buf());

    assert!(read_data.is_ok());
    let read_data = read_data.unwrap();

    debug_assert_eq!(data.name, read_data.name);
    debug_assert_eq!(data.date_string, read_data.date_string);
    debug_assert_eq!(data.place, read_data.place);
    debug_assert_eq!(data.executor, read_data.executor);
    debug_assert_eq!(data.organizer, read_data.organizer);
    debug_assert_eq!(data.referee, read_data.referee);
    debug_assert_eq!(data.competition_manager, read_data.competition_manager);
    debug_assert_eq!(data.clerk, read_data.clerk);
    debug_assert_eq!(data.additional_text, read_data.additional_text);
    debug_assert_eq!(data.count_teams, read_data.count_teams);
    debug_assert_eq!(data.team_distribution, read_data.team_distribution);
    debug_assert_eq!(data.teams.is_none(), read_data.teams.is_none());
    if let Some(data_teams) = data.teams.as_ref() {
        if let Some(read_teams) = read_data.teams.as_ref() {
            debug_assert_eq!(data_teams.len(), read_teams.len());
            data_teams
                .iter()
                .zip(read_teams.iter())
                .for_each(|(data_group, read_group)| {
                    debug_assert_eq!(data_group.len(), read_group.len());
                    data_group
                        .iter()
                        .zip(read_group.iter())
                        .for_each(|(data_team, read_team)| {
                            debug_assert_eq!(data_team.name, read_team.name);
                            debug_assert_eq!(data_team.region, read_team.region);
                            debug_assert_eq!(data_team.player_names, read_team.player_names);
                        });
                });
        }
    }
    debug_assert_eq!(data.group_names, read_data.group_names);
    debug_assert_eq!(data.matches.len(), read_data.matches.len());

    data.matches.iter().zip(read_data.matches.iter()).for_each(
        |(data_group_matches, read_group_matches)| {
            debug_assert_eq!(data_group_matches.len(), read_group_matches.len());
            data_group_matches
                .iter()
                .zip(read_group_matches.iter())
                .for_each(|(data_match, read_match)| {
                    debug_assert_eq!(data_match.team_a, read_match.team_a);
                    debug_assert_eq!(data_match.team_b, read_match.team_b);
                    debug_assert_eq!(data_match.points, read_match.points);
                    debug_assert_eq!(data_match.result, read_match.result);
                    debug_assert_eq!(data_match.batch, read_match.batch);
                    debug_assert_eq!(data_match.lane, read_match.lane);
                });
        },
    );

    debug_assert_eq!(data.current_batch, read_data.current_batch);
    debug_assert_eq!(data.with_break, read_data.with_break);
}

pub fn check_read_write_threads_messages(program_state: &mut ProgramState) {
    // check if any of the save threads send a new message and remove the corresponding entry iff the thread has finished its work
    let mut i = 0;
    while i < program_state.threads.save_channels.len() {
        let channel = &program_state.threads.save_channels[i];
        let path_res = match channel.try_recv() {
            Ok(path_res) => path_res,
            Err(_) => {
                i += 1;
                continue;
            }
        };

        // check for error and else save the data to the given path
        match path_res {
            Ok(path_opt) => match path_opt {
                Some(path) => match program_state.competition.handle_save_file(path) {
                    Ok(_) => (),
                    Err(msg) => show_error_message("save_as_button", msg),
                },
                None => eprintln!("[save_as_button]: FileDialog returned None path!"),
            },
            Err(msg) => show_error_message("save_as_button", msg.to_string()),
        };

        program_state.threads.save_channels.remove(i);
    }

    // check if any of the open threads send a new message and remove the corresponding entry iff the thread has finished its work
    i = 0;
    while i < program_state.threads.open_channels.len() {
        let channel = &program_state.threads.open_channels[i];
        let path_res = match channel.try_recv() {
            Ok(path_res) => path_res,
            Err(_) => {
                i += 1;
                continue;
            }
        };

        // check for errors and else load the data from the file and switch to the CurrentErgViewStage
        match path_res {
            Ok(path_opt) => match path_opt {
                Some(path) => match program_state.competition.handle_open_file(path) {
                    Ok(_) => program_state.switch_to_stage(ProgramStage::CurrentErgViewStage),
                    Err(msg) => show_error_message("open_button", msg),
                },
                None => eprintln!("[open_button]: FileDialog returned None path!"),
            },
            Err(msg) => show_error_message("open_button", msg.to_string()),
        }

        program_state.threads.open_channels.remove(i);
    }
}

// helper for showing error messages
fn show_error_message(function: &str, msg: String) {
    // TODO:
    /*match native_dialog::MessageDialog::new().set_type(MessageType::Error).set_title("Error occurred!").set_text(format!("An error occurred during opening the file open dialog. Please try again.\nError Message:\n{msg}").as_str()).show_alert() {
        Ok(_) => (),
        Err(_) => eprintln!("[{function}]: Could not open message dialog! Error message: {msg}"),
    }*/
}

// FIXME: Crashes because channel is closed. Either find the bug or remove the library and write own implementation
pub fn spawn_autosave_timer(interval: Duration, program_state: &mut ProgramState) {
    let (tx, rx) = mpsc::channel();
    let timer = Timer::new();
    let guard = timer.schedule_repeating(interval, move || {
        // TODO: Log it!
        println!("Autosave Timer: Sending save signal message!");
        match tx.send(()) {
            Ok(()) => (),
            Err(_) => panic!("Autosave Timer: Cannot send message!"),
        }
    });

    program_state.threads.autosave_guard = Some(guard);
    program_state.threads.autosave_channel = Some(rx);
    program_state.threads.timer = Some(timer);
}

pub fn check_autosave_thread_messages(program_state: &mut ProgramState) {
    let rx = match program_state.threads.autosave_channel.as_ref() {
        Some(rx) => rx,
        None => return,
    };

    match rx.try_recv() {
        Ok(()) => (),
        Err(TryRecvError::Empty) => return,
        Err(TryRecvError::Disconnected) => panic!("Autosave Timer: Sender disconnected!"),
    }

    // use set file path or the default path in tmp
    let path = match program_state.competition.absolute_file_path.as_ref() {
        Some(path) => path.to_path_buf(),
        None => match default_path_for_autosave(program_state) {
            Ok(default_path) => default_path,
            Err(_) => return, // TODO: Log it!,
        },
    };

    // TODO: Log it
    println!(
        "Received autosave signal, starting save to {} at {}",
        path.display().to_string(),
        Local::now().format("%d.%m.%Y %H:%M:%S")
    );

    match program_state.competition.handle_save_file(path) {
        Ok(_) => println!(
            "Finished autosave at {}",
            Local::now().format("%d.%m.%Y %H:%M:%S")
        ),
        Err(_) => (), //TODO: LOG it,
    }
}

// TODO: Set this to more useful default path than tmp dir
fn default_path_for_autosave(program_state: &ProgramState) -> Result<PathBuf, String> {
    let default_folder = std::env::temp_dir().join("israt").join("autosaves");
    let data = match program_state.competition.data.as_ref() {
        Some(data) => data,
        None => {
            return Err(String::from(
                "There is no competition data, cannot create default autosave file name!",
            ))
        }
    };

    let sanitized_compettion_name = data.name.replace(&['/', '\\', '%', '.', '~'], "");
    let autosave_file_name = format!(
        "{sanitized_compettion_name}-{}.json",
        Local::now().format("%Y%m%d-%H%M")
    );
    return Ok(default_folder.join(autosave_file_name));
}
