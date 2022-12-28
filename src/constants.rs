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
pub const SELECTED_SCREEN_PADDING_FOUR: [f32; 4] = [0.025, 0.025, 0.025, 0.025];

// defines the rounding used in the elevated background tilings
pub const BG_TILE_ROUNDING: f32 = 20.0;

pub const RESULT_TABLE_PADDING_Y: f32 = 0.35;

// constants for the group overview screen, e.g. padding and widths
pub mod group_overview {

    /** padding between result table and upcoming matches table in the group overview screen */
    pub const INTER_TABLE_PADDING: f32 = 0.02;

    /* assertions for valid constants */

    // assert INTER_TABLE_PADDING in interval [0.0, 1.0]
    const _: () = assert!(INTER_TABLE_PADDING >= 0.0 && INTER_TABLE_PADDING <= 1.0);

    pub mod result_table {
        /** scaling factors of column widths of the result table, have to sum up to 1.0 */
        pub const RT_COLUMN_WIDTH_SCALE: [f32; 5] = [0.05, 0.7, 0.075, 0.075, 0.1];

        /** padding around the result table */
        pub const RT_PADDING: [f32; 4] = [0.05; 4];

        // assert RT_COLUMN_WIDTH_SCALE in interval [0.0, 1.0] and sum up to 1.0
        const _: () = {
            let mut i = 0;
            let mut sum = 0.0;
            while i < RT_COLUMN_WIDTH_SCALE.len() {
                assert!(RT_COLUMN_WIDTH_SCALE[i] >= 0.0 && RT_COLUMN_WIDTH_SCALE[i] <= 1.0);
                sum += RT_COLUMN_WIDTH_SCALE[i];
                i += 1
            }

            assert!(sum == 1.0);
        };
    }

    pub mod upcoming_matches_table {
        /** scaling factors of column widths of the upcoming matches table, have to sum up to 1.0 */
        pub const UMT_COLUMN_WIDTHS_SCALE: [f32; 3] = [0.2, 0.4, 0.4];

        /** padding around the upcoming matches table */
        pub const UMT_PADDING: [f32; 4] = [0.05; 4];

        // assert UMT_COLUMN_WIDTHS_SCALE in interval [0.0, 1.0] and sum up to 1.0
        const _: () = {
            let mut i = 0;
            let mut sum = 0.0;
            while i < UMT_COLUMN_WIDTHS_SCALE.len() {
                assert!(UMT_COLUMN_WIDTHS_SCALE[i] >= 0.0 && UMT_COLUMN_WIDTHS_SCALE[i] <= 1.0);
                sum += UMT_COLUMN_WIDTHS_SCALE[i];
                i += 1;
            }
            assert!(sum == 1.0);
        };
    }
}
