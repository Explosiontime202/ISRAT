use imgui::{
    ChildWindow, Id, StyleColor, TableColumnFlags, TableColumnSetup, TableFlags, TableRowFlags, Ui,
};

use crate::{
    data::{CompetitionData, InterimResultEntry, Match, MatchResult, Team},
    ProgramState,
};

pub fn build(ui: &Ui, program_state: &mut ProgramState, menu_bar_height: f32) {
    let child_bg_color = ui.push_style_color(StyleColor::ChildBg, [0.0, 0.0, 0.0, 1.0]);
    let window_bg_color = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]);
    ChildWindow::new("##erg_view_screen")
        .size([
            program_state.size[0],
            program_state.size[1] - menu_bar_height,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .build(ui, || {
            assert!(program_state.competition_data.is_some());
            let data = program_state.competition_data.as_mut().unwrap();

            assert!(data.current_interim_result.len() == data.team_distribution[0] as usize);

            if let Some(_tab_bar_token) = ui.tab_bar("##group_selector") {
                for (idx, group_name) in data.group_names.as_ref().unwrap().iter().enumerate() {
                    if let Some(_tab_item_token) = ui.tab_item(group_name) {
                        let erg_screen_state = program_state.erg_screen_state.as_mut().unwrap();
                        // calculate interim result if not available
                        if data.current_interim_result[idx].is_none() {
                            data.current_interim_result[idx] =
                                Some(data.calc_interim_result_for_group(idx));
                        }

                        draw_erg_table(ui, data, idx);

                        ui.new_line();
                        ui.new_line();
                        ui.new_line();
                        ui.separator();
                        ui.new_line();

                        draw_upcoming_matches(
                            ui,
                            &mut data.matches[idx],
                            erg_screen_state,
                            idx,
                            &data.teams.as_ref().unwrap()[idx],
                            data.current_batch[idx],
                            data.team_distribution[1] / 2,
                        );

                        draw_submit_button(
                            ui,
                            erg_screen_state,
                            &mut data.matches[idx],
                            &mut data.current_batch[idx],
                            &mut data.current_interim_result[idx],
                            idx,
                        );
                    }
                }
            }
        });

    window_bg_color.end();
    child_bg_color.end();
}

fn draw_erg_table(ui: &Ui, data: &CompetitionData, group_idx: usize) {
    let column_widths = [
        ui.calc_text_size("Place")[0] * 1.5,
        0.0,
        ui.calc_text_size("999 : 999")[0] * 2.0,
        ui.calc_text_size("99.999")[0] * 2.0,
        ui.calc_text_size("9999 : 9999")[0] * 2.0,
    ];

    if let Some(_table_token) = ui.begin_table_with_flags(
        "##erg_table",
        5,
        TableFlags::BORDERS | TableFlags::SIZING_FIXED_FIT,
    ) {
        // add the columns
        ui.table_setup_column_with(TableColumnSetup {
            name: "Place",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: column_widths[0],
            user_id: Id::Int(0),
        });

        ui.table_setup_column_with(TableColumnSetup {
            name: "Team",
            flags: TableColumnFlags::WIDTH_STRETCH,
            init_width_or_weight: 0.0,
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Points",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: column_widths[2],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Quotient",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: column_widths[3],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Stock Points",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: column_widths[4],
            user_id: Id::Int(0),
        });

        // add header row and center the headers
        ui.table_next_row_with_flags(TableRowFlags::HEADERS);

        ui.table_next_column();
        center(ui, "Place");

        ui.table_next_column();
        center(ui, "Team");

        ui.table_next_column();
        center(ui, "Points");

        ui.table_next_column();
        center(ui, "Quotient");

        ui.table_next_column();
        center(ui, "Stock Points");

        // draw the rows and center the entries
        data.current_interim_result[group_idx]
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(place_idx, entry)| {
                ui.table_next_row();

                ui.table_next_column();
                center(ui, (place_idx + 1).to_string());

                ui.table_next_column();
                center(
                    ui,
                    &data.teams.as_ref().unwrap()[group_idx][entry.team_idx].name,
                );

                ui.table_next_column();
                center(
                    ui,
                    format!("{} : {}", entry.match_points[0], entry.match_points[1]),
                );

                ui.table_next_column();
                center(ui, format!("{:.3}", entry.quotient));

                ui.table_next_column();
                center(
                    ui,
                    format!("{} : {}", entry.stock_points[0], entry.stock_points[1]),
                );
            });
    }
}

