#![allow(dead_code)]

// 0x202020
pub const BACKGROUND: [f32; 4] = [
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    1.0,
];

// 0x232425
pub const ELEVATED_BACKGROUND: [f32; 4] = [
    0x23 as f32 / 255.0,
    0x24 as f32 / 255.0,
    0x25 as f32 / 255.0,
    1.0,
];

// orange, 0xd65307
pub const HIGHLIGHT: [f32; 4] = [
    0xd6 as f32 / 255.0,
    0x53 as f32 / 255.0,
    0x07 as f32 / 255.0,
    1.0,
];

// light gray 3, 0xcccccc
pub const TEXT: [f32; 4] = [
    0xcc as f32 / 255.0,
    0xcc as f32 / 255.0,
    0xcc as f32 / 255.0,
    1.0,
];

pub const BORDER: [f32; 4] = [
    0xd6 as f32 / 255.0,
    0x53 as f32 / 255.0,
    0x07 as f32 / 255.0,
    0.5,
];

pub const SCROLLBAR_BG: [f32; 4] = BACKGROUND;
pub const SCROLLBAR_GRAB: [f32; 4] = [
    0x2d as f32 / 255.0,
    0x2e as f32 / 255.0,
    0x2f as f32 / 255.0,
    0.5,
];

pub const SCROLLBAR_GRAB_ACTIVE: [f32; 4] = [
    0x2d as f32 / 255.0,
    0x2e as f32 / 255.0,
    0x2f as f32 / 255.0,
    0.75,
];

pub const SCROLLBAR_GRAB_HOVERED: [f32; 4] = [
    0x2d as f32 / 255.0,
    0x2e as f32 / 255.0,
    0x2f as f32 / 255.0,
    1.0,
];

// light gray 3, 0xcccccc
pub const BUTTON_TEXT: [f32; 4] = [
    0xcc as f32 / 255.0,
    0xcc as f32 / 255.0,
    0xcc as f32 / 255.0,
    1.0,
];

pub const BUTTON_TEXT_HOVERED: [f32; 4] = [
    0xe5 as f32 / 255.0,
    0xe5 as f32 / 255.0,
    0xe5 as f32 / 255.0,
    1.0,
];

pub const BUTTON_TEXT_ACTIVE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub const SEPARATOR: [f32; 4] = [
    0x80 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0x80 as f32 / 255.0,
    1.0,
];

pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
