pub const FONT_SIZE: f64 = 20.0;
pub const HEADLINE_FONT_SIZE: f64 = 30.0;

pub const TOP_MENU_WIDTH: f32 = 1.0;
pub const TOP_MENU_HEIGHT: f32 = 0.05;

pub const NAVIGATION_BAR_WIDTH: f32 = 0.10;
pub const NAVIGATION_BAR_HEIGHT: f32 = 0.875;

pub const BOTTOM_MENU_WIDTH: f32 = 1.0;
pub const BOTTOM_MENU_HEIGHT: f32 = 0.075;

pub const SELECTED_SCREEN_WIDTH: f32 = 0.90;
pub const SELECTED_SCREEN_HEIGHT: f32 = 0.875;

// assert some metrics to ensure to provide valid values
const _: () = assert!(TOP_MENU_WIDTH >= 0.0 && TOP_MENU_WIDTH <= 1.0);
const _: () = assert!(TOP_MENU_HEIGHT >= 0.0 && TOP_MENU_HEIGHT <= 1.0);

const _: () = assert!(NAVIGATION_BAR_WIDTH >= 0.0 && NAVIGATION_BAR_WIDTH <= 1.0);
const _: () = assert!(NAVIGATION_BAR_HEIGHT >= 0.0 && NAVIGATION_BAR_HEIGHT <= 1.0);

const _: () = assert!(BOTTOM_MENU_WIDTH >= 0.0 && BOTTOM_MENU_WIDTH <= 1.0);
const _: () = assert!(BOTTOM_MENU_HEIGHT >= 0.0 && BOTTOM_MENU_HEIGHT <= 1.0);

const _: () = assert!(SELECTED_SCREEN_WIDTH >= 0.0 && SELECTED_SCREEN_WIDTH <= 1.0);
const _: () = assert!(SELECTED_SCREEN_HEIGHT >= 0.0 && SELECTED_SCREEN_HEIGHT <= 1.0);

const _: () = assert!(NAVIGATION_BAR_WIDTH + SELECTED_SCREEN_WIDTH == 1.0);
const _: () = assert!(NAVIGATION_BAR_HEIGHT == SELECTED_SCREEN_HEIGHT);
const _: () = assert!(TOP_MENU_HEIGHT + NAVIGATION_BAR_HEIGHT + BOTTOM_MENU_HEIGHT == 1.0);

pub const BORDER_THICKNESS: f32 = 1.0;
pub const NAVIGATION_PADDING: [f32; 2] = [0.025, 0.01];
pub const NAVIGATION_SEPARATOR_LEN: f32 = 1.0 - NAVIGATION_PADDING[0] * 3.0;

pub const SELECTED_SCREEN_PADDING: [f32; 2] = [0.025, 0.01];

pub const RESULT_TABLE_COLUMN_WIDTHS_SCALE: [f32; 5] = [0.05, 0.7, 0.075, 0.075, 0.1];
pub const RESULT_TABLE_WIDTH_SCALE: f32 = {
    let mut sum = 0.0;
    let mut i = 0;
    while i < RESULT_TABLE_COLUMN_WIDTHS_SCALE.len() {
        sum += RESULT_TABLE_COLUMN_WIDTHS_SCALE[i];
        i += 1;
    }
    sum
};

pub const RESULT_TABLE_PADDING_Y: f32 = 0.35;

pub const _: () = assert!(RESULT_TABLE_WIDTH_SCALE > 0.0 && RESULT_TABLE_WIDTH_SCALE <= 1.0);

pub const NEXT_GAME_TABLE_COLUMN_WIDTHS_SCALE: [f32; 3] = [0.2, 0.4, 0.4];
pub const NEXT_GAME_TABLE_WIDTH_SCALE: f32 = {
    let mut sum = 0.0;
    let mut i = 0;
    while i < RESULT_TABLE_COLUMN_WIDTHS_SCALE.len() {
        sum += RESULT_TABLE_COLUMN_WIDTHS_SCALE[i];
        i += 1;
    }
    sum
};

pub const _: () = assert!(NEXT_GAME_TABLE_WIDTH_SCALE > 0.0 && NEXT_GAME_TABLE_WIDTH_SCALE <= 1.0);
