use std::collections::HashSet;

use imgui::{ChildWindow, StyleColor, Ui};

use crate::colors::{BORDER, BUTTON_TEXT_HOVERED, ELEVATED_BACKGROUND, HIGHLIGHT, SEPARATOR, TEXT};
use crate::common::{list_view, padding_relative};
use crate::constants::{BORDER_THICKNESS, NAVIGATION_PADDING, NAVIGATION_SEPARATOR_LEN};
use crate::constants::{
    BOTTOM_MENU_HEIGHT, BOTTOM_MENU_WIDTH, NAVIGATION_BAR_HEIGHT, NAVIGATION_BAR_WIDTH,
    TOP_MENU_HEIGHT, TOP_MENU_WIDTH,
};
use crate::ProgramStage;
use crate::ProgramState;

mod my_input_text;

pub mod buttons;
pub mod erg_screen;
pub mod group_overview;
pub mod home_screen;
pub mod new_screen;
pub mod start_screen;

const GENERAL_BUTTONS_COUNT: u64 = 4;
const GROUP_OFFSET: u64 = GENERAL_BUTTONS_COUNT + 1;
const GROUP_BUTTON_COUNT: u64 = 3;

pub fn build(ui: &Ui, program_state: &mut ProgramState) {
    let text_token = ui.push_style_color(StyleColor::Text, TEXT);
    build_top_menu(ui, program_state);
    build_navigation_bar(ui, program_state);
    build_bottom_menu(ui, program_state);
    build_selected_screen(ui, program_state);
    text_token.pop();
}

