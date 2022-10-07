pub const TOP_MENU_WIDTH: f32 = 1.0;
pub const TOP_MENU_HEIGHT: f32 = 0.05;

pub const NAVIGATION_BAR_WIDTH: f32 = 0.15;
pub const NAVIGATION_BAR_HEIGHT: f32 = 0.85;

pub const BOTTOM_MENU_WIDTH: f32 = 1.0;
pub const BOTTOM_MENU_HEIGHT: f32 = 0.1;

pub const SELECTED_SCREEN_WIDTH: f32 = 0.85;
pub const SELECTED_SCREEN_HEIGHT: f32 = 0.85;

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
