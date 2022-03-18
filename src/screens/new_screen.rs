use imgui::{ChildWindow, Selectable, StyleColor, Ui};

use crate::{CompetitionData, ProgramState};

use super::my_input_text::MyTextInput;

pub fn build(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    if program_state.new_screen_state.is_none() {
        program_state.new_screen_state = Some(NewScreenState {
            stage: NewScreenStage::Start,
            submit_failure_msg: None,
        });
    }
    match program_state.new_screen_state.unwrap().stage {
        NewScreenStage::Start => build_init_stage(ui, program_state, menu_bar_height),
        NewScreenStage::Teams => build_teams_stage(ui, program_state, menu_bar_height),
    }
}

fn build_init_stage(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    let child_bg_color = ui.push_style_color(StyleColor::ChildBg, [0.0, 0.0, 0.0, 1.0]);
    let window_bg_color = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]);

    ChildWindow::new("Main screen")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .build(ui, || {
            // init competition data if not already done
            if program_state.competition_data.is_none() {
                program_state.competition_data = Some(CompetitionData::empty());
            }

            // init references and data structures
            let new_screen_state = program_state.new_screen_state.as_mut().unwrap();
            let data = program_state.competition_data.as_mut().unwrap();

            let labels = [
                "Competition Name:",
                "Date:",
                "Place:",
                "Executor:",
                "Organizer:",
                "Count teams:",
                "Team distribution:",
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

            // TODO: Move the focus to the next input on TAB or ESC
            // draw InputText boxes and check wether changes happen
            let mut anything_changed = false;
            my_input_boxes.iter_mut().for_each(|b| {
                let changed = b.build(ui, max_label_size);
                anything_changed = anything_changed || changed;
                // ui.set_keyboard_focus_here();
            });

            // draw count teams integer input box
            let mut count_teams_helper = data.count_teams as i32;
            ui.text(labels[5]);
            ui.same_line_with_pos(max_label_size + 20.0);
            ui.input_int("##count_teams", &mut count_teams_helper)
                .build();

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
            if let Some(token) = ui.begin_combo("##group_selection", &mut team_distribution) {
                group_possibilities.iter().for_each(|[g, t]| {
                    if Selectable::new(format!("{}x{}", *g, *t)).build(ui) {
                        data.team_distribution = [*g, *t];
                        anything_changed = true;
                    }
                });

                token.end();
            }

            // reset submit failure message
            if anything_changed && new_screen_state.submit_failure_msg.is_some() {
                new_screen_state.submit_failure_msg = None;
            }

            // draw submit button and check for valid inputs, possibly set failure message
            if ui.button("Submit") {
                if let Some(err_msg) = check_valid_inputs(data, NewScreenStage::Start) {
                    new_screen_state.submit_failure_msg = Some(err_msg);
                } else {
                    new_screen_state.stage = NewScreenStage::Teams;
                    new_screen_state.submit_failure_msg = None;
                }
            }

            // draw submit failure message
            if let Some(msg) = new_screen_state.submit_failure_msg {
                ui.same_line_with_pos(max_label_size + 20.0);
                ui.text(msg);
            }
        });
    window_bg_color.pop();
    child_bg_color.pop();
}

fn build_teams_stage(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    let child_bg_color = ui.push_style_color(StyleColor::ChildBg, [0.0, 0.0, 0.0, 1.0]);
    let window_bg_color = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]);

    ChildWindow::new("Main screen")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .build(ui, || {
            ui.indent_by(10.0);

            // TODO: Implement back button

            // Write headline
            ui.new_line();
            ui.set_window_font_scale(2.0);
            ui.text("Enter group and team names:");
            ui.set_window_font_scale(1.0);

            let state = program_state.new_screen_state.as_mut().unwrap();
            let data = program_state.competition_data.as_mut().unwrap();

            // init team names vector if not yet done
            if data.team_names.is_none() {
                data.team_names = Some(
                    (1..=data.team_distribution[0])
                        .map(|_| {
                            (1..=data.team_distribution[1])
                                .map(|_| String::from(""))
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

            // create tab bar for all groups and add text input boxes for setting group and team names
            if let Some(team_names) = data.team_names.as_mut() {
                if let Some(group_names) = data.group_names.as_mut() {
                    if let Some(tab_bar_token) = ui.tab_bar("Choose the group:") {
                        for group_idx in 1..=data.team_distribution[0] {
                            // TODO: Find a way to dynamically change tab item name, but keep focus on input text
                            if let Some(tab_item_token) = ui.tab_item(format!("Group {group_idx}"))
                            {
                                // draw input text box for group name
                                MyTextInput::new(
                                    "Group Name:",
                                    "Enter a specific name for this group",
                                    group_names.get_mut((group_idx - 1) as usize).unwrap(),
                                )
                                .build(ui, max_label_size);

                                let team_names_for_group =
                                    team_names.get_mut((group_idx - 1) as usize).unwrap();

                                // draw input text boxes for team names
                                for team_idx in 1..=data.team_distribution[1] {
                                    MyTextInput::new(
                                        format!("Team {team_idx}").as_str(),
                                        "",
                                        team_names_for_group
                                            .get_mut((team_idx - 1) as usize)
                                            .unwrap(),
                                    )
                                    .build(ui, max_label_size);
                                }
                                tab_item_token.end();
                            }
                        }
                        tab_bar_token.end();
                    }
                }
            }

            ui.separator();

            if ui.button("Submit") {}
        });

    window_bg_color.end();
    child_bg_color.end();
}

fn calc_group_possibilities(count_teams: u32) -> Vec<[u32; 2]> {
    if count_teams == 0 {
        Vec::new()
    } else {
        let mut possibilities = vec![[1, count_teams]];
        for group_count in 2..count_teams {
            if count_teams % group_count == 0 {
                possibilities.push([group_count, count_teams / group_count]);
            }
        }

        possibilities
    }
}

fn check_valid_inputs<'a>(data: &CompetitionData, stage: NewScreenStage) -> Option<&'a str> {
    match stage {
        NewScreenStage::Start => {
            if data.name == "" {
                Some("Enter a name for the competition!")
            } else if data.date_string == "" {
                Some("Enter a date when the competition takes place!")
            } else if data.place == "" {
                Some("Enter a place where the competition takes place!")
            } else if data.executor == "" {
                // TODO: Make an executor optional?
                Some("Enter an executor of the competition!")
            } else if data.organizer == "" {
                // TODO: Make an organizer optional?
                Some("Enter an organizer of the competition!")
            } else if data.count_teams < 2 {
                Some("A competition needs at least 2 teams!")
            } else if data.team_distribution == [0, 0] {
                Some("Choose a team distribution!")
            } else {
                None
            }
        }
        _ => {
            panic!("This new screens stage needs to be handled!")
        }
    }
}

#[derive(Clone, Copy)]
pub enum NewScreenStage {
    Start,
    Teams,
}

#[derive(Clone, Copy)]
pub struct NewScreenState<'a> {
    stage: NewScreenStage,
    submit_failure_msg: Option<&'a str>,
}
