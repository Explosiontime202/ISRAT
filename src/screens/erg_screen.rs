use imgui::{
    ChildWindow, Id, StyleColor, TableColumnFlags, TableColumnSetup, TableFlags, TableRowFlags, Ui,
};

use crate::{
    data::{CompetitionData, MatchResult},
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
            let erg_screen_state = program_state.erg_view_screen_state.as_mut().unwrap();

            if data.current_interim_result.is_none() {
                data.calc_interim_result();
            }

            if let Some(_tab_bar_token) = ui.tab_bar("##group_selector") {
                data.group_names
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|(idx, group_name)| {
                        if let Some(_tab_item_token) = ui.tab_item(group_name) {
                            draw_erg_table(ui, data, idx);
                            ui.new_line();
                            ui.new_line();
                            ui.new_line();
                            draw_upcoming_matches(ui, data, idx);
                        }
                    });
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
        data.current_interim_result.as_ref().unwrap()[group_idx]
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

fn draw_upcoming_matches(ui: &Ui, data: &CompetitionData, group_idx: usize) {
    center(ui, "Next Matches:");
    ui.new_line();
    if let Some(_table_token) =
        ui.begin_table_with_flags("##upcoming_matches_table", 3, TableFlags::BORDERS)
    {
        ui.table_setup_column_with(TableColumnSetup {
            name: "##Lane",
            flags: TableColumnFlags::WIDTH_STRETCH,
            init_width_or_weight: 1.0,
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

        let current_batch = data.current_batch[group_idx];
        data.matches[group_idx]
            .iter()
            .filter(|_match| _match.batch == current_batch)
            .for_each(|_match| {
                assert!(_match.result == MatchResult::NotPlayed);
                ui.table_next_row();
                ui.table_next_column();

                center(ui, format!("Lane {}", (_match.lane + 1)));

                ui.table_next_column();

                let team_a_name = &data.teams.as_ref().unwrap()[group_idx][_match.team_a].name;
                center(ui, team_a_name);

                ui.table_next_column();

                let team_b_name = &data.teams.as_ref().unwrap()[group_idx][_match.team_b].name;
                center(ui, team_b_name);
            });
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

pub struct ErgScreenState {}

impl ErgScreenState {
    pub fn new() -> Self {
        Self {}
    }
}
