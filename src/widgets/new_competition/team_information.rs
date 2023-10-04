use crate::data::{CompetitionData, Group, Team};
use crate::widgets::{fix_indexed_list::FixIndexedList, new_competition::group_team_object::GroupTeamObject, tile::Tile, new_competition::base_information::is_valid_name_character};
use gdk4::{
    gio::ListStore,
    glib::{clone, closure_local, once_cell::sync::Lazy, subclass::Signal, translate::FromGlib, SignalHandlerId, GString},
    prelude::*,
    subclass::prelude::*,
};
use gtk4::{
    glib, prelude::*, subclass::widget::*, Align, BoolFilter, Box as GtkBox, Button, CenterBox, ClosureExpression, DropDown, Entry, EntryBuffer,
    EveryFilter, Expression, FilterListModel, Image, Label, LayoutManager, ListBox, Orientation, Paned, PropertyExpression, SearchEntry,
    SignalListItemFactory, StringFilter, StringList, StringObject, Widget,
};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct TeamInformationScreen {
        /// The main child containing all the content. (a direct child)
        tile: Tile,
        /// A button to switch to the next stage of the new competition creation. (a direct child)
        /// Will be insensitive if there are any invalid inputs.
        next_button_center: CenterBox,
        /// The 6 player name buffer.
        player_name_buffer: Vec<EntryBuffer>,
        /// The buffer which contain invalid characters (indexes into `player_name_buffer`).
        erroneous_buffer: RefCell<HashSet<u32>>,
        /// An option for the currently selected team, i.e. those player names can be edited.
        selected_team_obj: RefCell<Option<GroupTeamObject>>,
        /// The currently selected group to filter for.
        /// Is None if for no group should be filtered.
        selected_group: RefCell<Option<String>>,
        /// A reference to the data of the competition which is currently created.
        new_competition_data: RefCell<Rc<RefCell<CompetitionData>>>,
        /// The model containing all teams, in pairs (team_name, group_name) stored in `GroupTeamObject`s.
        team_model: ListStore,
        /// The model containing all the groups.
        group_model: StringList,
    }

    impl Default for TeamInformationScreen {
        fn default() -> Self {
            let player_name_buffer: Vec<EntryBuffer> = (0..6).map(|_| EntryBuffer::new(None::<&str>)).collect();
            let team_name_model = ListStore::new::<GroupTeamObject>();

            Self {
                tile: Tile::new("Team Information"),
                next_button_center: CenterBox::new(),
                player_name_buffer,
                erroneous_buffer: RefCell::default(),
                selected_team_obj: RefCell::new(None),
                selected_group: RefCell::new(None),
                new_competition_data: RefCell::default(),
                team_model: team_name_model,
                group_model: StringList::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TeamInformationScreen {
        const NAME: &'static str = "TeamInformationScreen";
        type Type = super::TeamInformationScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("team_information");
        }
    }

    impl ObjectImpl for TeamInformationScreen {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .property::<LayoutManager>("layout_manager")
                .set_property("orientation", Orientation::Vertical);

            self.tile.set_parent(&*self.obj());
            self.tile.set_hexpand(true);
            self.tile.set_vexpand(true);

            let paned = Paned::builder()
                .orientation(Orientation::Horizontal)
                .vexpand(true)
                .shrink_start_child(false)
                .shrink_end_child(false)
                .build();

            paned.connect_position_notify(|paned| {
                // prevent rescaling, especially during searching
                paned.set_property("position-set", true);
            });

            let team_selector_box = self.create_team_selector_box();
            let player_name_box = self.create_player_name_box();
            paned.set_start_child(Some(&team_selector_box));
            paned.set_end_child(Some(&player_name_box));

            self.tile.set_child(paned);
            self.init_next_button();

            #[cfg(debug_assertions)]
            self.add_test_values_key_binding();
        }

        fn dispose(&self) {
            self.tile.unparent();
            self.next_button_center.unparent();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("selected-team")
                        .param_types([String::static_type(), String::static_type()])
                        .build(),
                    Signal::builder("unselect-team").build(),
                    Signal::builder("next-screen").build(),
                    Signal::builder("all-entries-valid").param_types([bool::static_type()]).build(),
                ]
            });
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for TeamInformationScreen {}

    impl TeamInformationScreen {
        /// The string used to select all groups.
        const ALL_GROUPS_SELECTOR_STR: &'static str = "All groups";

        ///
        /// Initializes the button to switch to the next screen.
        /// Button will be set insensitive if there are some invalid inputs.
        ///
        fn init_next_button(&self) {
            let center_box = CenterBox::new();
            center_box.set_start_widget(Some(&Label::new(Some("Create"))));
            center_box.set_end_widget(Some(&Image::from_icon_name("go-next")));
            let next_button = Button::builder().child(&center_box).css_name("next_button").build();
            self.next_button_center.set_end_widget(Some(&next_button));
            self.next_button_center.set_parent(&*self.obj());

            next_button.connect_clicked(clone!(@weak self as this => move |_button| {
                this.obj().emit_next_screen();
            }));

            self.obj().connect_all_entries_valid(clone!(@weak next_button => move |_, all_valid| {
                next_button.set_sensitive(all_valid);
            }));
        }

        ///
        /// Creates the left half of the Paned, i.e. where the user can select a team, for which the user likes to enter player names.
        ///
        fn create_team_selector_box(&self) -> GtkBox {
            let team_selector_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .css_name("team_selector_box")
                .build();

            // TODO: Maybe sort list alphabetically? Or by group?

            let search_box = GtkBox::builder().orientation(gtk4::Orientation::Horizontal).build();
            let search_entry = SearchEntry::builder().hexpand(true).build();
            let group_selector = self.create_group_selector();
            search_box.append(&search_entry);
            search_box.append(&group_selector);

            team_selector_box.append(&search_box);

            let team_selector_list = ListBox::builder().selection_mode(gtk4::SelectionMode::Single).build();
            // TODO: retrieve model on show
            let model = self.setup_team_model(&search_entry, &team_selector_list, &group_selector);

            team_selector_list.bind_model(
                Some(&model),
                clone!(@weak self as this, @weak team_selector_list => @default-panic, move |obj| {
                    let string_obj: &GroupTeamObject = obj.downcast_ref().unwrap();
                    Self::create_team_widget(&string_obj.team_name(), &string_obj.group_name())
                }),
            );

            team_selector_list.connect_row_selected(clone!(@weak self as this, @weak model => move |_, row_opt| {
                if let Some(row) = row_opt {
                    let team: GroupTeamObject = model.item(row.index() as u32).and_downcast().unwrap();

                    // save player names before selecting the new one
                    if let Some(prev_selected) = this.selected_team_obj.borrow().as_ref() {
                        if prev_selected != &team {
                            this.save_player_names(&prev_selected.team_name(), &prev_selected.group_name());
                            this.clear_player_name_entries();
                        }
                    }

                    // only override selected_team_obj if it is currently none or another string is stored
                    if this.selected_team_obj.borrow().is_none() || this.selected_team_obj.borrow().as_ref().unwrap() != &team {
                        this.select_team(&team);
                    }
                }
            }));

            // default select first item
            team_selector_list.select_row(team_selector_list.row_at_index(0).as_ref());

            team_selector_box.append(&team_selector_list);

            team_selector_box
        }

        ///
        /// Stores `team` in `selected_team_obj`, updates
        ///
        fn select_team(&self, team: &GroupTeamObject) {
            *self.selected_team_obj.borrow_mut() = Some(team.clone());
            self.load_player_names(&team.team_name(), &team.group_name());
            self.obj().emit_selected_team(team.team_name(), team.group_name());
        }

        ///
        /// Creates a widget for a given `team_name` and `group_name` to be displayed in the `team_selector_list`.
        ///
        fn create_team_widget(team_name: &str, group_name: &str) -> Widget {
            let center_box = CenterBox::new();
            let hbox = GtkBox::new(Orientation::Horizontal, 10);

            hbox.append(&Label::new(Some(team_name)));

            hbox.append(&Label::builder().label("-").css_classes(["dimmed"]).build());
            hbox.append(&Label::builder().label(group_name).css_classes(["dimmed"]).build());
            center_box.set_center_widget(Some(&hbox));
            center_box.into()
        }

        ///
        /// Save currently entered player names to the competition data.
        /// Stores None for empty fields.
        ///
        fn save_player_names(&self, team_name: &str, group_name: &str) {
            let data_ptr = self.new_competition_data.borrow();
            let mut data = data_ptr.borrow_mut();

            // find team in group
            let team = Self::find_team(&mut data, team_name, group_name);

            // overwrite player names
            for (idx, buffer) in self.player_name_buffer.iter().enumerate() {
                team.player_names[idx] = if !buffer.text().is_empty() { Some(buffer.text().into()) } else { None };
            }
        }

        ///
        /// Loads player names from competition data into entry buffer.
        ///
        fn load_player_names(&self, team_name: &str, group_name: &str) {
            let data_ptr = self.new_competition_data.borrow();
            let mut data = data_ptr.borrow_mut();

            // find team in group
            let team = Self::find_team(&mut data, team_name, group_name);

            // overwrite plyer name entry buffer
            for (idx, buffer) in self.player_name_buffer.iter().enumerate() {
                buffer.set_text(if let Some(player_name) = team.player_names[idx].as_ref() {
                    player_name
                } else {
                    ""
                });
            }
        }

        ///
        /// Finds the unique team for `team_name` and `group_name`.
        ///
        fn find_team<'a>(data: &'a mut CompetitionData, team_name: &str, group_name: &str) -> &'a mut Team {
            let mut teams: Vec<&'a mut Team> = data
                .groups
                .iter_mut()
                .filter(|group: &&'a mut Group| &group.name == group_name)
                .flat_map(|group| &mut group.teams)
                .filter(|team| &team.name == team_name)
                .collect();

            assert!(
                teams.len() == 1,
                "Found multiple teams with the same name ({team_name}) in the same group ({group_name})!"
            );

            teams.remove(0)
        }

        ///
        /// Resets all entries in `player_name_buffer`.
        ///
        fn clear_player_name_entries(&self) {
            // TODO: clear errors
            for buffer in &self.player_name_buffer {
                buffer.set_text("");
            }
        }

        fn create_player_name_box(&self) -> Widget {
            let player_name_box = GtkBox::builder()
                .orientation(Orientation::Vertical)
                .css_name("player_name_box")
                .sensitive(false)
                .build();

            let team_name_center = CenterBox::new();
            let team_name_box = GtkBox::new(Orientation::Horizontal, 10);
            let team_name = Label::builder().css_classes(["subheadline"]).build();
            let hyphen = Label::builder().label("-").css_classes(["subheadline", "dimmed"]).visible(false).build();
            let group_name = Label::builder().css_classes(["subheadline", "dimmed"]).build();

            self.obj().connect_selected_team(
                clone!(@weak team_name, @weak group_name, @weak hyphen, @weak player_name_box => move |_, selected_team_name, selected_group_name| {
                    team_name.set_label(&selected_team_name);
                    group_name.set_label(&selected_group_name);
                    if !hyphen.is_visible() {
                        hyphen.set_visible(true);
                    }
                    player_name_box.set_sensitive(true)
                }),
            );

            self.obj().connect_unselect_team(
                clone!(@weak team_name, @weak group_name, @weak hyphen, @weak player_name_box => move |_| {
                    team_name.set_label("");
                    group_name.set_label("");
                    hyphen.set_visible(false);
                    player_name_box.set_sensitive(false);
                }),
            );

            team_name_box.append(&team_name);
            team_name_box.append(&hyphen);
            team_name_box.append(&group_name);

            team_name_center.set_center_widget(Some(&team_name_box));
            player_name_box.append(&team_name_center);

            let player_name_list =
                FixIndexedList::<EntryBuffer, "FixIndexedList_EntryBuffer", "FixIndexedListEntry_EntryBuffer">::with_default_objects(
                    self.player_name_buffer.clone(),
                    clone!(@weak self as this => @default-panic, move |_, position, buffer| {
                        this.create_player_row(position, buffer)
                    }),
                );

            player_name_box.append(&player_name_list);
            player_name_box.into()
        }

        ///
        /// Creates the drop down menu to selected a group to filter for.
        /// Also provides an option to select all groups.
        ///
        fn create_group_selector(&self) -> DropDown {
            // TODO: Maybe replace by list of checkboxes to allow the user to select more than one group, but not all.

            self.setup_group_model();
            let group_selector = DropDown::builder().model(&self.group_model).build();

            let group_selector_factory = {
                let factory = SignalListItemFactory::new();

                factory.connect_setup(clone!(@weak group_selector => move|_, list_item| {
                    let center_box = CenterBox::new();
                    center_box.set_start_widget(Some(&Label::new(None)));
                    center_box.set_end_widget(Some(&Image::new()));
                    list_item.set_child(Some(&center_box));

                    let signal_handler_id = group_selector.connect_selected_notify(clone!(@weak list_item => move |group_selector| {
                        // set check mark icon if selected and mark row as selected with css class
                        let center_box: CenterBox = list_item.child().and_downcast().unwrap();
                        let icon : Image = center_box.end_widget().and_downcast().unwrap();
                        let parent = center_box.parent().unwrap();
                        if list_item.position() == group_selector.selected() {
                            parent.add_css_class("selected");
                            icon.set_icon_name(Some("object-select-symbolic"));
                        } else {
                            parent.remove_css_class("selected");
                            icon.set_icon_name(None);
                        }
                    }));

                    // keep signal handler id to disconnect it later
                    unsafe {
                        list_item.set_data("signal-handler-id", signal_handler_id.as_raw());
                    }
                }));

                factory.connect_bind(clone!(@weak group_selector => move |_, list_item| {
                    let center_box: CenterBox = list_item.child().and_downcast().unwrap();
                    let label: Label = center_box.start_widget().and_downcast().unwrap();
                    let group: StringObject = list_item.item().and_downcast().unwrap();
                    label.set_label(&group.string());
                }));

                factory.connect_teardown(clone!(@weak group_selector => move |_, list_item| {
                    // retrieve signal handler id and disconnect signal
                    let data_opt = unsafe {list_item.data("signal-handler-id")};
                    let data = data_opt.unwrap();
                    let signal_handler_id = unsafe { SignalHandlerId::from_glib(*data.as_ptr()) };
                    group_selector.disconnect(signal_handler_id);
                }));

                factory
            };

            group_selector.set_list_factory(Some(&group_selector_factory));

            group_selector
        }

        /// Creates a new row for a player name, represented by `buffer`.
        fn create_player_row(&self, position: u32, buffer: &EntryBuffer) -> Widget {
            let entry = Entry::builder()
                .buffer(buffer)
                .halign(Align::Center)
                .hexpand(true)
                .xalign(0.5)
                .placeholder_text(format!("Player {}", position + 1))
                .build();
            entry.set_max_width_chars(1000);

            entry.connect_text_notify(clone!(@weak self as this => move |entry| {
                let mut erroneous_entries = this.erroneous_buffer.borrow_mut();
                // show error when disallowed characters are entered
                if !entry.text().chars().all(|c| is_valid_name_character(c)) {
                    if !entry.css_classes().contains(&"error".into()) {
                        entry.add_css_class("error");
                        entry.error_bell();
                    }
                    erroneous_entries.insert(position);
                    drop(erroneous_entries);
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                } else if erroneous_entries.contains(&position)  {
                    // text is valid, reset possibly set error marker
                    erroneous_entries.remove(&position);
                    entry.remove_css_class("error");
                    drop(erroneous_entries);
                    this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                }
            }));

            entry.into()
        }

        fn are_all_entries_valid(&self) -> bool {
            self.erroneous_buffer.borrow().len() == 0
        }

        ///
        /// Setup the model for the team names.
        /// The entries are GroupTeamObjects, so the group name is also provided.
        ///
        /// Filters the entries by the selected group name and the entered string in the search entry.
        ///
        /// Requires `search_entry`, `group_selector` and `list_box` to connect signals.
        ///
        fn setup_team_model(&self, search_entry: &SearchEntry, list_box: &ListBox, group_selector: &DropDown) -> FilterListModel {
            // filter by team names, i.e. by what was typed in the search entry
            let team_name_filter = {
                let team_name_expression = PropertyExpression::new(GroupTeamObject::static_type(), Expression::NONE, "team-name");
                let team_name_filter = StringFilter::builder()
                    .expression(team_name_expression)
                    .match_mode(gtk4::StringFilterMatchMode::Substring)
                    .search("")
                    .build();

                // if the entered search changes, update the filter
                search_entry.connect_search_changed(clone!(@weak team_name_filter => move |entry| {
                    team_name_filter.set_search(Some(entry.text().as_str()));
                }));
                team_name_filter
            };

            // filter by group name, i.e. the selected group in the drop down menu
            // let all entries through if all groups were selected
            let group_name_filter = {
                let group_name_expression = PropertyExpression::new(GroupTeamObject::static_type(), Expression::NONE, "group-name");
                let closure_expression = ClosureExpression::new::<bool>(
                    [group_name_expression],
                    closure_local!(@weak-allow-none self as this_opt => move |_: GroupTeamObject, group_name: GString| {
                        if let Some(this) = this_opt {
                            let sel_group_borrow = this.selected_group.borrow();
                            if let Some(sel_group_name) = sel_group_borrow.as_ref() {
                                sel_group_name.to_string() == group_name
                            } else {
                                true
                            }
                        } else {
                            false
                        }
                    }),
                );

                let group_name_filter = BoolFilter::new(Some(closure_expression));

                group_selector.connect_selected_item_notify(clone!(@weak self as this, @weak group_name_filter => move |group_selector| {
                    // update the stored selected group, store none if all groups are selected
                    if let Some(group) = group_selector.selected_item().and_downcast::<StringObject>() {
                        let mut sel_group = this.selected_group.borrow_mut();
                        if group.string() == Self::ALL_GROUPS_SELECTOR_STR {
                            *sel_group = None;
                        } else {
                            *sel_group = Some(group.string().into());
                        }
                    }

                    // inform the filter that the filter criteria have changed
                    group_name_filter.changed(gtk4::FilterChange::Different);
                }));

                group_name_filter
            };

            // combine filter
            let filter = EveryFilter::new();
            filter.append(team_name_filter.clone());
            filter.append(group_name_filter);

            let model = FilterListModel::new(Some(self.team_model.clone()), Some(filter.clone()));

            // when the filter has changed, check whether the previously selected row passes through the filter and mark it as selected
            filter.connect_changed(clone!(@weak self as this, @weak filter, @weak list_box, @weak model => move |_, _| {
                let selected_team_obj = this.selected_team_obj.borrow();
                if let Some(team) = selected_team_obj.as_ref() {
                    // if selected team currently passes the filter, select its row in the ListBox
                    if filter.match_(team) {
                        // find index of team in model
                        let mut team_idx = None;
                        for idx in 0..model.n_items() {
                            let model_item = model.item(idx).and_downcast::<GroupTeamObject>().unwrap();
                            if &model_item == team {
                                team_idx = Some(idx as i32);
                            }
                        };

                        if let Some(idx) = team_idx {
                            // find row and select row
                            let row = list_box.row_at_index(idx).unwrap();
                            list_box.select_row(Some(&row));
                        }
                    }
                }
            }));

            model
        }

        fn setup_group_model(&self) {
            // insert select all groups marker
            self.group_model.append(Self::ALL_GROUPS_SELECTOR_STR);
        }

        pub fn reload(&self) {
            self.clear_player_name_entries();
            let selected_team_obj = self.selected_team_obj.borrow_mut().take();
            self.obj().emit_unselect_team();

            // remove all items from team model
            for idx in (0..self.team_model.n_items()).rev() {
                self.team_model.remove(idx);
            }

            // remove all items from group model (except all groups selector)
            for idx in (1..self.group_model.n_items()).rev() {
                self.group_model.remove(idx);
            }

            // and insert new teams & groups
            let new_competition_data = self.new_competition_data.borrow();
            let data = new_competition_data.borrow();
            let mut found_sel_team_obj = None;
            for group in data.groups.iter() {
                for team in group.teams.iter() {
                    let group_team_obj = GroupTeamObject::new(group.name.clone(), team.name.clone());
                    self.team_model.append(&group_team_obj);

                    // select team obj again
                    if let Some(prev_sel_team_obj) = selected_team_obj.as_ref() {
                        if prev_sel_team_obj.group_name() == group.name && prev_sel_team_obj.team_name() == team.name {
                            debug_assert!(found_sel_team_obj.is_none());
                            found_sel_team_obj = Some(group_team_obj);
                        }
                    }
                }
            }

            for group in data.groups.iter() {
                self.group_model.append(&group.name);
            }

            // avoid double borrow of data
            drop(data);
            if let Some(sel_team) = found_sel_team_obj.as_ref() {
                self.select_team(sel_team);
            }
        }

        pub fn set_competition_data(&self, data: Rc<RefCell<CompetitionData>>) {
            *self.new_competition_data.borrow_mut() = data;
        }

        #[cfg(debug_assertions)]
        fn add_test_values_key_binding(&self) {
            use gdk4::{Key, ModifierType};
            use gtk4::{EventControllerKey, glib::Propagation};

            let key_event_controller = EventControllerKey::new();
            key_event_controller.connect_key_pressed(
                clone!(@weak self as this => @default-panic, move |_ :&EventControllerKey, _/*key*/: Key, key_code: u32, modifier_type : ModifierType| {
                    if key_code == 28 && modifier_type.contains(ModifierType::CONTROL_MASK) {
                        println!("Adding test data to TeamInformationScreen!");
                        this.erroneous_buffer.borrow_mut().clear();

                        let new_competition_data = this.new_competition_data.borrow();
                        let mut data = new_competition_data.borrow_mut();

                        for group in &mut data.groups {
                            for team in &mut group.teams {
                                for i in 0..6 {
                                    team.player_names[i] = Some(format!("{} - Player {}", team.name, i + 1));
                                }
                            }
                        }
                        
                        // avoid double borrow
                        drop(data);

                        this.reload();
                        this.obj().emit_all_entries_valid(this.are_all_entries_valid());
                        return Propagation::Stop
                    }

                    Propagation::Proceed
                }),
            );
            self.obj().add_controller(key_event_controller);
        }
    }
}

