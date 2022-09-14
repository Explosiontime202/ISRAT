use imgui::{ChildWindow, Selectable, StyleColor, Ui};

use crate::{
    data::calc_group_possibilities, screens::buttons, CompetitionData, ProgramStage, ProgramState,
    Team,
};

use super::my_input_text::{MyMultilineTextInput, MyTextInput};

pub fn build(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    debug_assert!(program_state.new_screen_state.is_some());
    debug_assert!(program_state.competition.data.is_some());

    let child_bg_color = ui.push_style_color(StyleColor::ChildBg, [0.0, 0.0, 0.0, 1.0]);
    let window_bg_color = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]);

    match program_state.new_screen_state.as_ref().unwrap().stage {
        NewScreenStage::GeneralInfo => build_init_stage(ui, program_state, menu_bar_height),
        NewScreenStage::TeamNames => build_teams_stage(ui, program_state, menu_bar_height),
        NewScreenStage::PlayerNames => build_player_names(ui, program_state, menu_bar_height),
    }

    window_bg_color.end();
    child_bg_color.end();
}

pub fn bottom_buttons(ui: &Ui, program_state: &mut ProgramState) {
    assert!(program_state.new_screen_state.is_some());
    let new_screen_state = program_state.new_screen_state.as_mut().unwrap();

    if ui.button("New") {
        ui.open_popup("##restart_competition_setup_wizard");
        new_screen_state.restart_popup = true;
    }

    if new_screen_state.restart_popup {
        if let Some(_popup_token) = ui.begin_popup("##restart_competition_setup_wizard") {
            ui.text("This will delete all entered information");
            ui.text("in this wizard to setup a new tournament.");
            ui.text("You will be redirected to the start of the setup wizard.");
            if ui.button("No") {
                new_screen_state.restart_popup = false;
                ui.close_current_popup();
            }
            ui.same_line();
            if ui.button("Yes") {
                // override state, deletes all information
                *new_screen_state = NewScreenState::new();
                program_state.competition.data = Some(CompetitionData::empty());
                ui.close_current_popup();
            }
        } else {
            new_screen_state.restart_popup = false;
            ui.close_current_popup();
        }
    }

    buttons::open_button(ui, program_state);
}

// builder for different stages

