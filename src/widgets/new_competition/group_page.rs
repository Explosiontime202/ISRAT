use super::base_information::is_valid_name_character;
use super::team_name_position_object::TeamNamePositionObject;
use gdk4::{
    glib::{clone, closure_local, once_cell::sync::Lazy, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
    ContentFormats, ContentProvider, DragAction,
};
use gtk4::{
    gio::ListStore, glib, subclass::widget::*, traits::*, Align, Box as GtkBox, BoxLayout, Button, CenterBox, DragSource, DropTarget, Entry,
    EntryBuffer, Image, Label, LayoutManager, ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode, Widget, WidgetPaintable,
};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::mem;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct GroupPage {
        /// The ScrolledWindow containing the team_name_list. (a direct child)
        scrolled_window: ScrolledWindow,
        /// The ListBox storing the team name entries.
        /// The rows can be reordered by drag'n'drop.
        team_name_list: ListBox,
        /// The CenterBox containing the button to add a new team. (a direct child)
        add_team_box: CenterBox,
        /// A counter used to generate generic default team names. Only incremented.
        team_counter: Cell<u32>,
        /// The row above the currently hovered/highlighted gap during drag'n'drop
        /// None if no drag'n'drop is active or the gap above the first list element is hovered/highlighted.
        row_above: RefCell<Option<ListBoxRow>>,
        /// Management information: The row below the currently hovered/highlighted gap during drag'n'drop.
        /// None if no drag'n'drop is active or the gap below the last list element is hovered/highlighted.
        row_below: RefCell<Option<ListBoxRow>>,
        /// Stores the team name entries as well as their positions, model of `team_name_list`.
        team_model: ListStore,
        ///
        erroneous_entries: RefCell<HashSet<EntryBuffer>>,
    }

    impl Default for GroupPage {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GroupPage {
        const NAME: &'static str = "GroupPage";
        type Type = super::GroupPage;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<BoxLayout>();
            klass.set_css_name("group_page");
        }
    }

    impl ObjectImpl for GroupPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.property::<LayoutManager>("layout_manager")
                .set_property("orientation", Orientation::Vertical);

            // bind the team name model to the team name list box
            self.team_name_list.bind_model(
                Some(&self.team_model),
                clone!(@weak self as this => @default-panic, move |object| {
                    let data: &TeamNamePositionObject = object.downcast_ref().expect("Should be a TeamNamePositionObject!");
                    this.create_row(data)
                }),
            );

            self.scrolled_window.set_child(Some(&self.team_name_list));
            self.scrolled_window.set_visible(false);
            self.scrolled_window.set_parent(&*obj);
            self.setup_drop_target();

            self.create_add_team_button();
        }

        fn dispose(&self) {
            self.scrolled_window.unparent();
            self.add_team_box.unparent();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("all-entries-valid").param_types([bool::static_type()]).build()]);
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for GroupPage {}

    impl GroupPage {
        pub fn new() -> Self {
            Self {
                scrolled_window: ScrolledWindow::builder()
                    .propagate_natural_height(true)
                    .hscrollbar_policy(gtk4::PolicyType::Never)
                    .build(),
                team_name_list: ListBox::builder().selection_mode(SelectionMode::None).build(),
                add_team_box: CenterBox::new(),
                team_counter: Cell::new(0),
                row_above: RefCell::default(),
                row_below: RefCell::default(),
                team_model: ListStore::new(TeamNamePositionObject::static_type()),
                erroneous_entries: RefCell::default(),
            }
        }

        ///
        /// Initializes the add team button and set self.obj() as parent.
        ///
        fn create_add_team_button(&self) {
            let add_center_box = CenterBox::builder().css_name("team_add_center").build();
            let add_team_button = Button::builder().child(&add_center_box).focusable(false).build();
            add_center_box.set_start_widget(Some(&Image::from_icon_name("list-add")));
            add_center_box.set_end_widget(Some(&Label::new(Some("Add Team"))));

            add_team_button.connect_clicked(clone!(@weak self as this => move |_| {
                this.add_team();
            }));

            self.add_team_box.set_center_widget(Some(&add_team_button));
            self.add_team_box.set_parent(&*self.obj());
        }

        ///
        /// Creates the DropTarget and adds it as controller to the team name list.
        /// Calls `handle_drop` when data is dropped onto the drop target.
        ///
        fn setup_drop_target(&self) {
            let drop_target = DropTarget::builder()
                .formats(&ContentFormats::for_type(TeamNamePositionObject::static_type()))
                .actions(DragAction::MOVE)
                .build();

            drop_target.connect_accept(|_, drop| {
                // only accept if drag is from this application
                drop.drag().is_some()
            });

            drop_target.connect_drop(clone!(@weak self as this => @default-return false, move |_, val, _, _| {
                if !this.check_valid_drop(val) {
                    this.unmark_before_after_row();
                    return false;
                }

                let dropped_data : TeamNamePositionObject = val.get().unwrap();

                this.handle_drop(&dropped_data);
                this.unmark_before_after_row();
                true
            }));

            drop_target.connect_enter(clone!(@weak self as this => @default-return DragAction::empty(), move |_, _, y| {
                this.update_above_below_row(y as i32);
                DragAction::MOVE
            }));

            drop_target.connect_motion(clone!(@weak self as this => @default-return DragAction::empty(), move |_, _, y| {
                this.unmark_before_after_row();
                this.update_above_below_row(y as i32);
                DragAction::MOVE
            }));

            drop_target.connect_leave(clone!(@weak self as this => move |_| {
                this.unmark_before_after_row();
            }));

            self.team_name_list.add_controller(drop_target);
        }

        ///
        /// Checks that the dropped data is valid and the drop should be handled further.
        /// Drops are rejected if the data has a wrong type or no move has to happen, i.e. the row is dropped right above or below itself.
        ///
        fn check_valid_drop(&self, value: &glib::Value) -> bool {
            // accept the drop only if a correct value type is dropped
            let drop_data: TeamNamePositionObject = match value.get() {
                Ok(drop_data) => drop_data,
                Err(_) => return false,
            };

            let src_idx = drop_data.position() as u32;
            let row_above = self.row_above.borrow();
            let row_below = self.row_below.borrow();

            // reject drags if the source row is either directly above or directly below the dropped position, i.e. no moving is necessary

            if let Some(prev_row) = row_above.as_ref() {
                let prev_pos = prev_row.index() as u32;
                if src_idx == prev_pos {
                    drop(row_above);
                    drop(row_below);
                    return false;
                }
            }

            if let Some(next_row) = row_below.as_ref() {
                let next_pos = next_row.index() as u32;
                if src_idx == next_pos {
                    drop(row_above);
                    drop(row_below);
                    return false;
                }
            }
            true
        }

        ///
        /// Handle the dropped data, i.e. moving the entry at the source position to the destination position (i.e. the point where it was dropped).
        /// All other entries between are shifted upwards or downwards, depending in which direction the row was moved.
        ///
        fn handle_drop(&self, data: &TeamNamePositionObject) {
            let row_above = self.row_above.borrow();
            let row_below = self.row_below.borrow();
            let src_idx = data.position() as u32;

            // determine the direction in which the row was moved as well as the row which will contain the data after the drop has been finished.
            let (rot_dir, dst_idx) = match (row_above.as_ref(), row_below.as_ref()) {
                (None, None) => panic!("Should not happen!"),
                (Some(_), None) => (RotateDirection::Down, self.team_model.n_items() - 1),
                (None, Some(_)) => (RotateDirection::Up, 0),
                (Some(prev_row), Some(next_row)) => {
                    // element was moved somewhere in between, move direction has to be calculated
                    let dst_idx = next_row.index();
                    let diff = dst_idx - src_idx as i32;
                    assert_ne!(diff, 0, "Difference between source {src_idx} and destination {dst_idx} cannot be zero!");
                    if diff < 0 {
                        (RotateDirection::Up, next_row.index() as u32)
                    } else {
                        (RotateDirection::Down, prev_row.index() as u32)
                    }
                }
            };

            self.rotate_entries(src_idx, dst_idx, rot_dir);
        }

        ///
        /// Rotates the entries in the `team_model` for positions in the interval `[src_idx, dst_idx]` in `rot_direction`.
        /// Entries rotated out on either side of the interval are moved in from the other side.
        /// The positions stored in the `TeamNamePositionObjects` stay the same, only the EntryBuffer are rotated around.
        ///
        fn rotate_entries(&self, src_idx: u32, dst_idx: u32, rot_direction: RotateDirection) {
            let src_data: TeamNamePositionObject = self
                .team_model
                .item(src_idx)
                .expect("Row data must exists!")
                .downcast()
                .expect("Row data has an unexpected type");

            // set teh initial values
            let mut next_modify_idx = dst_idx as i32;
            let mut buf = src_data.buffer();

            // determine which loop condition and which increment step is used, i.e. whether to count up or down
            let (cmp_f, inc): (fn(&i32, &i32) -> bool, i32) = match rot_direction {
                RotateDirection::Up => (i32::le, 1),
                RotateDirection::Down => (i32::ge, -1),
            };

            //
            while cmp_f(&next_modify_idx, &(src_idx as i32)) {
                let row_data: TeamNamePositionObject = self
                    .team_model
                    .item(next_modify_idx as u32)
                    .expect("Row data must exists!")
                    .downcast()
                    .expect("Row data has an unexpected type");

                // exchange the buffers, the buffer of this row will be handed to the the next row
                let next_buf = row_data.buffer();
                row_data.set_buffer(buf);
                buf = next_buf;

                // emit the signal that the item at this index was changed so that the ListBox updates the row.
                self.team_model.items_changed(next_modify_idx as u32, 1, 1);

                next_modify_idx = next_modify_idx + inc;
            }
        }

        ///
        /// Adds a new team to `team_model` and therefore to the list. The default team name is a generic name based on `team_counter`.
        ///
        fn add_team(&self) {
            let team_counter = self.team_counter.get() + 1;
            self.team_counter.set(team_counter);
            let generic_team_name = format!("Team {team_counter}");
            let buffer = EntryBuffer::new(Some(generic_team_name));
            let data = TeamNamePositionObject::new(self.team_model.n_items(), buffer);
            self.team_model.append(&data);

            if self.team_model.n_items() == 1 {
                self.scrolled_window.set_visible(true);
            }
        }

        ///
        /// Removes the team & row at `index`.
        ///
        fn remove_team(&self, index: u32) {
            // first rotate the row at the end of the list
            self.rotate_entries(index, self.team_model.n_items() - 1, RotateDirection::Down);
            // then remove it
            self.team_model.remove(self.team_model.n_items() - 1);
            if self.team_model.n_items() == 0 {
                self.scrolled_window.set_visible(false);
            }
        }

        ///
        /// Create the widgets for a row containing the `data`.
        ///
        fn create_row(&self, data: &TeamNamePositionObject) -> Widget {
            let row = GtkBox::new(Orientation::Horizontal, 10);

            let dnd_icon = Image::from_icon_name("open-menu-symbolic");
            let drag_source = DragSource::builder().actions(DragAction::MOVE).build();

            let number_label = Label::builder()
                .label((data.position() + 1).to_string())
                .justify(gtk4::Justification::Center)
                .build();

            let entry = Entry::builder()
                .buffer(&data.buffer())
                .halign(Align::Center)
                .hexpand(true)
                .xalign(0.5)
                .build();

            entry.connect_text_notify(clone!(@weak self as this => move |entry| {
                let mut erroneous_entries = this.erroneous_entries.borrow_mut();
                // show error when disallowed characters are entered
                if !entry.text().chars().all(|c| is_valid_name_character(c)) {
                    if !entry.css_classes().contains(&"error".into()) {
                        entry.error_bell();
                    }
                    entry.add_css_class("error");
                    erroneous_entries.insert(entry.buffer());
                    drop(erroneous_entries);
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                } else if erroneous_entries.contains(&entry.buffer())  {
                    // text is valid, reset possibly set error marker
                    erroneous_entries.remove(&entry.buffer());
                    entry.remove_css_class("error");
                    drop(erroneous_entries);
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                }
            }));

            let remove_button = Button::builder().icon_name("list-remove").focusable(false).build();

            row.append(&dnd_icon);
            row.append(&number_label);
            row.append(&entry);
            row.append(&remove_button);

            let list_box_row = ListBoxRow::builder().child(&row).focusable(false).build();

            remove_button.connect_clicked(clone!(@weak self as this, @weak list_box_row => move |_| {
                this.remove_team(list_box_row.index() as u32);
            }));

            drag_source.connect_prepare(clone!(@weak data, @weak row => @default-panic, move |drag_source, x, y| {
                drag_source.set_icon(Some(&WidgetPaintable::new(Some(&row))), x as i32, y as i32);

                let content_provider = ContentProvider::for_value(&data.to_value());
                Some(content_provider)
            }));

            dnd_icon.add_controller(drag_source);
            list_box_row.into()
        }

        ///
        /// Updates `row_above` and `row_below` according to the `y` coordinate where the mouse during drag and drop is currently hovering.
        ///
        fn update_above_below_row(&self, y: i32) {
            // get the row where the mouse is hovering
            if let Some(row) = self.team_name_list.row_at_y(y) {
                let allocation = row.allocation();
                let row_before: Option<ListBoxRow>;
                let row_after: Option<ListBoxRow>;

                if y < (allocation.y() + allocation.height() / 2) {
                    // if hovering in the upper half of the row, the gap above this row is highlighted
                    row_before = row.prev_sibling().map(|widget| widget.downcast().unwrap());
                    row_after = Some(row);
                } else {
                    // if hovering in the lower half of the row, the gap below this row is highlighted
                    row_after = row.next_sibling().map(|widget| widget.downcast().unwrap());
                    row_before = Some(row);
                }

                // set the css classes in order to highlight the corresponding top and bottom borders
                if let Some(prev_row) = row_before.as_ref() {
                    prev_row.add_css_class("drag-hover-bottom");
                }
                if let Some(next_row) = row_after.as_ref() {
                    next_row.add_css_class("drag-hover-top");
                }

                *self.row_above.borrow_mut() = row_before;
                *self.row_below.borrow_mut() = row_after;
            }
        }

        ///
        /// Remove the highlighting of the gap.
        ///
        fn unmark_before_after_row(&self) {
            let mut row_before = self.row_above.borrow_mut();
            if let Some(prev_row) = mem::replace(&mut *row_before, None) {
                prev_row.remove_css_class("drag-hover-bottom");
            }

            let mut row_after = self.row_below.borrow_mut();
            if let Some(next_row) = mem::replace(&mut *row_after, None) {
                next_row.remove_css_class("drag-hover-top");
            }
        }

        fn are_all_entries_valid(&self) -> bool {
            self.erroneous_entries.borrow().is_empty()
        }
    }

    /// The direction in which the entries should be rotated.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum RotateDirection {
        Up,
        Down,
    }
}

glib::wrapper! {
    pub struct GroupPage(ObjectSubclass<inner::GroupPage>)
        @extends Widget;
}

impl GroupPage {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }

    pub fn connect_all_entries_valid<F: Fn(&Self, bool) + 'static>(&self, f: F) {
        self.connect_closure(
            "all-entries-valid",
            true,
            closure_local!(move |page: &Self, all_entries_valid: bool| {
                f(page, all_entries_valid);
            }),
        );
    }

    pub fn emit_all_entries_valid(&self, all_entries_valid: bool) {
        let _: () = self.emit_by_name("all-entries-valid", &[&all_entries_valid.to_value()]);
    }
}
