use crate::data::{Group, Team};
use crate::widgets::tile::Tile;
use crate::widgets::time_selector::TimeSelector;
use crate::{data::CompetitionData, widgets::new_competition::group_page::GroupPage};
use gdk4::{
    glib::{clone, closure_local, once_cell::sync::Lazy, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
};
use gtk4::{
    glib, prelude::*, subclass::widget::*, Align, Box as GtkBox, Button, Calendar, CenterBox, Entry, EntryBuffer, FlowBox, Grid, Image,
    Justification, Label, LayoutManager, Notebook, Orientation, PackType, TextBuffer, TextView, Widget, Window,
};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct BaseInformationScreen {
        /// The FlowBox storing the main tiles. (a direct child)
        flow_box: FlowBox,
        /// The Buffer of the competition name entry field.
        competition_name_buffer: EntryBuffer,
        /// The label displaying the currently selected date & time, if any.
        /// Part of the button, used to open the date & time chooser.
        date_time_label: Label,
        /// Buffer of the location entry field.
        location_buffer: EntryBuffer,
        /// Buffer of the executor entry field.
        executor_buffer: EntryBuffer,
        /// Buffer of the organizer entry field.
        organizer_buffer: EntryBuffer,
        /// Buffer of the referee entry field.
        referee_buffer: EntryBuffer,
        /// Buffer of the competition manager entry field.
        competition_manager_buffer: EntryBuffer,
        /// Buffer of the secretary entry field.
        secretary_buffer: EntryBuffer,
        /// Buffer of the additional text field.
        additional_text_buffer: TextBuffer,
        /// Currently entered date and time, can be None if no date and time was entered yet.
        date_time: RefCell<Option<glib::DateTime>>,
        /// Increment-only counter, used to generate generic default group names.
        group_idx_counter: Cell<u32>,
        /// A vector of all currently available groups.
        groups: RefCell<Vec<GroupData>>,
        /// A set of all group pages currently containing invalid inputs.
        erroneous_groups: RefCell<HashSet<GroupPage>>,
        /// A set of all group name buffers currently containing invalid chars.
        erroneous_group_names: RefCell<HashSet<EntryBuffer>>,
        /// A set of all base information entry buffers currently containing invalid chars.
        erroneous_base_infos: RefCell<HashSet<EntryBuffer>>,
        /// A button to switch to the next stage of the new competition creation.
        /// Will be insensitive if there are any invalid inputs.
        next_button_center: CenterBox,
        /// A reference to the competition data of the newly created competition by this widget.
        new_competition_data: RefCell<Rc<RefCell<CompetitionData>>>,
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
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("base_information");
        }
    }

    impl ObjectImpl for BaseInformationScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.property::<LayoutManager>("layout_manager")
                .set_property("orientation", Orientation::Vertical);

            self.flow_box.set_hexpand(true);
            obj.set_hexpand(true);

            let base_info = self.create_base_info_tile();
            let groups_tile = self.create_groups_tile();

            self.flow_box.insert(&base_info, -1);
            self.flow_box.insert(&groups_tile, -1);

            // make flowbox children not focusable
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
            self.init_next_button();
        }

        fn dispose(&self) {
            self.flow_box.unparent();
            self.next_button_center.unparent();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("all-entries-valid").param_types([bool::static_type()]).build(),
                    Signal::builder("next-screen").build(),
                ]
            });
            SIGNALS.as_ref()
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
                .vexpand(true)
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
                groups: RefCell::default(),
                erroneous_groups: RefCell::default(),
                erroneous_group_names: RefCell::default(),
                erroneous_base_infos: RefCell::default(),
                next_button_center: CenterBox::new(),
                new_competition_data: RefCell::default(),
            }
        }

        ///
        /// Initializes the button to switch to the next screen.
        /// Button will be set insensitive if there are some invalid inputs.
        ///
        fn init_next_button(&self) {
            let center_box = CenterBox::new();
            center_box.set_start_widget(Some(&Label::new(Some("Next"))));
            center_box.set_end_widget(Some(&Image::from_icon_name("go-next")));
            let next_button = Button::builder().child(&center_box).css_name("next_button").build();
            self.next_button_center.set_end_widget(Some(&next_button));
            self.next_button_center.set_parent(&*self.obj());

            next_button.connect_clicked(clone!(@weak self as this => move |_button| {
                this.handle_next();
            }));

            // set next button not sensitive by default
            next_button.set_sensitive(false);

            self.obj().connect_all_entries_valid(clone!(@weak next_button => move |_, all_valid| {
                next_button.set_sensitive(all_valid);
            }));
        }

        ///
        /// Handle the event when the user presses the next button.
        ///
        fn handle_next(&self) {
            // button is only enabled if all entries are valid, no checks here necessary
            // we can just switch to the next screen
            debug_assert!(self.are_all_entries_valid());

            // TODO: Only override changed data?
            // Current update strategy of competition data: complete override

            {
                let new_competition_data = self.new_competition_data.borrow();
                let mut data = new_competition_data.borrow_mut();

                debug_assert!(data.groups.is_empty());

                data.name = self.competition_name_buffer.text().to_string();

                {
                    let date_time = self.date_time.borrow();
                    data.date_string = date_time.as_ref().unwrap().format("%d.%B.%Y").unwrap().to_string();
                    data.time_string = date_time.as_ref().unwrap().format("%H:%M").unwrap().to_string();
                }

                data.location = self.location_buffer.text().to_string();
                data.executor = self.executor_buffer.text().to_string();
                data.organizer = self.organizer_buffer.text().to_string();
                data.referee = self.referee_buffer.text().to_string();
                data.competition_manager = self.competition_manager_buffer.text().to_string();
                data.secretary = self.secretary_buffer.text().to_string();
                data.additional_text = self
                    .additional_text_buffer
                    .text(&self.additional_text_buffer.start_iter(), &self.additional_text_buffer.end_iter(), false)
                    .to_string();

                data.groups = self
                    .groups
                    .borrow()
                    .iter()
                    .map(|group| {
                        let teams = group
                            .page
                            .get_team_names()
                            .iter()
                            .map(|[team_name, region_name]| Team::new(&mut data, team_name.clone(), region_name.clone()))
                            .collect();

                        Group {
                            name: group.name_buffer.text().to_string(),
                            teams,
                            with_break: true, // TODO: Fix with_break
                            count_batches: 0,
                            current_batch: 0,
                            matches: Vec::new(),
                        }
                    })
                    .collect();

                data.count_teams = data.groups.iter().map(|group| group.teams.len()).sum::<usize>() as u32;

                // TODO: with break flag?!
            }

            self.obj().emit_next_screen();
        }

        ///
        /// Returns whether all group entries are in a valid state.
        ///
        fn are_all_entries_valid(&self) -> bool {
            // TODO: Check for empty text fields and empty groups or groups with only 1 team (+ possibly other invalid group constellations?)
            // TODO: Check for empty timestamp
            self.erroneous_groups.borrow().is_empty()
                && self.erroneous_group_names.borrow().is_empty()
                && self.erroneous_base_infos.borrow().is_empty()
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
                entry.connect_text_notify(clone!(@weak self as this => move |entry| {
                    let mut erroneous_group_names = this.erroneous_base_infos.borrow_mut();
                    // show error when disallowed characters are entered
                    if !entry.text().chars().all(|c| is_valid_name_character(c)) {
                        if !entry.css_classes().contains(&"error".into()) {
                            entry.error_bell();
                        }
                        entry.add_css_class("error");
                        erroneous_group_names.insert(entry.buffer());
                        drop(erroneous_group_names);
                        this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                    } else if erroneous_group_names.contains(&entry.buffer()) {
                        // text is valid, reset possibly set error marker
                        entry.remove_css_class("error");
                        erroneous_group_names.remove(&entry.buffer());
                        drop(erroneous_group_names);
                        this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                    }
                }));
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
            let add_group_button = Button::builder().child(&button_hbox).focusable(false).build();
            notebook.set_action_widget(&add_group_button, PackType::End);

            add_group_button.connect_clicked(clone!(@weak notebook, @weak self as this => move |_| {
                this.create_new_groups_tab(&notebook);
                // select last child, i.e. the new created one
                notebook.set_current_page(None);
            }));

            groups_tile.set_child(notebook);
            groups_tile.into()
        }

        ///
        /// Creates a new tab in `notebook` with a generic default group name created with `group_idx_counter`.
        /// Adds a GroupPage as child to the Notebook.
        ///
        fn create_new_groups_tab(&self, notebook: &Notebook) {
            let group_idx_counter = self.group_idx_counter.get();
            self.group_idx_counter.set(group_idx_counter + 1);

            let page = GroupPage::new();

            page.connect_all_entries_valid(clone!(@weak self as this => move |page, all_valid| {
                    {
                        let mut erroneous_groups = this.erroneous_groups.borrow_mut();
                        match all_valid {
                            true => erroneous_groups.remove(page),
                            false => erroneous_groups.insert(page.clone()),
                        };
                    }
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
            }));

            let center_box = CenterBox::builder().hexpand(true).focusable(false).build();
            let group_name_buffer = EntryBuffer::new(Some(&format!("Group {}", group_idx_counter)));
            let group_name_entry = Entry::builder()
                .buffer(&group_name_buffer)
                .hexpand(true)
                .max_width_chars(16)
                .width_chars(8)
                .build();

            group_name_entry.connect_text_notify(clone!(@weak self as this => move |entry| {
                let mut erroneous_base_infos = this.erroneous_group_names.borrow_mut();
                // show error when disallowed characters are entered
                if !entry.text().chars().all(|c| is_valid_name_character(c)) {
                    if !entry.css_classes().contains(&"error".into()) {
                        entry.error_bell();
                    }
                    entry.add_css_class("error");
                    erroneous_base_infos.insert(entry.buffer());
                    drop(erroneous_base_infos);
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                } else if erroneous_base_infos.contains(&entry.buffer()) {
                    // text is valid, reset possibly set error marker
                    entry.remove_css_class("error");
                    erroneous_base_infos.remove(&entry.buffer());
                    drop(erroneous_base_infos);
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                }
            }));

            center_box.set_start_widget(Some(&group_name_entry));

            // add button to be able to remove this group from the notebook
            let remove_button = Button::builder().icon_name("list-remove").focusable(false).build();
            remove_button.add_css_class("group_remove");
            let group_name_buffer = group_name_entry.buffer();
            remove_button.connect_clicked(
                clone!(@weak self as this, @weak notebook, @weak page, @weak group_name_buffer => move |_| {
                    if notebook.n_pages() > 1 {
                        let page_pos = notebook.page_num(&page).unwrap();
                        // remove page & group name buffer
                        notebook.remove_page(Some(page_pos));
                        this.groups.borrow_mut().remove(page_pos as usize);

                        if notebook.n_pages() == 1 {
                            // only one group page left, cannot be removed => disable remove button of this widget
                            let page = notebook.nth_page(Some(0)).unwrap();
                            let tab_label = notebook.tab_label(&page).unwrap();
                            tab_label.last_child().unwrap().set_sensitive(false);
                        }

                        this.erroneous_groups.borrow_mut().remove(&page);
                        this.erroneous_group_names.borrow_mut().remove(&group_name_buffer);
                        this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                    }
                }),
            );
            center_box.set_end_widget(Some(&remove_button));

            // if no page is available, the remove button should not be active
            if notebook.n_pages() == 0 {
                remove_button.set_sensitive(false);
            }

            // re-enable remove button of other tab if previously only one group page was available
            if notebook.n_pages() == 1 {
                let page = notebook.nth_page(Some(0)).unwrap();
                let tab_label = notebook.tab_label(&page).unwrap();
                tab_label.last_child().unwrap().set_sensitive(true);
            }

            notebook.append_page(&page, Some(&center_box));

            center_box.parent().unwrap().set_focusable(false);
            self.groups.borrow_mut().push(GroupData {
                name_buffer: group_name_buffer,
                page,
            });
        }

        pub fn set_competition_data(&self, data: Rc<RefCell<CompetitionData>>) {
            *self.new_competition_data.borrow_mut() = data;
        }
    }

    #[derive(Debug)]
    struct GroupData {
        name_buffer: EntryBuffer,
        page: GroupPage,
    }
}

