// Helper

use imgui::Ui;

pub fn center<T: AsRef<str>>(ui: &Ui, text: T) {
    ui.set_cursor_pos([
        ui.cursor_pos()[0] + (ui.content_region_avail()[0] - ui.calc_text_size(&text)[0]) / 2.0,
        ui.cursor_pos()[1],
    ]);
    ui.text(&text);
}

pub fn padding(ui: &Ui, padding: [f32; 2]) {
    let mut pos = ui.cursor_pos();
    let window_size = ui.window_size();
    pos[0] += window_size[0] * padding[0];
    pos[1] += window_size[1] * padding[1];
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
