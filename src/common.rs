// Helper

use imgui::Ui;

#[allow(dead_code)]
pub fn center<T: AsRef<str>>(ui: &Ui, text: T) {
    let cursor_pos = ui.cursor_pos();
    ui.set_cursor_pos([
        cursor_pos[0] + (ui.content_region_avail()[0] - ui.calc_text_size(&text)[0]) / 2.0,
        cursor_pos[1] + (ui.content_region_avail()[1] - ui.calc_text_size(&text)[1]) / 2.0,
    ]);
    ui.text(&text);
}

pub fn center_x<T: AsRef<str>>(ui: &Ui, text: T) {
    let cursor_pos = ui.cursor_pos();
    ui.set_cursor_pos([
        cursor_pos[0] + (ui.content_region_avail()[0] - ui.calc_text_size(&text)[0]) / 2.0,
        cursor_pos[1],
    ]);
    ui.text(&text);
}

#[allow(dead_code)]
pub fn center_y<T: AsRef<str>>(ui: &Ui, text: T) {
    let cursor_pos = ui.cursor_pos();
    ui.set_cursor_pos([
        cursor_pos[0],
        cursor_pos[1] + (ui.content_region_avail()[1] - ui.calc_text_size(&text)[1]) / 2.0,
    ]);
    ui.text(&text);
}

#[allow(dead_code)]
pub fn center_text_around_cursor<T: AsRef<str>>(ui: &Ui, text: T) {
    let text_size = ui.calc_text_size(&text)[0];
    let cursor_pos = ui.cursor_pos();
    ui.set_cursor_pos([cursor_pos[0] - text_size / 2.0, cursor_pos[1]]);
    ui.text(&text);
}

pub fn padding_relative(ui: &Ui, padding: [f32; 2]) {
    let mut pos = ui.cursor_pos();
    let window_size = ui.window_size();
    pos[0] += window_size[0] * padding[0];
    pos[1] += window_size[1] * padding[1];
    ui.set_cursor_pos(pos);
}

#[allow(dead_code)]
pub fn padding_absolut(ui: &Ui, padding: [f32; 2]) {
    let mut pos = ui.cursor_pos();
    pos[0] += padding[0];
    pos[1] += padding[1];
    ui.set_cursor_pos(pos);
}

#[allow(dead_code)]
pub fn padding_absolut_x(ui: &Ui, padding_x: f32) {
    let mut pos = ui.cursor_pos();
    pos[0] += padding_x;
    ui.set_cursor_pos(pos);
}

pub fn padding_absolut_y(ui: &Ui, padding_y: f32) {
    let mut pos = ui.cursor_pos();
    pos[1] += padding_y;
    ui.set_cursor_pos(pos);
}

///
/// Displays the item builded by `item_builder` as a list.
/// `item_builder` gets as parameter the item index to build and returns the height
/// of the builded item. (in window coordinates)
///
pub fn list_view<F: FnMut(u64) -> f32>(ui: &Ui, item_count: u64, mut item_builder: F) {
    assert!(item_count > 0);
    let mut cursor_pos = ui.cursor_pos();
    for item_idx in 0..item_count {
        ui.set_cursor_pos(cursor_pos);
        let item_height = item_builder(item_idx);
        cursor_pos[1] += item_height;
    }
}
