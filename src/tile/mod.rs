use imgui::Ui;

use crate::common::aadd;

type Color = [f32; 4];

pub trait Drawable {
    fn draw(&mut self, ui: &Ui, space: [f32; 2]);
}

pub struct Padding<T: Drawable> {
    /// The child contained surrounded by the padding.
    child: Option<Box<T>>,

    /// The padding, relative to the available space: \[left, up, right, down\]
    padding: [f32; 4],

    /// Optional: The color the background of the padding is drawn.
    background_color: Option<Color>,
}

impl<T: Drawable> Padding<T> {
    /// Mainly used to break dependency chains, child **must** be initialized later on **before** drawing the Padding else this will panic!
    pub fn empty(padding: [f32; 4]) -> Box<Self> {
        if !Self::is_valid_padding(&padding) {
            panic!("Invalid padding {:?}", padding)
        }
        Box::new(Self {
            child: None,
            padding,
            background_color: None,
        })
    }

    pub fn new(child: Box<T>, padding: [f32; 4]) -> Box<Self> {
        if !Self::is_valid_padding(&padding) {
            panic!("Invalid padding {:?}", padding)
        }
        Box::new(Self {
            child: Some(child),
            padding,
            background_color: None,
        })
    }

    pub fn colored(child: Box<T>, padding: [f32; 4], background_color: Color) -> Box<Self> {
        if !Self::is_valid_padding(&padding) {
            panic!("Invalid padding {:?}", padding)
        }
        Box::new(Self {
            child: Some(child),
            padding,
            background_color: Some(background_color),
        })
    }

    /// Overrides the child of the padding with `child`.
    pub fn set_child(&mut self, child: Box<T>) {
        self.child = Some(child);
    }

    fn is_valid_padding(padding: &[f32; 4]) -> bool {
        padding.iter().all(|&p| p >= 0.0 && p <= 1.0)
            && padding[0] + padding[2] <= 1.0
            && padding[1] + padding[3] <= 1.0
    }
}

impl<T: Drawable> Drawable for Padding<T> {
    fn draw(&mut self, ui: &Ui, space: [f32; 2]) {
        assert!(self.child.is_some());

        let mut pos = ui.cursor_pos();
        let window_pos = ui.window_pos();
        let pos_w = aadd(&[pos, window_pos]);
        if let Some(bg_color) = &self.background_color {
            ui.get_window_draw_list()
                .add_rect(pos_w, aadd(&[pos_w, space]), *bg_color)
                .filled(true)
                .build();
        }
        pos[0] += space[0] * self.padding[0];
        pos[1] += space[1] * self.padding[1];
        ui.set_cursor_pos(pos);
        let child_space = [
            space[0] * (1.0 - self.padding[0] - self.padding[2]),
            space[1] * (1.0 - self.padding[1] - self.padding[3]),
        ];
        self.child.as_mut().unwrap().draw(ui, child_space);
    }
}

// TODO: Different sized text
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: String) -> Box<Self> {
        Box::new(Self { text })
    }
}

impl Drawable for Text {
    fn draw(&mut self, ui: &Ui, _: [f32; 2]) {
        ui.text(&self.text);
    }
}

// TODO: Extend to other childs than strings, e.g. Images or whole other Drawables
pub struct Button {
    text: String,
    on_change: fn(&Ui) -> (),
}

impl Drawable for Button {
    fn draw(&mut self, ui: &Ui, _: [f32; 2]) {
        let mouse_pos = ui.io().mouse_pos;
        println!("mouse_pos = {:?}", mouse_pos);
        if ui.button(&self.text) {
            (self.on_change)(ui);
        }
    }
}
