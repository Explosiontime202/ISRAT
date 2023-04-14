use gdk4::{glib::clone, prelude::*, subclass::prelude::*, ContentProvider, DragAction};
use gtk4::{
    glib, subclass::widget::*, traits::*, Align, Box as GtkBox, BoxLayout, Button, CenterBox, DragSource, DropTarget, EditableLabel, Image, Label,
    LayoutManager, ListBox, ListBoxRow, Orientation, SelectionMode, Text, Widget, WidgetPaintable,
};
use std::cell::{Cell, RefCell};
use std::mem;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct GroupPage {
        team_name_list: ListBox,
        add_team_box: CenterBox,
        team_counter: Cell<u32>,
        count_teams: Cell<u32>,
        row_before: RefCell<Option<ListBoxRow>>,
        row_after: RefCell<Option<ListBoxRow>>,
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
            // The layout manager determines how child widgets are laid out.
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

            let add_center_box = CenterBox::builder().css_name("team_add_center").build();
            let add_team_button = Button::builder().child(&add_center_box).build();
            add_center_box.set_start_widget(Some(&Image::from_icon_name("list-add")));
            add_center_box.set_end_widget(Some(&Label::new(Some("Add Team"))));

            add_team_button.connect_clicked(clone!(@weak self as this => move |_| {
                this.add_team();
            }));

            self.add_team_box.set_center_widget(Some(&add_team_button));

            self.team_name_list.set_parent(&*obj);
            self.add_team_box.set_parent(&*obj);

            self.setup_drop_target();
        }

        fn dispose(&self) {
            self.team_name_list.unparent();
            self.add_team_box.unparent();
        }
    }
    impl WidgetImpl for GroupPage {}

    impl GroupPage {
        pub fn new() -> Self {
            let team_name_list = ListBox::builder().selection_mode(SelectionMode::None).build();

            Self {
                team_name_list,
                add_team_box: CenterBox::new(),
                team_counter: Cell::new(0),
                count_teams: Cell::new(0),
                row_before: RefCell::default(),
                row_after: RefCell::default(),
            }
        }

        fn setup_drop_target(&self) {
            let drop_target = DropTarget::new(ListBoxRow::static_type(), DragAction::MOVE);

            drop_target.connect_accept(|_, drop| {
                // only accept if drag is from this application
                drop.drag().is_some()
            });

            drop_target.connect_drop(clone!(@weak self as this => @default-return false, move |target, val, _, _| {
                let list_box : ListBox =  match target.widget().downcast() {
                    Ok(list_box) => list_box,
                    Err(_) => {
                        this.unmark_before_after_row();
                        return false;
                    }
                };

                let dropped_row : ListBoxRow = match val.get() {
                    Ok(dropped_row) => dropped_row,
                    Err(_) => {
                        this.unmark_before_after_row();
                        return false
                    },
                };

                {
                    let row_before = this.row_before.borrow();
                    let row_after = this.row_after.borrow();

                    if let Some(prev_row) = row_before.as_ref() {
                        if dropped_row == *prev_row {
                            drop(row_before);
                            drop(row_after);
                            this.unmark_before_after_row();
                            return false;
                        }
                    }

                    if let Some(next_row) = row_after.as_ref() {
                        if dropped_row == *next_row {
                            drop(row_before);
                            drop(row_after);
                            this.unmark_before_after_row();
                            return false;
                        }
                    }

                    // TODO: adjust numbers
                    list_box.remove(&dropped_row);
                    list_box.insert(&dropped_row, row_before.as_ref().map_or(0, |prev_row| prev_row.index() + 1));
                }

                this.unmark_before_after_row();
                true
            }));

            drop_target.connect_enter(clone!(@weak self as this => @default-return DragAction::empty(), move |_, _, y| {
                this.update_before_after_row(y as i32);
                DragAction::MOVE
            }));

            drop_target.connect_motion(clone!(@weak self as this => @default-return DragAction::empty(), move |_, _, y| {
                this.unmark_before_after_row();
                this.update_before_after_row(y as i32);
                DragAction::MOVE
            }));

            drop_target.connect_leave(clone!(@weak self as this => move |_| {
                this.unmark_before_after_row();
            }));

            self.team_name_list.add_controller(drop_target);
        }

        fn add_team(&self) {
            let row = GtkBox::new(Orientation::Horizontal, 10);

            let dnd_icon = Image::from_icon_name("open-menu-symbolic");
            let drag_source = DragSource::builder().actions(DragAction::all()).build();

            let count_teams = self.count_teams.get() + 1;
            self.count_teams.set(count_teams);

            let number_label = Label::builder()
                .label(count_teams.to_string())
                .justify(gtk4::Justification::Center)
                .build();

            let team_counter = self.team_counter.get() + 1;
            self.team_counter.set(team_counter);

            let editable_text = EditableLabel::builder()
                .text(&format!("Team {team_counter}"))
                .halign(Align::Center)
                .hexpand(true)
                .xalign(0.5)
                .build();

            let entry: Text = editable_text.first_child().unwrap().last_child().unwrap().downcast().unwrap();
            entry.buffer().connect_text_notify(|_buf| {
                // TODO: Show error when disallowed characters are entered
            });

            let remove_button = Button::from_icon_name("list-remove");

            row.append(&dnd_icon);
            row.append(&number_label);
            row.append(&editable_text);
            row.append(&remove_button);

            let list_box_row = ListBoxRow::builder().child(&row).build();
            self.team_name_list.append(&list_box_row);

            remove_button.connect_clicked(clone!(@weak self as this, @weak list_box_row => move |_| {
                // TODO: Handle numbers
                this.team_name_list.remove(&list_box_row);
                this.count_teams.set(this.count_teams.get() - 1);
            }));

            drag_source.connect_prepare(
                clone!(@weak self as this, @weak list_box_row, @weak row => @default-return None, move |drag_source, x, y| {
                    drag_source.set_icon(Some(&WidgetPaintable::new(Some(&row))), x as i32, y as i32);

                    let content_provider = ContentProvider::for_value(&list_box_row.to_value());
                    Some(content_provider)
                }),
            );

            dnd_icon.add_controller(drag_source);
        }

        fn update_before_after_row(&self, y: i32) {
            if let Some(row) = self.team_name_list.row_at_y(y) {
                let allocation = row.allocation();
                let row_before: Option<ListBoxRow>;
                let row_after: Option<ListBoxRow>;

                if (y) < (allocation.y() + allocation.height() / 2) {
                    row_before = row.prev_sibling().map(|widget| widget.downcast().unwrap());
                    row_after = Some(row);
                } else {
                    row_after = row.next_sibling().map(|widget| widget.downcast().unwrap());
                    row_before = Some(row);
                }

                if let Some(prev_row) = row_before.as_ref() {
                    prev_row.add_css_class("drag-hover-bottom");
                }
                if let Some(next_row) = row_after.as_ref() {
                    next_row.add_css_class("drag-hover-top");
                }

                *self.row_before.borrow_mut() = row_before;
                *self.row_after.borrow_mut() = row_after;
            }
        }

        fn unmark_before_after_row(&self) {
            let mut row_before = self.row_before.borrow_mut();
            if let Some(prev_row) = mem::replace(&mut *row_before, None) {
                prev_row.remove_css_class("drag-hover-bottom");
            }

            let mut row_after = self.row_after.borrow_mut();
            if let Some(next_row) = mem::replace(&mut *row_after, None) {
                next_row.remove_css_class("drag-hover-top");
            }
        }
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
}