fn draw_upcoming_matches(
    ui: &Ui,
    matches: &mut Vec<Match>,
    erg_screen_state: &mut ErgScreenState,
    group_idx: usize,
    teams: &Vec<Team>,
    current_batch: u32,
    count_lanes: u32,
) {
    center(ui, "Next Matches:");
    ui.new_line();

    // setup table for upcoming matches and to enter the results
    if let Some(_table_token) =
        ui.begin_table_with_flags("##upcoming_matches_table", 4, TableFlags::BORDERS)
    {
        // setup up columns
        ui.table_setup_column_with(TableColumnSetup {
            name: "##Lane",
            flags: TableColumnFlags::WIDTH_STRETCH,
            init_width_or_weight: 1.5,
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##TeamA",
            flags: TableColumnFlags::WIDTH_STRETCH,
            init_width_or_weight: 3.0,
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##TeamB",
            flags: TableColumnFlags::WIDTH_STRETCH,
            init_width_or_weight: 3.0,
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##MatchResult",
            flags: TableColumnFlags::WIDTH_STRETCH,
            init_width_or_weight: 2.0,
            user_id: Id::Int(0),
        });

        // draw upcoming matches for each lane
        (0..count_lanes).for_each(|lane_idx| {
            ui.table_next_row();
            ui.table_next_column();

            center(ui, format!("Lane {}", (lane_idx + 1)));

            ui.table_next_column();

            // check if some match on this lane exists ...
            if let Some(_match) = matches
                .iter()
                .find(|&_match| _match.batch == current_batch && _match.lane == lane_idx)
            {
                assert!(_match.result == MatchResult::NotPlayed);

                // draw team names
                let team_a_name = &teams[_match.team_a].name;
                center(ui, team_a_name);
                ui.table_next_column();
                let team_b_name = &teams[_match.team_b].name;
                center(ui, team_b_name);
                ui.table_next_column();

                // initialize points string, for each team the string representation of the entered points, empty string if no points were entered
                let mut points_str = if let Some(intermediate_result) = erg_screen_state
                    .intermediate_results[group_idx]
                    .iter()
                    .find(|&i_res| i_res.lane_idx == lane_idx)
                {
                    intermediate_result.result.map(|points_opt| {
                        if let Some(points) = points_opt {
                            points.to_string()
                        } else {
                            String::from("")
                        }
                    })
                } else {
                    [String::from(""), String::from("")]
                };

                // function to store the entered strings (points) as integer options in the state
                let mut save_results = |point_a_str: &String, point_b_str: &String| {
                    if let Some(intermediate_result) = erg_screen_state.intermediate_results
                        [group_idx]
                        .iter_mut()
                        .find(|i_res| i_res.lane_idx == lane_idx)
                    {
                        intermediate_result.result =
                            [point_a_str, point_b_str].map(|point_str| match point_str.parse() {
                                Ok(result) => Some(result),
                                Err(_) => None,
                            });
                    } else {
                        erg_screen_state.intermediate_results[group_idx].push(IntermediateResult {
                            lane_idx,
                            result: [point_a_str, point_b_str].map(|point_str| {
                                match point_str.parse() {
                                    Ok(result) => Some(result),
                                    Err(_) => None,
                                }
                            }),
                        });
                    }
                };

                // align and draw text input fields to enter the points
                let available_space = ui.content_region_avail()[0];
                let _token = ui.push_item_width(available_space * 0.4);
                if ui
                    .input_text(format!("##result_{lane_idx}_team_a"), &mut points_str[0])
                    .chars_decimal(true)
                    .chars_hexadecimal(false)
                    .chars_noblank(true)
                    .chars_uppercase(false)
                    .build()
                {
                    save_results(&points_str[0], &points_str[1]);
                }
                let text_width = ui.calc_text_size(":")[0];
                ui.same_line_with_pos(available_space * 0.5 - text_width / 2.0);
                ui.text(":");
                ui.same_line_with_pos(available_space * 0.6);
                if ui
                    .input_text(format!("##result_{lane_idx}_team_b"), &mut points_str[1])
                    .chars_decimal(true)
                    .chars_hexadecimal(false)
                    .chars_noblank(true)
                    .chars_uppercase(false)
                    .build()
                {
                    save_results(&points_str[0], &points_str[1]);
                }
            } else {
                // ... else display that there is no match on this lane
                center(ui, "Empty");
                ui.table_next_column();
                center(ui, "Empty");
                ui.table_next_column();
            }
        });
    }
}

