use imgui::{Ui, InputText};

pub struct MyTextInput<'a> {
    label: &'a str,
    hint: &'a str,
    buf: &'a mut String,
}

impl MyTextInput<'_> {
    pub fn new<'a>(label: &'a str, hint: &'a str, buf: &'a mut String) -> MyTextInput<'a> {
        MyTextInput {
            label,
            hint,
            buf,
        }
    }

    // returns true if changed
    pub fn build(self: &mut Self, ui: &Ui, max_label_size: f32) -> bool {
        ui.text(self.label);
        ui.same_line_with_pos(max_label_size + 20.0);
        InputText::new(ui, format!("##{}", self.label), self.buf)
            .hint(self.hint)
            .auto_select_all(false)
            .build()
    }
}
