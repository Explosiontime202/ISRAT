use imgui::*;
use screens::new_screen::NewScreenState;
use winit::window::Fullscreen;

mod screens;

mod support;

// TODO: Add auto saving

fn main() {
    let mut system = support::init("Ergebnis-Manager");

    // set borderless full screen at start
    system
        .display
        .gl_window()
        .window()
        .set_fullscreen(Some(Fullscreen::Borderless(None)));

    // ger monitor size
    let size = system.display.gl_window().window().inner_size();

    // initialize program state
    let mut state = ProgramState::new(
        ProgramStage::StartScreenStage,
        [size.width as f32, size.height as f32],
    );

    // TODO: Remove for productive builds
    #[cfg(debug_assertions)]
    {
        state.stage = ProgramStage::CurrentErgViewStage;
        state.competition_data = Some(CompetitionData {
            name: String::from("Mustermeisterschaft"),
            date_string: String::from("01.01.2022"),
            place: String::from("Musterstadt"),
            executor: String::from("SV Musterverein"),
            organizer: String::from("Musterverband"),
            count_teams: 20,
            team_distribution: [2, 10],
            teams: Some(vec![
                vec![
                    Team {
                        name: String::from("Musterteam A"),
                        player_names: [
                            Some(String::from("Mustername A.1")),
                            Some(String::from("Mustername A.2")),
                            Some(String::from("Mustername A.3")),
                            Some(String::from("Mustername A.4")),
                            Some(String::from("Mustername A.5")),
                            Some(String::from("Mustername A.6")),
                        ],
                    },
                    Team {
                        name: String::from("Musterteam B"),
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
                        player_names: [
                            Some(String::from("Mustername J.1")),
                            Some(String::from("Mustername J.2")),
                            Some(String::from("Mustername J.3")),
                            Some(String::from("Mustername J.4")),
                            Some(String::from("Mustername J.5")),
                            Some(String::from("Mustername J.6")),
                        ],
                    },
                ],
                vec![
                    Team {
                        name: String::from("Musterteam K"),
                        player_names: [
                            Some(String::from("Mustername K.1")),
                            Some(String::from("Mustername K.2")),
                            Some(String::from("Mustername K.3")),
                            Some(String::from("Mustername K.4")),
                            Some(String::from("Mustername K.5")),
                            Some(String::from("Mustername K.6")),
                        ],
                    },
                    Team {
                        name: String::from("Musterteam L"),
                        player_names: [
                            Some(String::from("Mustername L.1")),
                            Some(String::from("Mustername L.2")),
                            Some(String::from("Mustername L.3")),
                            Some(String::from("Mustername L.4")),
                            Some(String::from("Mustername L.5")),
                            Some(String::from("Mustername L.6")),
                        ],
                    },
                    Team {
                        name: String::from("Musterteam M"),
                        player_names: [
                            Some(String::from("Mustername M.1")),
                            Some(String::from("Mustername M.2")),
                            Some(String::from("Mustername M.3")),
                            Some(String::from("Mustername M.4")),
                            Some(String::from("Mustername M.5")),
                            Some(String::from("Mustername M.6")),
                        ],
                    },
                    Team {
                        name: String::from("Musterteam N"),
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
                        player_names: [
                            Some(String::from("Mustername T.1")),
                            Some(String::from("Mustername T.2")),
                            Some(String::from("Mustername T.3")),
                            Some(String::from("Mustername T.4")),
                            Some(String::from("Mustername T.5")),
                            Some(String::from("Mustername T.6")),
                        ],
                    },
                ],
            ]),
            group_names: Some(vec![
                String::from("Gruppe BLAU"),
                String::from("Gruppe ROT"),
            ]),
        });
        state.new_screen_state = None;
    }

    // set color theme
    let style = system.imgui.style_mut();
    style.colors[StyleColor::TitleBgActive as usize] = style.colors[StyleColor::TitleBg as usize];

    // start main loop
    system.main_loop(move |run, ui, window| {
        let size = window.inner_size();
        state.size = [size.width as f32, size.height as f32];

        let window_border_size_token = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
        let window_padding_token = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        Window::new("Ergebnis-Manager")
            .size(state.size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .no_decoration()
            .title_bar(true)
            .no_nav()
            .bring_to_front_on_focus(false)
            .resizable(false)
            .opened(run)
            .build(ui, || {
                screens::build(ui, &mut state);

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

                /*add_main_menu(ui);
                ui.text("Hello world!");
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

// TODO: Use this method properly
fn add_main_menu(ui: &Ui) {
    if let Some(main_menu_bar_token) = ui.begin_main_menu_bar() {
        if let Some(file_menu_token) = ui.begin_menu("File") {
            if MenuItem::new("New").build(ui) {
                // TODO: Implement new call
            }
            if MenuItem::new("Open").build(ui) {
                // TODO: Implement open saved data
            }
            if MenuItem::new("Save").build(ui) {
                // TODO: Implement save data, same as "Save as" if no file to save is specified
            }
            if MenuItem::new("Save as").build(ui) {
                // TODO: Implement save data as file (specify file)
            }
            file_menu_token.end();
        }
        main_menu_bar_token.end();
    }
}

#[derive(Clone, Copy)]
pub enum ProgramStage {
    StartScreenStage,
    NewScreenStage,
    CurrentErgViewStage,
    AddNextGamesStage,
}

pub struct CompetitionData {
    pub name: String,
    pub date_string: String,
    pub place: String,
    pub executor: String,
    pub organizer: String,
    pub count_teams: u32,
    pub team_distribution: [u32; 2], // count_groups x count_teams_per_group
    pub teams: Option<Vec<Vec<Team>>>, // for each group a vector of teams, ordered by ids
    pub group_names: Option<Vec<String>>, // a vector of the group names, ordered by id
}

impl CompetitionData {
    pub fn empty() -> CompetitionData {
        CompetitionData {
            name: String::from(""),
            date_string: String::from(""),
            place: String::from(""),
            executor: String::from(""),
            organizer: String::from(""),
            count_teams: 0,
            team_distribution: [0, 0],
            teams: None,
            group_names: None,
        }
    }
}

pub struct Team {
    pub name: String,
    pub player_names: [Option<String>; 6], // maximal 6 possible players per team
}

pub struct ProgramState {
    pub stage: ProgramStage,
    pub size: [f32; 2],
    pub competition_data: Option<CompetitionData>,
    pub new_screen_state: Option<NewScreenState>,
}

impl ProgramState {
    pub fn new(stage: ProgramStage, size: [f32; 2]) -> ProgramState {
        ProgramState {
            stage,
            size,
            competition_data: None,
            new_screen_state: None,
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
                if self.competition_data.is_none() {
                    self.competition_data = Some(CompetitionData::empty());
                }
            }

            ProgramStage::CurrentErgViewStage => {
                self.new_screen_state = None;
                // TODO: Add more state resets if needed
            }
            ProgramStage::AddNextGamesStage => todo!(),
        }
        self.stage = new_stage;
    }
}