fn draw_submit_button(
    ui: &Ui,
    erg_screen_state: &mut ErgScreenState,
    matches: &mut Vec<Match>,
    current_batch: &mut u32,
    current_interim_result: &mut Option<Vec<InterimResultEntry>>,
    group_idx: usize,
) {
    // align submit button right, with some indent and draw it
    ui.set_cursor_pos([
        ui.cursor_pos()[0] + ui.content_region_avail()[0] - ui.calc_text_size("Submit")[0] - 20.0,
        ui.cursor_pos()[1],
    ]);
    if ui.button("Submit") {
        // check for valid inputs
        erg_screen_state.failure_msg = matches
            .iter()
            .filter(|_match| _match.batch == *current_batch)
            .map(|_match| {
                match erg_screen_state.intermediate_results[group_idx]
                    .iter()
                    .find(|i_res| i_res.lane_idx == _match.lane)
                {
                    Some(i_res) if i_res.result.iter().all(|point_opt| point_opt.is_some()) => None,
                    _ => Some(String::from("Please enter results for each match played.")),
                }
            })
            .fold(None, |a, b| if a.is_some() { a } else { b });

        // only submit entered results if input is valid
        if erg_screen_state.failure_msg.is_some() {
            return;
        }

        // process entered results, i.e. transfer the entered points from the intermediate_results in the state to the matches in the competition data
        for _match in matches
            .iter_mut()
            .filter(|_match| _match.batch == *current_batch)
        {
            let i_res = erg_screen_state.intermediate_results[group_idx]
                .iter()
                .find(|i_res| i_res.lane_idx == _match.lane)
                .unwrap();

            _match.points = Some(i_res.result.map(|point_opt| point_opt.unwrap()));
            _match.result = match _match.points.unwrap()[0].cmp(&_match.points.unwrap()[1]) {
                std::cmp::Ordering::Less => MatchResult::WinnerB,
                std::cmp::Ordering::Equal => MatchResult::Draw,
                std::cmp::Ordering::Greater => MatchResult::WinnerA,
            };
        }

        erg_screen_state.intermediate_results[group_idx].clear();
        *current_interim_result = None;
        *current_batch += 1;
    }

    // display failure message if some exists
    if let Some(failure_msg) = erg_screen_state.failure_msg.as_ref() {
        ui.same_line();
        ui.set_cursor_pos([
            ui.cursor_pos()[0] + ui.content_region_avail()[0]
                - ui.calc_text_size("Submit")[0]
                - ui.calc_text_size(failure_msg)[0]
                - 40.0,
            ui.cursor_pos()[1],
        ]);
        ui.text(failure_msg);
    }
}

// Helper

fn center<T: AsRef<str>>(ui: &Ui, text: T) {
    ui.set_cursor_pos([
        ui.cursor_pos()[0] + (ui.content_region_avail()[0] - ui.calc_text_size(&text)[0]) / 2.0,
        ui.cursor_pos()[1],
    ]);
    ui.text(&text);
}

pub struct ErgScreenState {
    intermediate_results: Vec<Vec<IntermediateResult>>, // for each group a vector of entered, but not submitted match results for the current batch
    failure_msg: Option<String>,
}

impl ErgScreenState {
    pub fn new(group_count: usize) -> Self {
        Self {
            intermediate_results: (0..group_count).map(|_| vec![]).collect(),
            failure_msg: None,
        }
    }
}

struct IntermediateResult {
    result: [Option<i32>; 2],
    lane_idx: u32,
}
