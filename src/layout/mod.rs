use imgui::{ChildWindow, Ui};

use crate::tile::Drawable;

pub struct GridLayout {
    /// Use dyn here in order to be able to draw different Tiles
    children: Vec<Box<dyn Drawable>>,

    /**
     * The amount of items drawn in x direction in each row.
     * Has to be greater than zero.
     */
    x_item_count: u32,

    /**
     * The amount of items drawn in y direction in each column.
     * Has to be zero iff GridLayout is scrollable.
     */
    y_item_count: u32,

    /// If set, a scrollable imgui ChildWindow will be used.
    scrollable: bool,
}

impl GridLayout {
    pub fn new(
        children: Vec<Box<dyn Drawable>>,
        x_item_count: u32,
        y_item_count: u32,
        scrollable: bool,
    ) -> Box<Self> {
        assert!(scrollable == (y_item_count == 0), "Iff scrollable, the y_item_count has to be zero!"
        );
        assert!(x_item_count > 0, "x_item_count cannot be zero!");
        assert!(
            y_item_count == 0 || (children.len() as u32) <= x_item_count * y_item_count,
            "Too much children, cannot draw in given item counts."
        );
        Box::new(Self {
            children: children,
            x_item_count,
            y_item_count,
            scrollable,
        })
    }

    /// The available space is splitted up by a grid with dimensions \[x_item_count, y_item_count\]. Each children gets one chunk.
    /// Assumes scrollable = false;
    fn draw_children(&mut self, ui: &Ui, space: [f32; 2]) {
        let chunk = [space[0] / self.x_item_count as f32, space[1] / self.y_item_count as f32];
        let base_pos = ui.cursor_pos();
        for y in 0..self.y_item_count {
            let y_pos = base_pos[1] + chunk[1] * y as f32;
        for x in 0..self.x_item_count {
                let x_pos = base_pos[0] + chunk[0] * x as f32;
                ui.set_cursor_pos([x_pos, y_pos]);
                let child = &mut self.children[(x + y * self.y_item_count) as usize];
                child.draw(ui, chunk);
            }
        }
    }

    /// draw the children in a way that they get as much y space as they need.
    /// Assumes scrollable = true.
    fn draw_children_greedy(&mut self, ui: &Ui, space: [f32; 2]) {
        todo!();
    }
}

impl Drawable for GridLayout {
    fn draw(&mut self, ui: &Ui, space: [f32; 2]) {
        if self.scrollable {
            ChildWindow::new("##")
                .size(space)
                .scrollable(true)
                .movable(false)
                .draw_background(false)
                .build(ui, || {
                    self.draw_children_greedy(ui, space);
                });
        } else {
            self.draw_children(ui, space);
        }
    }
}