fn build_init_stage(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    ChildWindow::new("##init_stage_window")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .build(ui, || {
            // init references and data structures
            let new_screen_state = program_state.new_screen_state.as_mut().unwrap();
            let data = program_state.competition.data.as_mut().unwrap();

            let text_input_width = 2.0 / 3.0 * program_state.size[0];

            let labels = [
                "Competition Name:",
                "Date:",
                "Place:",
                "Executor:",
                "Organizer:",
                "Count teams:",
                "Team distribution:",
                "Referee:",
                "Competition Manager:",
                "Clerk:",
                "Additional text:"
            ];

            let mut my_input_boxes = [
                MyTextInput::new(
                    labels[0],
                    "Enter the name of the competition",
                    &mut data.name,
                ),
                MyTextInput::new(
                    labels[1],
                    "Enter the date of the competition",
                    &mut data.date_string,
                ),
                MyTextInput::new(
                    labels[2],
                    "Enter the place of the competition",
                    &mut data.place,
                ),
                MyTextInput::new(
                    labels[3],
                    "Enter the executor of the competition",
                    &mut data.executor,
                ),
                MyTextInput::new(
                    labels[4],
                    "Enter the organzier of the competition",
                    &mut data.organizer,
                ),
                MyTextInput::new(
                    labels[7],
                    "Enter the referee of the competition, leave empty if none should be displayed.",
                    &mut data.referee
                ),
                MyTextInput::new(
                    labels[8],
                    "Enter the manager of the competition, leave empty if none should be displayed.",
                    &mut data.competition_manager,
                ),
                MyTextInput::new(
                    labels[9],
                    "Enter the clerk of the competition, leave empty if none should be displayed.",
                    &mut data.clerk
                )
            ];

            // find the maximal length of a label used
            let max_label_size = labels
                .iter()
                .map(|s| ui.calc_text_size(s)[0])
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            ui.indent_by(10.0);

            // Write headline
            ui.new_line();
            ui.set_window_font_scale(2.0);
            ui.text("Create new competition");
            ui.set_window_font_scale(1.0);

            let mut anything_changed = false;
            {
                let width_token = ui.push_item_width(text_input_width);
                // TODO: Move the focus to the next input on TAB or ESC
                // draw InputText boxes and check wether changes happen

                my_input_boxes.iter_mut().for_each(|b| {
                    let changed = b.build(ui, max_label_size);
                    anything_changed = anything_changed || changed;
                    // ui.set_keyboard_focus_here();
                });

                width_token.pop(ui);
            }

            // draw count teams integer input box
            let mut count_teams_helper = data.count_teams as i32;
            ui.text(labels[5]);
            ui.same_line_with_pos(max_label_size + 20.0);
            {
                let width_token = ui.push_item_width(text_input_width);
                ui.input_int("##count_teams", &mut count_teams_helper)
                .build();
                width_token.pop(ui);
            }

            // store data and check for changes or negative inputs
            if count_teams_helper < 0 {
                data.count_teams = 0;
                anything_changed = true;
            } else {
                if data.count_teams != count_teams_helper as u32 {
                    data.count_teams = count_teams_helper as u32;
                    // reset team distribution when team count changes
                    data.team_distribution = [0, 0];
                    anything_changed = true;
                }
            };

            // generate current team distribution string
            let mut team_distribution = if data.team_distribution[0] == 0 {
                String::from("")
            } else {
                format!(
                    "{}x{}",
                    data.team_distribution[0], data.team_distribution[1]
                )
            };

            let group_possibilities = calc_group_possibilities(data.count_teams);

            // draw drop down menu for team distribution and check for changes
            ui.text(labels[6]);
            ui.same_line_with_pos(max_label_size + 20.0);
            {
                let width_token = ui.push_item_width(text_input_width);
                if let Some(_combo_token) = ui.begin_combo("##group_selection", &mut team_distribution)
                {
                    group_possibilities.iter().for_each(|[g, t]| {
                        if Selectable::new(format!("{}x{}", *g, *t)).build(ui) {
                            data.team_distribution = [*g, *t];
                            anything_changed = true;
                        }
                    });
                }
                width_token.pop(ui);
            }

            if data.team_distribution[1] != 0 && data.team_distribution[1] % 2 == 0 {
                ui.same_line();
                ui.checkbox("With Breaks", &mut data.with_break);
            }

            {
                let width_token = ui.push_item_width(text_input_width);
                MyMultilineTextInput::new(labels[10], &mut data.additional_text).build(ui, max_label_size, [text_input_width,ui.current_font_size() * 4.0]);
                width_token.pop(ui);
            }

            // reset submit failure message
            if anything_changed && new_screen_state.submit_failure_msg.is_some() {
                new_screen_state.submit_failure_msg = None;
            }

            // draw submit button and check for valid inputs, possibly set failure message
            if ui.button("Submit") {
                if let Some(err_msg) = check_valid_inputs(data, NewScreenStage::GeneralInfo) {
                    new_screen_state.submit_failure_msg = Some(err_msg);
                } else {
                    new_screen_state.go_to_stage(NewScreenStage::TeamNames, data);
                }
            }

            // draw reset button
            ui.same_line();
            if ui.button("Reset") {
                ui.open_popup("##reset_popup");
                new_screen_state.reset_popup = true;
            }

            if new_screen_state.reset_popup {
                ui.popup_modal("##reset_popup")
                    .resizable(false)
                    .movable(false)
                    .scrollable(false)
                    .build(ui, || {
                        ui.text("Are you sure you want to reset all information?");
                        ui.text("This will delete all your entered information!");
                        if ui.button("Yes") {
                            new_screen_state.reset_stage(data);
                            ui.close_current_popup();
                        }
                        ui.same_line();
                        if ui.button("No") {
                            new_screen_state.reset_popup = false;
                            ui.close_current_popup();
                        }
                    });
            }

            // draw submit failure message
            if let Some(msg) = &new_screen_state.submit_failure_msg {
                ui.same_line();
                ui.text(msg);
            }
        });
}

