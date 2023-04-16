use crate::widgets::new_competition::group_page::GroupPage;
use crate::widgets::tile::Tile;
use crate::widgets::time_selector::TimeSelector;
use gdk4::{glib::clone, prelude::*, subclass::prelude::*};
use gtk4::Align;
use gtk4::{
    glib, subclass::widget::*, traits::*, Box as GtkBox, Button, Calendar, CenterBox, Entry, EntryBuffer, FlowBox, Grid, Image, Justification, Label,
    Notebook, Orientation, PackType, TextBuffer, TextView, Widget, Window,
};
use std::cell::{Cell, RefCell};

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct BaseInformationScreen {
        flow_box: FlowBox,
        competition_name_buffer: EntryBuffer,
        date_time_label: Label,
        location_buffer: EntryBuffer,
        executor_buffer: EntryBuffer,
        organizer_buffer: EntryBuffer,
        referee_buffer: EntryBuffer,
        competition_manager_buffer: EntryBuffer,
        secretary_buffer: EntryBuffer,
        additional_text_buffer: TextBuffer,
        date_time: RefCell<Option<glib::DateTime>>,
        group_idx_counter: Cell<u32>,
    }

    impl Default for BaseInformationScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BaseInformationScreen {
        const NAME: &'static str = "BaseInformationScreen";
        type Type = super::BaseInformationScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("base_information");
        }
    }

    impl ObjectImpl for BaseInformationScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            self.flow_box.set_hexpand(true);
            obj.set_hexpand(true);

            let base_info = self.create_base_info_tile();
            let groups_tile = self.create_groups_tile();

            self.flow_box.insert(&base_info, -1);
            self.flow_box.insert(&groups_tile, -1);

            // make TextView focusable with one click
            self.flow_box.set_focusable(false);
            self.flow_box.set_focus_on_click(false);
            {
                let mut child_opt = self.flow_box.first_child();
                loop {
                    if let Some(child) = child_opt.as_ref() {
                        child.set_focusable(false);
                        child.set_focus_on_click(false);
                        child_opt = child.next_sibling();
                    } else {
                        break;
                    }
                }
            }

            self.flow_box.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.flow_box.unparent();
        }
    }
    impl WidgetImpl for BaseInformationScreen {}

    impl BaseInformationScreen {
        pub fn new() -> Self {
            let flow_box = FlowBox::builder()
                .max_children_per_line(2)
                .min_children_per_line(2)
                .orientation(gtk4::Orientation::Horizontal)
                .selection_mode(gtk4::SelectionMode::None)
                .homogeneous(true)
                .focus_on_click(false)
                .focusable(false)
                .build();

            Self {
                flow_box,
                competition_name_buffer: EntryBuffer::new(None::<String>),
                date_time_label: Label::new(Some("Select date and time")),
                location_buffer: EntryBuffer::new(None::<String>),
                executor_buffer: EntryBuffer::new(None::<String>),
                organizer_buffer: EntryBuffer::new(None::<String>),
                referee_buffer: EntryBuffer::new(None::<String>),
                competition_manager_buffer: EntryBuffer::new(None::<String>),
                secretary_buffer: EntryBuffer::new(None::<String>),
                additional_text_buffer: TextBuffer::new(None),
                date_time: RefCell::new(None),
                group_idx_counter: Cell::new(0),
            }
        }

        ///
        /// Creates the tile containing input fields for the basic information.
        ///
        fn create_base_info_tile(&self) -> Widget {
            let base_info_tile = Tile::new("Base Information");

            let grid = Grid::builder().column_spacing(20).row_spacing(20).build();

            // add label and widget as a row to the grid. increase the row number each time when closure is invoked
            let mut row_counter = 0;
            let mut insert_into_grid = |label, child: &Widget| {
                // left align labels and right align children
                let label_widget = Label::builder().label(label).halign(Align::Start).build();
                child.set_halign(Align::End);
                child.set_hexpand(true);
                grid.attach(&label_widget, 0, row_counter, 1, 1);
                grid.attach(child, 1, row_counter, 1, 1);
                row_counter += 1;
            };

            let connect_check_chars = |entry: &Entry| {
                entry.connect_text_notify(|entry| {
                    // show error when disallowed characters are entered
                    if !entry.text().chars().all(|c| is_valid_name_character(c)) {
                        if !entry.css_classes().contains(&"error".into()) {
                            entry.error_bell();
                        }
                        entry.add_css_class("error");
                    } else {
                        // text is valid, reset possibly set error marker
                        entry.remove_css_class("error");
                    }
                });
            };

            let create_entry = |buffer: &EntryBuffer| -> Widget {
                let entry = Entry::with_buffer(buffer);
                connect_check_chars(&entry);
                entry.upcast()
            };

            insert_into_grid("Competition Name", &create_entry(&self.competition_name_buffer));

            let date_time_button = Button::builder().child(&self.date_time_label).css_classes(["elevated"]).build();
            date_time_button.connect_clicked(clone!(@weak self as this => move |_| this.open_date_time_window()));
            insert_into_grid("Date", &date_time_button.into());

            insert_into_grid("Location", &create_entry(&self.location_buffer));
            insert_into_grid("Executor", &create_entry(&self.executor_buffer));
            insert_into_grid("Organizer", &create_entry(&self.organizer_buffer));
            insert_into_grid("Referee", &create_entry(&self.referee_buffer));
            insert_into_grid("Competition Manager", &create_entry(&self.competition_manager_buffer));
            insert_into_grid("Secretary", &create_entry(&self.secretary_buffer));

            let additional_text_view = TextView::builder()
                .buffer(&self.additional_text_buffer)
                .justification(Justification::Center)
                .margin_start(10)
                .top_margin(5)
                .bottom_margin(5)
                .vexpand(true)
                .build();

            insert_into_grid("Additional text on bottom of result list", &additional_text_view.upcast_ref());
            additional_text_view.set_halign(Align::Fill);

            // TODO:Limit number of lines of additional text, e.g. somewhere between 3 and 5
            // self.additional_text_buffer.connect_insert_text(|buf, iter, string| {});

            base_info_tile.set_child(grid);
            base_info_tile.into()
        }

        ///
        /// Opens a window in which the user can select the time and date of the competition.
        /// Modifies `date_time` only if submit button is pressed.
        ///
        fn open_date_time_window(&self) {
            let date_time_opt = self.date_time.borrow();

            // create time selector, use previously entered values
            let time_selector = if let Some(date_time) = date_time_opt.as_ref() {
                TimeSelector::with_defaults(date_time.hour() as u32, date_time.minute() as u32)
            } else {
                TimeSelector::new()
            };

            // create calendar and set previously entered values
            let calendar = Calendar::builder().hexpand(true).vexpand(true).build();
            if let Some(date_time) = date_time_opt.as_ref() {
                calendar.set_day(date_time.day_of_month());
                // adjust for zero indexed month in calendar and 1 indexed month in date_time, so january is
                // date_time: 1
                // calendar: 0
                calendar.set_month(date_time.month() - 1);
                calendar.set_year(date_time.year());
            }

            // wrap time selector in center box
            let ts_center_box = CenterBox::builder().orientation(Orientation::Vertical).build();
            ts_center_box.set_center_widget(Some(&time_selector));

            // align calendar and time selector horizontally next to each other
            let hbox_selectors = GtkBox::new(Orientation::Horizontal, 0);
            hbox_selectors.append(&calendar);
            hbox_selectors.append(&ts_center_box);

            let hbox_buttons = GtkBox::builder()
                .orientation(Orientation::Horizontal)
                .spacing(5)
                .halign(Align::End)
                .build();

            // add buttons to submit selected date & time or to cancel
            let cancel_button = Button::builder()
                .label("Cancel")
                .css_name("highlighted_button")
                .css_classes(["action"])
                .build();
            let submit_button = Button::builder()
                .label("Submit")
                .css_name("highlighted_button")
                .css_classes(["action"])
                .build();
            hbox_buttons.append(&cancel_button);
            hbox_buttons.append(&submit_button);

            // place action buttons below calendar and time selector
            let vbox = GtkBox::new(gtk4::Orientation::Vertical, 0);
            vbox.append(&hbox_selectors);
            vbox.append(&hbox_buttons);

            // create window and make transient for root of self
            let window = Window::builder()
                .child(&vbox)
                .modal(true)
                .resizable(true)
                .title("Date & Time")
                .default_width(480)
                .default_height(360)
                .destroy_with_parent(true)
                .resizable(false)
                .build();
            let root_widget = self.obj().root().unwrap().downcast::<Window>().unwrap();
            window.set_transient_for(Some(&root_widget));
            window.show();

            // update date_time and close window when submit button is pressed
            submit_button.connect_clicked(clone!(@weak window, @weak calendar, @weak time_selector, @weak self as this => move |_| {
                let mut calendar_date = calendar.date();
                // set hours, minutes and seconds to zero
                calendar_date = calendar_date.add_hours(-calendar_date.hour()).unwrap();
                calendar_date = calendar_date.add_minutes(-calendar_date.minute()).unwrap();
                calendar_date = calendar_date.add_seconds(-calendar_date.seconds()).unwrap();

                // set entered hours and minutes
                calendar_date = calendar_date.add_hours(time_selector.hours() as i32).unwrap();
                calendar_date = calendar_date.add_minutes(time_selector.minutes() as i32).unwrap();

                // write to label in button and store date_time for later use
                let date_time_str = calendar_date.format("%d.%B %Y, %H:%M").expect("Failed to format current date time!");
                this.date_time_label.set_label(&date_time_str);
                *this.date_time.borrow_mut() = Some(calendar_date);
                window.close();
            }));

            // close window when cancel button is clicked
            cancel_button.connect_clicked(clone!(@weak window => move |_| {
                window.close();
            }));
        }

        ///
        /// Creates the tile where the user can create groups and enter team names.
        ///
        fn create_groups_tile(&self) -> Widget {
            let groups_tile = Tile::new("Groups & Teams");
            let notebook = Notebook::builder().scrollable(true).show_border(false).build();

            // default create a group
            self.create_new_groups_tab(&notebook);

            // add button to add more groups
            let button_hbox = GtkBox::new(Orientation::Horizontal, 5);
            button_hbox.append(&Image::from_icon_name("list-add"));
            button_hbox.append(&Label::new(Some("Add Group")));
            let add_group_button = Button::builder().child(&button_hbox).build();
            notebook.set_action_widget(&add_group_button, PackType::End);

            add_group_button.connect_clicked(clone!(@weak notebook, @weak self as this => move |_| {
                this.create_new_groups_tab(&notebook);
                // select last child, i.e. the new created one
                notebook.set_current_page(None);
            }));

            groups_tile.set_child(notebook);
            groups_tile.into()
        }

        fn create_new_groups_tab(&self, notebook: &Notebook) {
            let group_idx_counter = self.group_idx_counter.get();
            let page = GroupPage::new();

            let label = CenterBox::new();
            label.set_start_widget(Some(&Label::new(Some(&format!("Group {}", group_idx_counter)))));

            // add button to be able to remove this group from the notebook
            let button = Button::from_icon_name("list-remove");
            button.add_css_class("group_remove");
            button.connect_clicked(clone!(@weak notebook, @weak page => move |_| {
                if notebook.n_pages() > 1 {
                    let page_pos = notebook.page_num(&page).unwrap();
                    notebook.remove_page(Some(page_pos));
                    if notebook.n_pages() == 1 {
                        // only one group page left, cannot be removed => disable remove button of this widget
                        let page = notebook.nth_page(Some(0)).unwrap();
                        let tab_label = notebook.tab_label(&page).unwrap();
                        tab_label.last_child().unwrap().set_sensitive(false);
                    }
                }
            }));
            label.set_end_widget(Some(&button));

            // if no page is available, the remove button should not be active
            if notebook.n_pages() == 0 {
                button.set_sensitive(false);
            }

            // re-enable remove button of other tab if previously only one group page was available
            if notebook.n_pages() == 1 {
                let page = notebook.nth_page(Some(0)).unwrap();
                let tab_label = notebook.tab_label(&page).unwrap();
                tab_label.last_child().unwrap().set_sensitive(true);
            }

            notebook.append_page(&page, Some(&label));
            self.group_idx_counter.set(group_idx_counter + 1);
        }
    }
}

glib::wrapper! {
    pub struct BaseInformationScreen(ObjectSubclass<inner::BaseInformationScreen>)
        @extends Widget;
}

impl BaseInformationScreen {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}

pub fn is_valid_name_character(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_whitespace() || c.is_ascii_digit() || c == '-'
}