glib::wrapper! {
    pub struct BaseInformationScreen(ObjectSubclass<inner::BaseInformationScreen>)
        @extends Widget;
}

impl BaseInformationScreen {
    pub fn new(competition_data: &Rc<RefCell<CompetitionData>>) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.imp().set_competition_data(Rc::clone(competition_data));
        obj
    }

    pub fn connect_all_entries_valid<F: Fn(&Self, bool) + 'static>(&self, f: F) {
        self.connect_closure(
            "all-entries-valid",
            true,
            closure_local!(move |base_info_screen: &Self, all_entries_valid: bool| {
                f(base_info_screen, all_entries_valid);
            }),
        );
    }

    pub fn connect_next_screen<F: Fn(&Self) + 'static>(&self, f: F) {
        self.connect_closure(
            "next-screen",
            true,
            closure_local!(move |base_info_screen: &Self| {
                f(base_info_screen);
            }),
        );
    }

    ///
    /// Emits a signal whether all entries contains valid text.
    ///
    #[inline]
    pub fn emit_all_entries_valid(&self, all_entries_valid: bool) {
        let _: () = self.emit_by_name("all-entries-valid", &[&all_entries_valid.to_value()]);
    }

    ///
    /// Emits a signal which tells that the next button was pressed and the next screen should be shown.
    ///
    #[inline]
    pub fn emit_next_screen(&self) {
        let _: () = self.emit_by_name("next-screen", &[]);
    }
}

///
/// Returns whether a character is a valid character in a group/team name or in any base information.
///
pub fn is_valid_name_character(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_whitespace() || c.is_ascii_digit() || c == '-'
}