fn build_teams_stage(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    ChildWindow::new("##teams_stage_window")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .build(ui, || {
            ui.indent_by(10.0);

            // Write headline
            ui.new_line();
            ui.set_window_font_scale(2.0);
            ui.text("Enter group and team names");
            ui.set_window_font_scale(1.0);

            let new_screen_state = program_state.new_screen_state.as_mut().unwrap();
            let data = program_state.competition.data.as_mut().unwrap();

            // init team names vector if not yet done
            if data.teams.is_none() {
                data.teams = Some(
                    (1..=data.team_distribution[0])
                        .map(|_| {
                            (1..=data.team_distribution[1])
                                .map(|_| Team {
                                    name: String::from(""),
                                    region: String::from(""),
                                    player_names: [None, None, None, None, None, None],
                                })
                                .collect()
                        })
                        .collect(),
                );
            }

            // init group names vector if not yet done
            if data.group_names.is_none() {
                data.group_names = Some(
                    (1..=data.team_distribution[0])
                        .map(|group_idx| format!("Group {group_idx}"))
                        .collect(),
                );
            }

            // calculate max label size: either "Group Name" or the "Team {}" with the greatest team number
            let max_label_size = ui.calc_text_size("Group Name")[0]
                .max(ui.calc_text_size(format!("Team {}", data.team_distribution[1]))[0]);

            let name_text_input_width = program_state.size[0] / 2.0;
            let region_label_size = ui.calc_text_size("Region:")[0];
            let region_text_input_width = program_state.size[0] / 5.0;
            let region_label_offset = name_text_input_width + max_label_size + 20.0;

            // create tab bar for all groups and add text input boxes for setting group and team names
            if let Some(teams) = data.teams.as_mut() {
                if let Some(group_names) = data.group_names.as_mut() {
                    if let Some(_tab_bar_token) = ui.tab_bar("Choose the group:") {
                        for group_idx in 1..=data.team_distribution[0] {
                            let item_width_token = ui.push_item_width(name_text_input_width);
                            // TODO: Find a way to dynamically change tab item name, but keep focus on input text
                            if let Some(_tab_item_token) = ui.tab_item(format!("Group {group_idx}"))
                            {
                                // draw input text box for group name
                                MyTextInput::new(
                                    "Group Name:",
                                    "",
                                    group_names.get_mut((group_idx - 1) as usize).unwrap(),
                                )
                                .build(ui, max_label_size);

                                let teams_for_group =
                                    teams.get_mut((group_idx - 1) as usize).unwrap();

                                // draw input text boxes for team names and team region
                                for team_idx in 1..=data.team_distribution[1] {
                                    MyTextInput::new(
                                        format!("Team {team_idx}").as_str(),
                                        "Enter team name, must not be empty.",
                                        &mut teams_for_group
                                            .get_mut((team_idx - 1) as usize)
                                            .unwrap()
                                            .name,
                                    )
                                    .build(ui, max_label_size);

                                    ui.same_line();

                                    {
                                        let item_width_token =
                                            ui.push_item_width(region_text_input_width);
                                        MyTextInput::new(
                                            "Region:",
                                            "Enter region, can be empty.",
                                            &mut teams_for_group
                                                .get_mut((team_idx - 1) as usize)
                                                .unwrap()
                                                .region,
                                        )
                                        .offset(region_label_offset)
                                        .text_input_label(format!("##team_{team_idx}_region"))
                                        .build(ui, region_label_size);
                                        item_width_token.pop(ui);
                                    }
                                }
                            }
                            item_width_token.pop(ui);
                        }
                    }
                }
            }

            ui.separator();

            if ui.button("Submit") {
                if let Some(failure_msg) = check_valid_inputs(data, new_screen_state.stage) {
                    new_screen_state.submit_failure_msg = Some(failure_msg);
                } else {
                    if data.matches.is_empty() {
                        data.generate_matches();
                    }
                    new_screen_state.go_to_stage(NewScreenStage::PlayerNames, data);
                }
            }

            ui.same_line();
            if ui.button("Reset") {
                ui.open_popup("##reset_popup");
                new_screen_state.reset_popup = true;
            }

            ui.same_line();
            if ui.button("Go back") {
                ui.open_popup("##go_back_popup");
                new_screen_state.go_back_popup = true;
            }

            if new_screen_state.reset_popup {
                ui.popup_modal("##reset_popup")
                    .resizable(false)
                    .movable(false)
                    .scrollable(false)
                    .build(ui, || {
                        ui.text("Are you sure you want to reset all team and group names?");
                        ui.text("This will delete all your entered group and team names!");
                        if ui.button("Yes") {
                            new_screen_state.reset_stage(data);
                            ui.close_current_popup();
                        }
                        ui.same_line();
                        if ui.button("No") {
                            new_screen_state.reset_popup = false;
                            ui.close_current_popup();
                        }
                    });
            }

            if new_screen_state.go_back_popup {
                ui.popup_modal("##go_back_popup")
                    .resizable(false)
                    .movable(false)
                    .scrollable(false)
                    .build(ui, || {
                        ui.text("Are you sure you want to go back to the previous step?");
                        ui.text("This will delete all your entered group and team names!");
                        if ui.button("Yes") {
                            new_screen_state.go_to_stage(NewScreenStage::GeneralInfo, data);
                            ui.close_current_popup();
                        }
                        ui.same_line();
                        if ui.button("No") {
                            new_screen_state.go_back_popup = false;
                            ui.close_current_popup();
                        }
                    });
            }

            // draw submit failure message
            if let Some(msg) = &new_screen_state.submit_failure_msg {
                ui.same_line();
                ui.text(msg);
            }
        });
}

