use crate::{
    colors::{BACKGROUND, TEXT},
    common::{center_text_around_cursor, center_x, padding_absolut_y, padding_relative},
    constants::{
        NEXT_GAME_TABLE_COLUMN_WIDTHS_SCALE, NEXT_GAME_TABLE_WIDTH_SCALE,
        RESULT_TABLE_COLUMN_WIDTHS_SCALE, RESULT_TABLE_PADDING_Y, RESULT_TABLE_WIDTH_SCALE,
        SELECTED_SCREEN_HEIGHT, SELECTED_SCREEN_PADDING, SELECTED_SCREEN_WIDTH,
    },
    data::{Competition, MatchResult},
    Fonts, ProgramStage, ProgramState,
};
use imgui::{
    ChildWindow, Id, StyleColor, TableColumnFlags, TableColumnSetup, TableFlags, TableRowFlags, Ui,
};

pub fn build(ui: &Ui, program_state: &mut ProgramState) {
    let text_token = ui.push_style_color(StyleColor::Text, TEXT);
    let bg_token = ui.push_style_color(StyleColor::ChildBg, BACKGROUND);
    ChildWindow::new("##group_overview")
        .size([
            program_state.size[0] * SELECTED_SCREEN_WIDTH,
            program_state.size[1] * SELECTED_SCREEN_HEIGHT,
        ])
        .no_nav()
        .bring_to_front_on_focus(false)
        .scroll_bar(true)
        .scrollable(true)
        .build(ui, || {
            assert!(program_state.competition.data.is_some());
            assert!(match program_state.stage {
                ProgramStage::GroupOverview(_) => true,
                _ => false,
            });
            let data = program_state.competition.data.as_ref().unwrap();
            let group_idx = program_state.stage.get_group_idx();

            padding_relative(ui, SELECTED_SCREEN_PADDING);

            let default_group_name = String::from(format!("Group {group_idx}"));
            let group_name = if data.group_names.is_some() {
                &data.group_names.as_ref().unwrap()[group_idx]
            } else {
                &default_group_name
            };

            let font_token = ui.push_font(program_state.fonts[Fonts::FontHeadline as usize]);
            let headline_str = format!(
                "{group_name} - Score after Batch {}",
                data.current_batch[group_idx]
            );
            ui.text(headline_str);
            font_token.pop();

            padding_relative(ui, [SELECTED_SCREEN_PADDING[0], 0.025]);
            draw_erg_table(ui, &mut program_state.competition, group_idx);
            ui.new_line();
            draw_upcoming_matches(ui, &mut program_state.competition, group_idx);
        });
    bg_token.pop();
    text_token.pop();
}