fn build_top_menu(ui: &Ui, program_state: &mut ProgramState) {
    ui.set_cursor_pos([0.0, 0.0]);
    let bg_token = ui.push_style_color(StyleColor::ChildBg, ELEVATED_BACKGROUND);
    ChildWindow::new("##top_menu")
        .size([
            program_state.size[0] * TOP_MENU_WIDTH,
            program_state.size[1] * TOP_MENU_HEIGHT,
        ])
        .bring_to_front_on_focus(false)
        .movable(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(ui, || {
            let window_size = ui.window_size();
            // draw border on bottom side
            ui.get_window_draw_list()
                .add_line(
                    [0.0, window_size[1] - BORDER_THICKNESS],
                    [window_size[0], window_size[1] - BORDER_THICKNESS],
                    BORDER,
                )
                .thickness(BORDER_THICKNESS)
                .build();
        });
    bg_token.pop();
}

fn build_navigation_bar(ui: &Ui, program_state: &mut ProgramState) {
    let bg_token = ui.push_style_color(StyleColor::ChildBg, ELEVATED_BACKGROUND);
    let y_offset = program_state.size[1] * TOP_MENU_HEIGHT;
    ui.set_cursor_pos([0.0, y_offset]);
    ChildWindow::new("##navigation_bar")
        .size([
            program_state.size[0] * NAVIGATION_BAR_WIDTH,
            program_state.size[1] * NAVIGATION_BAR_HEIGHT,
        ])
        .bring_to_front_on_focus(false)
        .movable(false)
        .scroll_bar(true)
        .scrollable(true)
        .build(ui, || {
            let window_size = ui.window_size();
            // draw border on right side
            ui.get_window_draw_list()
                .add_line(
                    [window_size[0] - BORDER_THICKNESS, y_offset],
                    [window_size[0] - BORDER_THICKNESS, y_offset + window_size[1]],
                    BORDER,
                )
                .thickness(BORDER_THICKNESS)
                .build();

            padding_relative(ui, NAVIGATION_PADDING);

            let button_bg_token = ui.push_style_color(StyleColor::Button, ELEVATED_BACKGROUND);

            let group_count = program_state
                .competition
                .data
                .as_ref()
                .map_or(0, |data| data.team_distribution[0]) as u64;

            let group_button_offset = GROUP_OFFSET + group_count + 1;

            let mut item_count = GENERAL_BUTTONS_COUNT;
            if program_state.competition.data.is_some() {
                // separator above and buttons for each group
                item_count += group_count + 1;
            }

            if program_state.stage.is_group_stage() {
                // separator above and buttons for each sub button
                item_count += GROUP_BUTTON_COUNT + 1;
            }

            list_view(ui, item_count, |item_idx| {
                if item_idx < GROUP_OFFSET {
                    match item_idx {
                        0 => build_navigation_button(
                            ui,
                            program_state,
                            "Home",
                            group_button_offset,
                            item_idx,
                            |program_state| {
                                program_state.switch_to_stage(ProgramStage::Home);
                            },
                        ),

                        1 => build_navigation_button(
                            ui,
                            program_state,
                            "Settings",
                            group_button_offset,
                            item_idx,
                            |program_state| {
                                program_state.switch_to_stage(ProgramStage::Settings);
                            },
                        ),

                        2 => build_navigation_button(
                            ui,
                            program_state,
                            "Current Competition",
                            group_button_offset,
                            item_idx,
                            |program_state| {
                                program_state.switch_to_stage(ProgramStage::CompetitionOverview);
                            },
                        ),
                        3 => build_navigation_button(
                            ui,
                            program_state,
                            "Exports",
                            group_button_offset,
                            item_idx,
                            |program_state| {
                                program_state.switch_to_stage(ProgramStage::Exports);
                            },
                        ),
                        4 => build_separator(ui, program_state),
                        _ => panic!("Implementation error: invalid index {item_idx} in list view"),
                    }
                } else if item_idx >= GROUP_OFFSET && item_idx < GROUP_OFFSET + group_count {
                    build_group_button(
                        ui,
                        program_state,
                        group_button_offset,
                        item_idx,
                        (item_idx - GROUP_OFFSET) as usize,
                    )
                } else if item_idx == GROUP_OFFSET + group_count {
                    build_separator(ui, program_state)
                } else if item_idx >= GROUP_OFFSET + group_count + 1
                    && item_idx <= GROUP_OFFSET + group_count + GROUP_BUTTON_COUNT + 1
                {
                    let sub_item_idx = item_idx - (GROUP_OFFSET + group_count + 1);
                    build_group_sub_button(
                        ui,
                        program_state,
                        group_button_offset,
                        item_idx,
                        sub_item_idx,
                    )
                } else {
                    panic!("Index {item_idx} not valid");
                }
            });

            button_bg_token.pop();
        });
    bg_token.pop();
}

fn build_navigation_button<F: Fn(&mut ProgramState) -> ()>(
    ui: &Ui,
    program_state: &mut ProgramState,
    button_text: &str,
    group_button_offset: u64,
    item_idx: u64,
    action: F,
) -> f32 {
    let text_token =
        if is_nav_button_highlighted(program_state.stage, group_button_offset, item_idx) {
            Some(ui.push_style_color(StyleColor::Text, HIGHLIGHT))
        } else if program_state.navigation.hovered_buttons.contains(&item_idx) {
            Some(ui.push_style_color(StyleColor::Text, BUTTON_TEXT_HOVERED))
        } else {
            None
        };

    if ui.button(button_text) {
        // TODO: button actions
        action(program_state);
    }

    if let Some(token) = text_token {
        token.pop();
    }

    if ui.is_item_hovered() {
        program_state.navigation.hovered_buttons.insert(item_idx);
    } else {
        program_state.navigation.hovered_buttons.remove(&item_idx);
    }

    ui.item_rect_size()[1] * 1.25
}

fn build_separator(ui: &Ui, program_state: &mut ProgramState) -> f32 {
    let pos = ui.cursor_pos();
    let window_size = ui.window_size();

    let x_mid = window_size[0] * 0.5;
    let half_separator_length = window_size[0] * NAVIGATION_SEPARATOR_LEN * 0.5;

    let p1 = [
        x_mid - half_separator_length,
        pos[1] + program_state.size[1] * TOP_MENU_HEIGHT + 10.0,
    ];

    // adjust for top menu, add_line takes coordinates relative to the whole os window, not the imgui window
    let p2 = [
        x_mid + half_separator_length,
        pos[1] + program_state.size[1] * TOP_MENU_HEIGHT + 10.0,
    ];

    ui.get_window_draw_list()
        .add_line(p1, p2, SEPARATOR)
        .build();
    let size = ui.item_rect_size()[1];
    size
}

fn build_group_button(
    ui: &Ui,
    program_state: &mut ProgramState,
    group_button_offset: u64,
    item_idx: u64,
    group_idx: usize,
) -> f32 {
    debug_assert!(
        (group_idx as u64)
            < program_state
                .competition
                .data
                .as_ref()
                .unwrap()
                .group_names
                .as_ref()
                .unwrap()
                .len() as u64
    );

    let group_name = program_state
        .competition
        .data
        .as_ref()
        .unwrap()
        .group_names
        .as_ref()
        .unwrap()[group_idx]
        .clone();

    build_navigation_button(
        ui,
        program_state,
        group_name.as_str(),
        group_button_offset,
        item_idx,
        |program_state| {
            program_state.switch_to_stage(ProgramStage::GroupOverview(group_idx));
        },
    )
}

fn build_group_sub_button(
    ui: &Ui,
    program_state: &mut ProgramState,
    group_button_offset: u64,
    item_idx: u64,
    sub_item_idx: u64,
) -> f32 {
    match sub_item_idx {
        0 => build_navigation_button(
            ui,
            program_state,
            "Overview",
            group_button_offset,
            item_idx,
            |program_state| {
                if !program_state.stage.is_group_stage() {
                    return;
                }
                program_state.switch_to_stage(ProgramStage::GroupOverview(
                    program_state.stage.get_group_idx(),
                ));
            },
        ),
        1 => build_navigation_button(
            ui,
            program_state,
            "Enter results",
            group_button_offset,
            item_idx,
            |program_state| {
                if !program_state.stage.is_group_stage() {
                    return;
                }
                program_state.switch_to_stage(ProgramStage::GroupEnterResults(
                    program_state.stage.get_group_idx(),
                ));
            },
        ),
        2 => build_navigation_button(
            ui,
            program_state,
            "Match history",
            group_button_offset,
            item_idx,
            |program_state| {
                if !program_state.stage.is_group_stage() {
                    return;
                }
                program_state.switch_to_stage(ProgramStage::GroupMatchHistory(
                    program_state.stage.get_group_idx(),
                ));
            },
        ),
        _ => panic!("Implementation error: invalid index {item_idx} and sub item index {sub_item_idx} in list view"),
    }
}

fn is_nav_button_highlighted(stage: ProgramStage, group_button_offset: u64, item_idx: u64) -> bool {
    // TODO: Adjust and extend
    match stage {
        ProgramStage::Home => item_idx == 0,
        ProgramStage::Settings => item_idx == 1,
        ProgramStage::CompetitionOverview => item_idx == 2,
        ProgramStage::Exports => item_idx == 3,
        ProgramStage::GroupOverview(group_idx) => {
            item_idx == GROUP_OFFSET + group_idx as u64 || item_idx == group_button_offset
        }
        ProgramStage::GroupEnterResults(group_idx) => {
            item_idx == GROUP_OFFSET + group_idx as u64 || item_idx == group_button_offset + 1
        }
        ProgramStage::GroupMatchHistory(group_idx) => {
            item_idx == GROUP_OFFSET + group_idx as u64 || item_idx == group_button_offset + 2
        }
    }
}

fn build_bottom_menu(ui: &Ui, program_state: &mut ProgramState) {
    let bg_token = ui.push_style_color(StyleColor::ChildBg, ELEVATED_BACKGROUND);
    let y_offset = program_state.size[1] * (TOP_MENU_HEIGHT + NAVIGATION_BAR_HEIGHT);
    ui.set_cursor_pos([0.0, y_offset]);
    ChildWindow::new("##bottom_menu")
        .size([
            program_state.size[0] * BOTTOM_MENU_WIDTH,
            program_state.size[1] * BOTTOM_MENU_HEIGHT,
        ])
        .bring_to_front_on_focus(false)
        .movable(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(ui, || {
            let window_size = ui.window_size();
            // draw border on top side
            ui.get_window_draw_list()
                .add_line([0.0, y_offset], [window_size[0], y_offset], BORDER)
                .thickness(BORDER_THICKNESS)
                .build();
        });
    bg_token.pop();
}

fn build_selected_screen(ui: &Ui, program_state: &mut ProgramState) {
    ui.set_cursor_pos([
        program_state.size[0] * NAVIGATION_BAR_WIDTH,
        program_state.size[1] * TOP_MENU_HEIGHT,
    ]);
    match program_state.stage {
        ProgramStage::Home => home_screen::build(ui, program_state),
        ProgramStage::GroupOverview(_) => group_overview::build(ui, program_state),
        _ => todo!(),
    }
}

pub struct NavigationState {
    pub hovered_buttons: HashSet<u64>,
}

impl NavigationState {
    pub fn new() -> Self {
        Self {
            hovered_buttons: HashSet::new(),
        }
    }
}
