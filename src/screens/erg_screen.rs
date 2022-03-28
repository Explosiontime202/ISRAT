use imgui::{
    ChildWindow, Id, StyleColor, TableColumnFlags, TableColumnSetup, TableFlags,
    TableRowFlags, Ui,
};

use crate::{data::CompetitionData, ProgramState};

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

            // TODO: draw selector for group
            /*if let Some(_combo_token) = ui.begin_combo(
                "##group_selector",
                data.group_names.as_ref().unwrap()[erg_screen_state.selected_group_idx].as_str(),
            ) {
                data.group_names.as_ref().unwrap().iter().enumerate().for_each(|(idx, group_name)| {
                    if Selectable::new(group_name)..build(ui) {

                    }
                });
            }*/

            if let Some(_tab_bar_token) = ui.tab_bar("##group_selector") {
                data.group_names
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|(idx, group_name)| {
                        if let Some(_tab_item_token) = ui.tab_item(group_name) {
                            draw_erg_table(ui, data, idx);
                        }
                    });
            }
        });

    window_bg_color.end();
    child_bg_color.end();
}

fn draw_erg_table(ui: &Ui, data: &CompetitionData, group_idx: usize) {
    let column_widths = [
        ui.calc_text_size(data.team_distribution[1].to_string())[0] * 2.0,
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

        let cursor_y_pos = ui.cursor_pos()[1];

        ui.table_next_column();
        ui.set_cursor_pos([
            ui.cursor_pos()[0]
                + (ui.content_region_avail()[0] - ui.calc_text_size("Place")[0]) / 2.0,
            cursor_y_pos,
        ]);
        ui.text("Place");

        ui.table_next_column();
        ui.set_cursor_pos([
            ui.cursor_pos()[0]
                + (ui.content_region_avail()[0] - ui.calc_text_size("Team")[0]) / 2.0,
            cursor_y_pos,
        ]);
        ui.text("Team");

        ui.table_next_column();
        ui.set_cursor_pos([
            ui.cursor_pos()[0]
                + (ui.content_region_avail()[0] - ui.calc_text_size("Points")[0]) / 2.0,
            cursor_y_pos,
        ]);
        ui.text("Points");

        ui.table_next_column();
        ui.set_cursor_pos([
            ui.cursor_pos()[0]
                + (ui.content_region_avail()[0] - ui.calc_text_size("Quotient")[0]) / 2.0,
            cursor_y_pos,
        ]);
        ui.text("Quotient");

        ui.table_next_column();
        ui.set_cursor_pos([
            ui.cursor_pos()[0]
                + (ui.content_region_avail()[0] - ui.calc_text_size("Stock Points")[0]) / 2.0,
            cursor_y_pos,
        ]);
        ui.text("Stock Points");

        // draw the rows and center the entries
        data.current_interim_result.as_ref().unwrap()[group_idx]
            .iter()
            .enumerate()
            .for_each(|(place_idx, entry)| {
                let cursor_y_pos = ui.cursor_pos()[1];
                ui.table_next_row();

                ui.table_next_column();
                ui.set_cursor_pos([
                    ui.cursor_pos()[0]
                        + (ui.content_region_avail()[0]
                            - ui.calc_text_size(place_idx.to_string())[0])
                            / 2.0,
                    cursor_y_pos,
                ]);

                ui.text(place_idx.to_string());

                ui.table_next_column();
                ui.set_cursor_pos([
                    ui.cursor_pos()[0]
                        + (ui.content_region_avail()[0]
                            - ui.calc_text_size(
                                &data.teams.as_ref().unwrap()[group_idx][entry.team_idx].name,
                            )[0])
                            / 2.0,
                    cursor_y_pos,
                ]);

                ui.text(&data.teams.as_ref().unwrap()[group_idx][entry.team_idx].name);

                ui.table_next_column();
                ui.set_cursor_pos([
                    ui.cursor_pos()[0]
                        + (ui.content_region_avail()[0]
                            - ui.calc_text_size(format!(
                                "{} : {}",
                                entry.match_points[0], entry.match_points[1]
                            ))[0])
                            / 2.0,
                    cursor_y_pos,
                ]);

                ui.text(format!(
                    "{} : {}",
                    entry.match_points[0], entry.match_points[1]
                ));

                ui.table_next_column();
                ui.set_cursor_pos([
                    ui.cursor_pos()[0]
                        + (ui.content_region_avail()[0]
                            - ui.calc_text_size(format!("{:.3}", entry.quotient))[0])
                            / 2.0,
                    cursor_y_pos,
                ]);

                ui.text(format!("{:.3}", entry.quotient));

                ui.table_next_column();
                ui.set_cursor_pos([
                    ui.cursor_pos()[0]
                        + (ui.content_region_avail()[0]
                            - ui.calc_text_size(format!(
                                "{} : {}",
                                entry.stock_points[0], entry.stock_points[1]
                            ))[0])
                            / 2.0,
                    cursor_y_pos,
                ]);

                ui.text(format!(
                    "{} : {}",
                    entry.stock_points[0], entry.stock_points[1]
                ));
            });
    }
}

pub struct ErgScreenState {}

impl ErgScreenState {
    pub fn new() -> Self {
        Self {}
    }
}