fn draw_erg_table(ui: &Ui, competition: &mut Competition, group_idx: usize) {
    // calculate interim result if not available
    if competition.current_interim_result[group_idx].is_none() {
        competition.current_interim_result[group_idx] = Some(
            competition
                .data
                .as_ref()
                .unwrap()
                .calc_interim_result_for_group(group_idx),
        );
    }

    let window_width_without_padding =
        ui.window_size()[0] * (1.0 - SELECTED_SCREEN_PADDING[0] * 2.0);

    if let Some(_table_token) = ui.begin_table_with_sizing(
        "##erg_table",
        5,
        TableFlags::BORDERS | TableFlags::SIZING_FIXED_FIT,
        [window_width_without_padding * RESULT_TABLE_WIDTH_SCALE, 0.0],
        window_width_without_padding * RESULT_TABLE_WIDTH_SCALE,
    ) {
        // add the columns
        ui.table_setup_column_with(TableColumnSetup {
            name: "Place",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding
                * RESULT_TABLE_COLUMN_WIDTHS_SCALE[0],
            user_id: Id::Int(0),
        });

        ui.table_setup_column_with(TableColumnSetup {
            name: "Team",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding
                * RESULT_TABLE_COLUMN_WIDTHS_SCALE[1],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Points",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding
                * RESULT_TABLE_COLUMN_WIDTHS_SCALE[2],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Quotient",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding
                * RESULT_TABLE_COLUMN_WIDTHS_SCALE[3],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Stock Points",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding
                * RESULT_TABLE_COLUMN_WIDTHS_SCALE[4],
            user_id: Id::Int(0),
        });

        // add header row and center the headers
        ui.table_next_row_with_height(
            TableRowFlags::HEADERS,
            ui.current_font_size() * (1.0 + RESULT_TABLE_PADDING_Y * 2.0),
        );

        let padding_height = ui.current_font_size() * RESULT_TABLE_PADDING_Y;

        ui.table_next_column();
        padding_absolut_y(ui, padding_height);
        center_x(ui, "Place");

        ui.table_next_column();
        padding_absolut_y(ui, padding_height);
        center_x(ui, "Team");

        ui.table_next_column();
        padding_absolut_y(ui, padding_height);
        center_x(ui, "Points");

        ui.table_next_column();
        padding_absolut_y(ui, padding_height);
        center_x(ui, "Quotient");

        ui.table_next_column();
        padding_absolut_y(ui, padding_height);
        center_x(ui, "Stock Points");

        // draw the rows and center the entries
        competition.current_interim_result[group_idx]
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(place_idx, entry)| {
                ui.table_next_row_with_height(
                    TableRowFlags::empty(),
                    ui.current_font_size() * (1.0 + RESULT_TABLE_PADDING_Y * 2.0),
                );

                ui.table_next_column();
                padding_absolut_y(ui, padding_height);
                center_x(ui, (place_idx + 1).to_string());

                ui.table_next_column();
                padding_absolut_y(ui, padding_height);
                center_x(
                    ui,
                    &competition.data.as_ref().unwrap().teams.as_ref().unwrap()[group_idx]
                        [entry.team_idx]
                        .name,
                );

                ui.table_next_column();
                padding_absolut_y(ui, padding_height);
                center_x(
                    ui,
                    format!("{} : {}", entry.match_points[0], entry.match_points[1]),
                );

                ui.table_next_column();
                padding_absolut_y(ui, padding_height);
                center_x(ui, format!("{:.3}", entry.quotient));

                ui.table_next_column();
                padding_absolut_y(ui, padding_height);
                center_x(
                    ui,
                    format!("{} : {}", entry.stock_points[0], entry.stock_points[1]),
                );
            });
    }
}

fn draw_upcoming_matches(ui: &Ui, competition: &mut Competition, group_idx: usize) {
    center_x(ui, "Next Matches:");
    ui.new_line();
    padding_relative(ui, [SELECTED_SCREEN_PADDING[0], 0.0]);

    // setup table for upcoming matches and to enter the results
    let window_width_without_padding =
        ui.window_size()[0] * (1.0 - SELECTED_SCREEN_PADDING[0] * 2.0);

    if let Some(_table_token) = ui.begin_table_with_sizing(
        "##upcoming_matches_table",
        3,
        TableFlags::BORDERS,
        [
            window_width_without_padding * NEXT_GAME_TABLE_WIDTH_SCALE,
            0.0,
        ],
        window_width_without_padding * NEXT_GAME_TABLE_WIDTH_SCALE,
    ) {
        // setup up columns
        ui.table_setup_column_with(TableColumnSetup {
            name: "##Lane",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding * NEXT_GAME_TABLE_COLUMN_WIDTHS_SCALE[0],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##TeamA",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding * NEXT_GAME_TABLE_COLUMN_WIDTHS_SCALE[1],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##TeamB",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: window_width_without_padding * NEXT_GAME_TABLE_COLUMN_WIDTHS_SCALE[2],
            user_id: Id::Int(0),
        });

        let data = competition.data.as_ref().unwrap();
        let count_lanes = data.team_distribution[1] / 2;
        let current_batch = data.current_batch[group_idx];
        let teams = &data.teams.as_ref().unwrap()[group_idx];

        // draw upcoming matches for each lane
        (0..count_lanes).for_each(|lane_idx| {
            ui.table_next_row();
            ui.table_next_column();

            center_x(ui, format!("Lane {}", (lane_idx + 1)));

            ui.table_next_column();

            // check if some match on this lane exists ...
            if let Some(_match) = data.matches[group_idx]
                .iter()
                .find(|&_match| _match.batch == current_batch && _match.lane == lane_idx)
            {
                debug_assert!(_match.result == MatchResult::NotPlayed);

                // draw team names
                let team_a_name = &teams[_match.team_a].name;
                center_x(ui, team_a_name);
                ui.table_next_column();
                let team_b_name = &teams[_match.team_b].name;
                center_x(ui, team_b_name);
            } else {
                // ... else display that there is no match on this lane
                center_x(ui, "Empty");
                ui.table_next_column();
                center_x(ui, "Empty");
                ui.table_next_column();
            }
        });
    }
}
