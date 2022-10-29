use imgui::{InputText, InputTextMultiline, Ui};

#[allow(dead_code)]
pub struct MyTextInput<'a> {
    label: &'a str,
    hint: &'a str,
    buf: &'a mut String,
    text_input_label: String,
    offset: f32,
}

#[allow(dead_code)]
impl MyTextInput<'_> {
    pub fn new<'a>(label: &'a str, hint: &'a str, buf: &'a mut String) -> MyTextInput<'a> {
        MyTextInput {
            label,
            hint,
            buf,
            offset: 0.0,
            text_input_label: format!("##{}", label),
        }
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    // sets the label for the underlying text input
    pub fn text_input_label(mut self, text_input_label: String) -> Self {
        self.text_input_label = text_input_label;
        self
    }

    // returns true if changed
    pub fn build(self: &mut Self, ui: &Ui, max_label_size: f32) -> bool {
        ui.text(self.label);

        ui.same_line_with_pos(self.offset + max_label_size + 20.0);
        InputText::new(ui, &self.text_input_label, self.buf)
            .hint(self.hint)
            .auto_select_all(false)
            .build()
    }
}

#[allow(dead_code)]
pub struct MyMultilineTextInput<'a> {
    label: &'a str,
    buf: &'a mut String,
}

#[allow(dead_code)]
impl MyMultilineTextInput<'_> {
    pub fn new<'a>(label: &'a str, buf: &'a mut String) -> MyMultilineTextInput<'a> {
        MyMultilineTextInput { label, buf }
    }

    pub fn build(self: &mut Self, ui: &Ui, max_label_size: f32, size: [f32; 2]) -> bool {
        ui.text(self.label);
        ui.same_line_with_pos(max_label_size + 20.0);
        InputTextMultiline::new(ui, format!("##{}", self.label), self.buf, size).build()
    }
}
