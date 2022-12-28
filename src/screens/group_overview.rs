use crate::{
    colors::{BACKGROUND, BLUE, ELEVATED_BACKGROUND, GREEN, RED, TEXT},
    common::{aadd, bg_tile, center_x, padding_absolut_y, padding_relative, padding_relative_y},
    constants::{
        group_overview::{
            result_table::{RT_COLUMN_WIDTH_SCALE, RT_PADDING},
            upcoming_matches_table::{UMT_COLUMN_WIDTHS_SCALE, UMT_PADDING},
            INTER_TABLE_PADDING,
        },
        RESULT_TABLE_PADDING_Y, SELECTED_SCREEN_HEIGHT, SELECTED_SCREEN_PADDING,
        SELECTED_SCREEN_PADDING_FOUR, SELECTED_SCREEN_WIDTH,
    },
    data::{Competition, MatchResult},
    layout::GridLayout,
    tile::{Drawable, Padding, Text},
    Fonts, ProgramStage, ProgramState,
};
use imgui::{
    ChildWindow, Id, StyleColor, TableColumnFlags, TableColumnSetup, TableFlags, TableRowFlags, Ui,
};

struct GroupOverview {
    headline: Box<Text>,
    body: Box<GridLayout>,
}

impl GroupOverview {
    pub fn new(
        headline: String,
        /*competition: &'mut Competition,*/ group_idx: usize,
    ) -> Box<Self> {
        let mut children: Vec<Box<dyn Drawable>> = Vec::with_capacity(2);

        let erg_table = Padding::new(
            Padding::colored(
                ResultTable::new(/*competition, group_idx*/),
                RT_PADDING,
                ELEVATED_BACKGROUND,
            ),
            [0.0, 0.0, INTER_TABLE_PADDING / 2.0, 0.0],
        );
        children.push(erg_table);

        let upcoming_matches_table = Padding::new(
            Padding::colored(
                UpcomingMatchesTable::new(),
                UMT_PADDING,
                ELEVATED_BACKGROUND,
            ),
            [INTER_TABLE_PADDING / 2.0, 0.0, 0.0, 0.0],
        );

        children.push(upcoming_matches_table);

        Box::new(Self {
            headline: Text::new(headline),
            body: GridLayout::new(children, 2, 1, false),
        })
    }
}

impl Drawable for GroupOverview {
    fn draw(&mut self, ui: &Ui, space: [f32; 2]) {
        let pos = ui.cursor_pos();
        // draw headline
        self.headline.draw(ui, [space[0], space[1] * 0.1]);
        ui.set_cursor_pos([pos[0], pos[1] + space[1] * 0.1]);
        // draw body / GridLayout
        self.body.draw(ui, [space[0], space[1] * 0.9]);
    }
}

struct ResultTable {
    // competition: &'a mut Competition,
    // group_idx: usize,
}

impl ResultTable {
    pub fn new(/*competition: &'a mut Competition, group_idx: usize*/) -> Box<Self> {
        Box::new(Self {
            // competition,
            // group_idx,
        })
    }
}

impl Drawable for ResultTable {
    fn draw(&mut self, ui: &Ui, space: [f32; 2]) {
        // TODO: reimplement
        // draw_erg_table(ui, self.competition, self.group_idx, space[0]);
        ui.text("Result table");
    }
}

struct UpcomingMatchesTable {}

impl UpcomingMatchesTable {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Drawable for UpcomingMatchesTable {
    fn draw(&mut self, ui: &Ui, space: [f32; 2]) {
        ui.text("Upcoming matches table!");
    }
}

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
            let data = program_state.competition.data.as_ref().unwrap();
            let group_idx = program_state.stage.get_group_idx();
            Padding::new(
                GroupOverview::new(
                    String::from("Group BLUE TODO"),
                    // &mut program_state.competition,
                    group_idx,
                ),
                SELECTED_SCREEN_PADDING_FOUR,
            )
            .draw(ui, ui.window_size());
        });
}

/* pub fn build(ui: &Ui, program_state: &mut ProgramState) {
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
            ui.text(group_name);
            font_token.pop();

            // calculate table width, equally space for both tables, take padding on the out- and inside into account
            let window_width_without_padding = ui.window_size()[0]
                * (1.0 - SELECTED_SCREEN_PADDING[0] * 2.0 - INTER_TABLE_PADDING);

            // left erg table, right upcoming matches, each half of the available space
            let table_width = window_width_without_padding / 2.0;

            padding_relative(ui, SELECTED_SCREEN_PADDING);
            draw_erg_table(ui, &mut program_state.competition, group_idx, table_width);

            draw_upcoming_matches(ui, &mut program_state.competition, group_idx, table_width);
        });
    bg_token.pop();
    text_token.pop();
} */

fn draw_erg_table(ui: &Ui, competition: &mut Competition, group_idx: usize, table_width: f32) {
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

    let base_pos = ui.cursor_pos();

    bg_tile(ui, [0.35, 0.35]);

    if let Some(_table_token) = ui.begin_table_with_sizing(
        "##erg_table",
        5,
        TableFlags::BORDERS | TableFlags::SIZING_FIXED_FIT,
        [table_width, 0.0],
        table_width,
    ) {
        // add the columns
        ui.table_setup_column_with(TableColumnSetup {
            name: "Place",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * RT_COLUMN_WIDTH_SCALE[0],
            user_id: Id::Int(0),
        });

        ui.table_setup_column_with(TableColumnSetup {
            name: "Team",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * RT_COLUMN_WIDTH_SCALE[1],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Points",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * RT_COLUMN_WIDTH_SCALE[2],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Quotient",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * RT_COLUMN_WIDTH_SCALE[3],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "Stock Points",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * RT_COLUMN_WIDTH_SCALE[4],
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

fn draw_upcoming_matches(
    ui: &Ui,
    competition: &mut Competition,
    group_idx: usize,
    table_width: f32,
) {
    // center_x(ui, "Next Matches:");
    // ui.new_line();
    // padding_relative(ui, [SELECTED_SCREEN_PADDING[0], 0.0]);

    bg_tile(ui, [0.35, 0.35]);

    if let Some(_table_token) = ui.begin_table_with_sizing(
        "##upcoming_matches_table",
        3,
        TableFlags::BORDERS,
        [table_width, 0.0],
        table_width,
    ) {
        // setup up columns
        ui.table_setup_column_with(TableColumnSetup {
            name: "##Lane",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * UMT_COLUMN_WIDTHS_SCALE[0],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##TeamA",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * UMT_COLUMN_WIDTHS_SCALE[1],
            user_id: Id::Int(0),
        });
        ui.table_setup_column_with(TableColumnSetup {
            name: "##TeamB",
            flags: TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: table_width * UMT_COLUMN_WIDTHS_SCALE[2],
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
