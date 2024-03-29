#![windows_subsystem = "windows"]

use std::{path::PathBuf, sync::mpsc::Receiver};

use chrono::Duration;
use data::{
    read_write::{
        check_autosave_thread_messages, check_read_write_threads_messages, spawn_autosave_timer,
    },
    Competition, CompetitionData, Team,
};
use imgui::*;
use main_menu_bar::MainMenuBarState;
use native_dialog::Error;
use screens::{buttons::ButtonState, erg_screen::ErgScreenState, new_screen::NewScreenState};
use timer::{Guard, Timer};
use winit::window::Fullscreen;

mod common;
mod data;
mod main_menu_bar;
mod screens;
mod support;

fn main() {
    let mut system = support::init("ISRAT");

    // set borderless full screen at start
    system
        .display
        .gl_window()
        .window()
        .set_fullscreen(Some(Fullscreen::Borderless(None)));

    // get monitor size
    let size = system.display.gl_window().window().inner_size();

    // initialize program state
    system.program_state = Some(ProgramState::new(
        ProgramStage::StartScreenStage,
        [size.width as f32, size.height as f32],
    ));

    // TODO: Make interval adjustable by using GUI settings or config in home directory
    spawn_autosave_timer(Duration::minutes(1), system.program_state.as_mut().unwrap());

    // TODO: Remove for productive builds
    #[cfg(debug_assertions)]
    initial_state(system.program_state.as_mut().unwrap());

    // set color theme
    let style = system.imgui.style_mut();
    style.colors[StyleColor::TitleBgActive as usize] = style.colors[StyleColor::TitleBg as usize];

    // start main loop
    system.main_loop(|run, ui, window, state| {
        let size = window.inner_size();
        state.size = [size.width as f32, size.height as f32];

        let window_border_size_token = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
        let window_padding_token = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        Window::new("ISRAT")
            .size(state.size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .no_decoration()
            .title_bar(true)
            .no_nav()
            .bring_to_front_on_focus(false)
            .resizable(false)
            .opened(run)
            .build(ui, || {
                screens::build(ui, state);

                // Escape is pressed, exit fullscreen mode
                if ui.io().keys_down[36] {
                    window.set_fullscreen(None);
                }

                // F11 is pressed, enter fullscreen mode
                if ui.io().keys_down[47] {
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }

                //if let Some(key) = ui.io().keys_down.iter().position(|&k| k == true) {
                //    println!("pressed_key = {}", key);
                //}

                main_menu_bar::draw_main_menu_bar(ui, state);
                check_for_thread_messages(state);
                /*ui.text("Hello world!");
                ui.text("こんにちは世界！");
                ui.text("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
                let bg_color = ui.push_style_color(StyleColor::ChildBg, [1.0, 0.0, 0.0, 1.0]);*/
                /*Window::new("Hello Welt")
                .size([200.0, 200.0], Condition::Always)
                .no_decoration()
                .position([200.0, 100.0], Condition::Always)
                .build(ui, || {
                    let text_color =
                        ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);
                    ui.text(format!("Screen Size: ({:.1}, {:.1})", width, height));
                    let c = ui.style_color(StyleColor::Text);
                    ui.text(format!("{} {} {} {}", c[0], c[1], c[2], c[3]));
                    text_color.pop();
                });*/
                /*ChildWindow::new("Hello Welt")
                    .size([200.0, 200.0])
                    .build(ui, || {
                        let text_color =
                            ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);
                        ui.text(format!("Screen Size: ({:.1}, {:.1})", 0.0, 0.0));
                        let c = ui.style_color(StyleColor::Text);
                        ui.text(format!("{} {} {} {}", c[0], c[1], c[2], c[3]));
                        text_color.pop();
                    });
                bg_color.pop();*/
            });
        window_padding_token.pop();
        window_border_size_token.pop();
    });
}

fn check_for_thread_messages(program_state: &mut ProgramState) {
    check_read_write_threads_messages(program_state);
    check_autosave_thread_messages(program_state);
}

// TODO: Remove for productive builds
#[cfg(debug_assertions)]
fn initial_state(state: &mut ProgramState) {
    use std::path::Path;

    use crate::data::MatchResult;

    state.stage = ProgramStage::CurrentErgViewStage;
    state.competition.data = Some(CompetitionData {
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
    });
    state.new_screen_state = None;
    state.erg_screen_state = Some(ErgScreenState::new(2));
    state.competition.data.as_mut().unwrap().generate_matches();
    state.competition.current_interim_result = vec![None, None];

    {
        let relative_path = Path::new(if cfg!(target_os = "windows") {
            r".\documents\"
        } else {
            "./documents"
        });
        if !relative_path.exists() {
            std::fs::create_dir_all(relative_path).expect("Directory creation failed!");
        }

        let abs_path = if cfg!(target_os = "windows") {
            let tmp = std::fs::canonicalize(relative_path).expect("Canonicalize failed!");
            let tmp2 = tmp.to_str().unwrap();
            let tmp3 = tmp2[4..tmp2.len()].to_string();
            let mut path_buf = PathBuf::new();
            path_buf.push(tmp3);
            path_buf
        } else {
            std::fs::canonicalize(relative_path).expect("Canonicalize failed!")
        };

        state.competition.absolute_dir_path = Some(abs_path);

        state.competition.absolute_file_path = Some(
            state
                .competition
                .absolute_dir_path
                .as_ref()
                .unwrap()
                .join("mustermeisterschaft.json"),
        );
    }

    let results = [
        MatchResult::WinnerA,
        MatchResult::WinnerB,
        MatchResult::Draw,
        MatchResult::WinnerB,
        MatchResult::WinnerA,
    ];
    let points = [[17, 13], [3, 11], [9, 9], [9, 13], [11, 5]];

    state.competition.data.as_mut().unwrap().matches[0]
        .iter_mut()
        .filter(|_match| _match.batch == 0 && _match.result != MatchResult::Break)
        .enumerate()
        .for_each(|(idx, _match)| {
            _match.result = results[idx];
            _match.points = Some(points[idx]);
        });

    let mut hash_set = std::collections::HashSet::new();
    state.competition.data.as_ref().unwrap().matches[0]
        .iter()
        .filter(|&_match| _match.result != MatchResult::Break)
        .map(|_match| {
            [
                _match.team_a.min(_match.team_b),
                _match.team_a.max(_match.team_b),
            ]
        })
        .for_each(|arr| {
            assert!(hash_set.insert(arr));
            assert_ne!(arr[0], arr[1]);
        });
}

#[derive(Clone, Copy)]
pub enum ProgramStage {
    StartScreenStage,
    NewScreenStage,
    CurrentErgViewStage,
}

pub struct ProgramState {
    pub stage: ProgramStage,
    pub size: [f32; 2],
    pub competition: Competition,
    pub new_screen_state: Option<NewScreenState>,
    pub erg_screen_state: Option<ErgScreenState>,
    pub button_state: ButtonState,
    pub main_menu_bar_state: MainMenuBarState,
    pub threads: ThreadState,
}

impl ProgramState {
    pub fn new(stage: ProgramStage, size: [f32; 2]) -> ProgramState {
        ProgramState {
            stage,
            size,
            competition: Competition::empty(),
            new_screen_state: None,
            erg_screen_state: None,
            button_state: ButtonState::empty(),
            main_menu_bar_state: MainMenuBarState::empty(),
            threads: ThreadState::new(),
        }
    }

    pub fn switch_to_stage(&mut self, new_stage: ProgramStage) {
        match new_stage {
            ProgramStage::StartScreenStage => {
                todo!("Currently not implemented StartScreenStage init!")
            }
            ProgramStage::NewScreenStage => {
                if self.new_screen_state.is_none() {
                    self.new_screen_state = Some(NewScreenState::new());
                }
                if self.competition.data.is_none() {
                    self.competition.data = Some(CompetitionData::empty());
                }
            }

            ProgramStage::CurrentErgViewStage => {
                // TODO: Add more state resets if needed
                self.new_screen_state = None;

                let group_count = self.competition.data.as_ref().unwrap().team_distribution[0];

                if self.erg_screen_state.is_none() {
                    self.erg_screen_state = Some(ErgScreenState::new(group_count as usize));
                }

                self.competition.current_interim_result = (0..group_count).map(|_| None).collect();
            }

            #[allow(unreachable_patterns)]
            _ => todo!("Implement stage switch for more stages!"),
        }
        self.stage = new_stage;
    }
}

pub struct ThreadState {
    pub save_channels: Vec<Receiver<Result<Option<PathBuf>, Error>>>,
    pub open_channels: Vec<Receiver<Result<Option<PathBuf>, Error>>>,
    pub autosave_channel: Option<Receiver<()>>,
    pub autosave_guard: Option<Guard>,
    pub timer: Option<Timer>,
}

impl ThreadState {
    pub fn new() -> Self {
        Self {
            save_channels: Vec::new(),
            open_channels: Vec::new(),
            autosave_channel: None,
            autosave_guard: None,
            timer: None,
        }
    }
}