fn build_player_names(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    // Submit, Skip, Reset Button

    ChildWindow::new("##player_names_stage_window")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .build(ui, || {
            ui.indent_by(10.0);

            // Write headline
            ui.new_line();
            ui.set_window_font_scale(2.0);
            ui.text("Enter player names:");
            ui.set_window_font_scale(1.0);

            {
                let new_screen_state = program_state.new_screen_state.as_mut().unwrap();
                let data = program_state.competition.data.as_mut().unwrap();
                assert!(data.teams.is_some());

                // get all teams sorted
                let mut sorted_teams: Vec<&mut Team> = data
                    .teams
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .flat_map(|v| v.iter_mut())
                    .collect();

                sorted_teams.sort_by(|a, b| a.name.cmp(&b.name));

                let max_label_size = ui.calc_text_size(format!("Player 6"))[0];

                // draw selector for team selection
                ui.text("Team:");
                ui.same_line_with_pos(max_label_size + 20.0);

                if let Some(_combo_token) = ui.begin_combo(
                    "##team_selector",
                    if let Some(selected_idx) = new_screen_state.selected_team {
                        sorted_teams.get(selected_idx).unwrap().name.as_str()
                    } else {
                        "Select a team!"
                    },
                ) {
                    sorted_teams.iter_mut().enumerate().for_each(|(idx, team)| {
                        if Selectable::new(&team.name).build(ui) {
                            new_screen_state.selected_team = Some(idx);
                        }
                    });
                }

                // draw text input boxes for selected team, if any team is selected
                if let Some(selected_idx) = new_screen_state.selected_team {
                    sorted_teams
                        .get_mut(selected_idx)
                        .unwrap()
                        .player_names
                        .iter_mut()
                        .enumerate()
                        .for_each(|(idx, player)| {
                            if player.is_none() {
                                *player = Some(String::from(""));
                            }
                            MyTextInput::new(
                                format!("Player {}", idx + 1).as_str(),
                                "",
                                player.as_mut().unwrap(),
                            )
                            .build(ui, max_label_size);
                        });
                }
            }
            ui.separator();

            // add buttons to submit the data, to skip the screen,
            // to reset the filled fields and to go back to the last screen

            if ui.button("Submit") {
                {
                    let new_screen_state = program_state.new_screen_state.as_mut().unwrap();
                    let data = program_state.competition.data.as_mut().unwrap();
                    assert!(check_valid_inputs(data, new_screen_state.stage).is_none());
                }

                program_state.switch_to_stage(ProgramStage::CurrentErgViewStage);
                return;
            }

            ui.same_line();
            if ui.button("Skip") {
                program_state.switch_to_stage(ProgramStage::CurrentErgViewStage);
                return;
            }

            {
                let new_screen_state = program_state.new_screen_state.as_mut().unwrap();
                let data = program_state.competition.data.as_mut().unwrap();

                ui.same_line();
                if ui.button("Reset") {
                    ui.open_popup("##reset_popup");
                    new_screen_state.reset_popup = true;
                }

                ui.same_line();
                if ui.button("Go back") {
                    ui.open_popup("##go_back_popup");
                    new_screen_state.go_back_popup = true;
                }

                // create reset popup popup
                if new_screen_state.reset_popup {
                    ui.popup_modal("##reset_popup")
                        .resizable(false)
                        .movable(false)
                        .scrollable(false)
                        .build(ui, || {
                            ui.text("Are you sure you want to reset all player names?");
                            ui.text("This will delete all your entered player names!");

                            if ui.button("Yes") {
                                new_screen_state.reset_stage(data);
                                ui.close_current_popup();
                            }

                            ui.same_line();

                            if ui.button("No") {
                                new_screen_state.reset_popup = false;
                                ui.close_current_popup();
                            }
                        });
                }

                // create go back popup
                if new_screen_state.go_back_popup {
                    ui.popup_modal("##go_back_popup")
                        .resizable(false)
                        .movable(false)
                        .scrollable(false)
                        .build(ui, || {
                            ui.text("Are you sure you want to go back to the previous step?");
                            ui.text("This will delete all your entered player names!");

                            if ui.button("Yes") {
                                new_screen_state.go_to_stage(NewScreenStage::TeamNames, data);
                                ui.close_current_popup();
                            }

                            ui.same_line();

                            if ui.button("No") {
                                new_screen_state.go_back_popup = false;
                                ui.close_current_popup();
                            }
                        });
                }
            }
        });
}

