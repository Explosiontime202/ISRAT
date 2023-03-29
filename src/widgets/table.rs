use gdk4::subclass::prelude::*;
use gtk4::{glib, prelude::*, subclass::widget::*, traits::WidgetExt, Label, Widget};
use gtk4::{BoxLayout, Grid};
use std::cell::RefCell;

type Row<'a> = Vec<&'a str>;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct Table {
        // the grid used to layout the children
        grid: Grid,
        // defines whether a header already was set
        header_set: RefCell<bool>,
        // the amount of columns in the table
        column_count: RefCell<u32>,
        // the amount of rows in the table (includes the header row)
        row_count: RefCell<u32>,
    }

    impl Default for Table {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Table {
        const NAME: &'static str = "Table";
        type Type = super::Table;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<BoxLayout>();
            klass.set_css_name("table");
        }
    }

    impl ObjectImpl for Table {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.grid.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.grid.unparent();
        }
    }

    impl WidgetImpl for Table {}

    impl Table {
        fn new() -> Self {
            let grid = Grid::builder().build();
            Self {
                grid,
                header_set: RefCell::new(false),
                column_count: RefCell::new(0),
                row_count: RefCell::new(0),
            }
        }

        ///
        /// Initially sets the header or overrides the current header with a new one.
        /// Has to be called at least once before any `add_row()`.
        /// `header_set` is afterwards `true`.
        ///
        pub fn set_header(&self, new_header: Row) {
            {
                let mut column_count = self.column_count.borrow_mut();

                let column_diff = *column_count as i32 - new_header.len() as i32;
                if column_diff < 0 {
                    // we need to add columns, as we have to less
                    for _ in 0..-column_diff {
                        self.grid.insert_column(-1);
                    }
                } else if column_diff > 0 {
                    // we need to remove columns, as we have to much
                    for _ in 0..column_diff {
                        self.grid.remove_column(-1);
                    }
                }
                *column_count = new_header.len() as u32;

                let mut row_count = self.row_count.borrow_mut();
                let mut header_set = self.header_set.borrow_mut();
                if !*header_set {
                    *row_count = 1;
                    *header_set = true;
                }
            }

            // clear header row
            self.grid.remove_row(0);
            self.grid.insert_row(0);

            let header_background = Label::builder().css_name("table_header").build();
            self.grid
                .attach(&header_background, 0, 0, new_header.len() as i32, 1);

            self.build_widgets_for_row(new_header, 0);
        }

        ///
        /// Adds a row to the table.
        /// `row` has to have as many entries as the header.
        /// `row_count` is incremented by one.
        ///
        pub fn add_row(&self, row: Row) {
            assert!(
                *self.header_set.borrow(),
                "Header has to be initialized beforehand using 'set_header'!"
            );
            let mut row_count_ref = self.row_count.borrow_mut();
            self.build_widgets_for_row(row, *row_count_ref as i32);
            *row_count_ref += 1;
        }

        ///
        /// Build widgets for the row. `row` is required to have the same size as the header.
        ///
        fn build_widgets_for_row(&self, row: Row, row_idx: i32) {
            assert_eq!(
                *self.column_count.borrow(),
                row.len() as u32,
                "Added rows have to have the same size as the header!"
            );

            for (column_idx, entry) in row.iter().enumerate() {
                self.grid.attach(
                    &self.build_widget_for_entry(entry),
                    column_idx as i32,
                    row_idx as i32,
                    1,
                    1,
                );
            }
        }

        ///
        /// Builds the widget for `entry`. The top widget for entry gets the CSS name "table_entry".
        ///
        fn build_widget_for_entry(&self, entry: &str) -> Widget {
            let label = Label::builder()
                .label(entry)
                .css_name("table_entry")
                .hexpand(true)
                .build();
            label.into()
        }

        ///
        /// Removes all rows from the table. Only the header is left.
        /// Has to be called after `set_header`.
        /// Afterwards the `row_count` is `1`.
        ///
        pub fn clear_rows(&self) {
            assert!(*self.header_set.borrow());
            let mut row_count = self.row_count.borrow_mut();
            for row_idx in (1..*row_count).rev() {
                self.grid.remove_row(row_idx as i32);
            }
            *row_count = 1;
        }
    }
}

glib::wrapper! {
    pub struct Table(ObjectSubclass<inner::Table>)
        @extends Widget;
}

impl Table {
    pub fn new(header: Row) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.imp().set_header(header);
        obj
    }

    ///
    /// Change the header row.
    /// `new_header` can have a different size then the old header.
    ///
    pub fn set_header(&self, new_header: Row) {
        self.imp().set_header(new_header);
    }

    ///
    /// Adds a row to the table.
    /// `row` has to have as many entries as the header.
    ///
    pub fn add_row(&self, row: Row) {
        self.imp().add_row(row);
    }

    ///
    /// Removes all rows from the table. Only the header is left.
    ///
    pub fn clear_rows(&self) {
        self.imp().clear_rows();
    }
}
