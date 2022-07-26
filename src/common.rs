// Helper

use imgui::Ui;

pub fn center<T: AsRef<str>>(ui: &Ui, text: T) {
    ui.set_cursor_pos([
        ui.cursor_pos()[0] + (ui.content_region_avail()[0] - ui.calc_text_size(&text)[0]) / 2.0,
        ui.cursor_pos()[1],
    ]);
    ui.text(&text);
}