// Helper

fn check_valid_inputs(data: &CompetitionData, stage: NewScreenStage) -> Option<String> {
    match stage {
        NewScreenStage::GeneralInfo => {
            if data.name == "" {
                Some("Enter a name for the competition!".to_string())
            } else if data.date_string == "" {
                Some("Enter a date when the competition takes place!".to_string())
            } else if data.place == "" {
                Some("Enter a place where the competition takes place!".to_string())
            } else if data.executor == "" {
                // TODO: Make an executor optional?
                Some("Enter an executor of the competition!".to_string())
            } else if data.organizer == "" {
                // TODO: Make an organizer optional?
                Some("Enter an organizer of the competition!".to_string())
            } else if data.count_teams < 2 {
                Some("A competition needs at least 2 teams!".to_string())
            } else if data.team_distribution == [0, 0] {
                Some("Choose a team distribution!".to_string())
            } else {
                None
            }
        }
        NewScreenStage::TeamNames => {
            // Check if every group and team name is non empty
            let mut ret_val = None;

            'outer_loop: for group_idx in 0..data.team_distribution[0] {
                // check group name
                if data
                    .group_names
                    .as_ref()
                    .unwrap()
                    .get(group_idx as usize)
                    .unwrap()
                    == ""
                {
                    ret_val = Some(format!("Enter non empty name for group {}!", group_idx + 1));
                    break;
                }

                // check team names
                for team_idx in 0..data.team_distribution[1] {
                    if data
                        .teams
                        .as_ref()
                        .unwrap()
                        .get(group_idx as usize)
                        .unwrap()
                        .get(team_idx as usize)
                        .unwrap()
                        .name
                        == ""
                    {
                        ret_val = Some(format!(
                            "Enter non empty name for team {} of group {}!",
                            team_idx + 1,
                            group_idx + 1
                        ));
                        break 'outer_loop;
                    }
                }
            }
            ret_val
        }
        NewScreenStage::PlayerNames => None,
        #[allow(unreachable_patterns)]
        // unreachable pattern, but for safety if enum is extended and this method is not adjusted
        _ => {
            todo!("This new screens stage needs to be handled!")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewScreenStage {
    GeneralInfo,
    TeamNames,
    PlayerNames,
}

impl NewScreenStage {
    pub fn previous(self) -> Option<Self> {
        match self {
            Self::GeneralInfo => None,
            Self::TeamNames => Some(Self::GeneralInfo),
            Self::PlayerNames => Some(Self::TeamNames),
        }
    }
}

pub struct NewScreenState {
    pub stage: NewScreenStage,
    pub submit_failure_msg: Option<String>,
    pub reset_popup: bool,
    pub go_back_popup: bool,
    pub restart_popup: bool,
    pub selected_team: Option<usize>,
}

impl NewScreenState {
    pub fn new() -> NewScreenState {
        NewScreenState {
            stage: NewScreenStage::GeneralInfo,
            submit_failure_msg: None,
            reset_popup: false,
            go_back_popup: false,
            restart_popup: false,
            selected_team: None,
        }
    }

    pub fn reset_stage(&mut self, data: &mut CompetitionData) {
        self.reset_stage_param(self.stage, data);
    }

    fn reset_common(&mut self) {
        self.submit_failure_msg = None;
        self.reset_popup = false;
        self.go_back_popup = false;
        self.selected_team = None;
    }

    fn reset_stage_param(&mut self, stage: NewScreenStage, data: &mut CompetitionData) {
        match stage {
            NewScreenStage::GeneralInfo => {
                self.reset_common();

                data.name = String::from("");
                data.date_string = String::from("");
                data.place = String::from("");
                data.executor = String::from("");
                data.organizer = String::from("");
                data.count_teams = 0;
                data.team_distribution = [0, 0];
                data.with_break = true;
            }
            NewScreenStage::TeamNames => {
                self.reset_common();
                data.teams = None;
                data.group_names = None;
            }
            NewScreenStage::PlayerNames => {
                self.reset_common();

                // delete player names
                data.teams.as_mut().unwrap().iter_mut().for_each(|group| {
                    group.iter_mut().for_each(|team| {
                        team.player_names = [None, None, None, None, None, None];
                    })
                });
            }
        }
    }

    pub fn go_to_stage(&mut self, new_stage: NewScreenStage, data: &mut CompetitionData) {
        if self.stage > new_stage {
            // go back
            {
                let mut cur_stage = self.stage;
                loop {
                    if cur_stage == new_stage {
                        break;
                    }
                    if let Some(prev_stage) = cur_stage.previous() {
                        self.reset_stage_param(cur_stage, data);
                        cur_stage = prev_stage;
                    }
                }
            }
        } else if self.stage < new_stage {
            // go forward
            self.reset_common();
        }

        self.stage = new_stage;
    }
}