glib::wrapper! {
    pub struct TeamInformationScreen(ObjectSubclass<inner::TeamInformationScreen>)
        @extends Widget;
}

impl TeamInformationScreen {
    pub fn new(competition_data: &Rc<RefCell<CompetitionData>>) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.imp().set_competition_data(Rc::clone(competition_data));
        obj
    }

    #[inline]
    fn connect_selected_team<F: Fn(&Self, String, String) + 'static>(&self, f: F) {
        self.connect_closure(
            "selected-team",
            true,
            closure_local!(move |team_info_screen: &Self, selected_team: String, selected_group_name: String| {
                f(team_info_screen, selected_team, selected_group_name);
            }),
        );
    }

    #[inline]
    fn connect_unselect_team<F: Fn(&Self) + 'static>(&self, f: F) {
        self.connect_closure(
            "unselect-team",
            true,
            closure_local!(move |team_info_screen: &Self| { f(team_info_screen) }),
        );
    }

    #[inline]
    pub fn connect_all_entries_valid<F: Fn(&Self, bool) + 'static>(&self, f: F) {
        self.connect_closure(
            "all-entries-valid",
            true,
            closure_local!(move |base_info_screen: &Self, all_entries_valid: bool| {
                f(base_info_screen, all_entries_valid);
            }),
        );
    }

    #[inline]
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
    /// Emits a signal that `selected_team` from `selected_group_name` was selected.
    ///
    #[inline]
    fn emit_selected_team(&self, selected_team: String, selected_group_name: String) {
        let _: () = self.emit_by_name("selected-team", &[&selected_team, &selected_group_name]);
    }

    #[inline]
    fn emit_unselect_team(&self) {
        let _: () = self.emit_by_name("unselect-team", &[]);
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

    pub fn reload(&self) {
        self.imp().reload();
    }
}
