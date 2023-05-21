use crate::widgets::{fix_indexed_list::FixIndexedList, new_competition::group_team_object::GroupTeamObject, tile::Tile};
use gdk4::{
    gio::ListStore,
    glib::GString,
    glib::{clone, closure_local, once_cell::sync::Lazy, subclass::Signal, translate::FromGlib, SignalHandlerId},
    prelude::*,
    subclass::prelude::*,
};
use gtk4::{
    glib, subclass::widget::*, traits::*, Align, BoolFilter, Box as GtkBox, CenterBox, ClosureExpression, DropDown, Entry, EntryBuffer, EveryFilter,
    Expression, FilterListModel, Image, Label, ListBox, Paned, PropertyExpression, SearchEntry, SignalListItemFactory, StringFilter, StringList,
    StringObject, Widget,
};
use std::cell::RefCell;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct TeamInformationScreen {
        // The main child containing all the content. (a direct child)
        tile: Tile,
        player_name_buffer: Vec<EntryBuffer>,
        selected_team_obj: RefCell<Option<GroupTeamObject>>,
        selected_group: RefCell<Option<String>>,
    }

    impl Default for TeamInformationScreen {
        fn default() -> Self {
            let player_name_buffer: Vec<EntryBuffer> = (0..6).map(|_| EntryBuffer::new(None::<&str>)).collect();

            Self {
                tile: Tile::new("Team Information"),
                player_name_buffer,
                selected_team_obj: RefCell::new(None),
                selected_group: RefCell::new(None),
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
            self.tile.set_parent(&*self.obj());
            self.tile.set_hexpand(true);

            let paned = Paned::builder()
                .orientation(gtk4::Orientation::Horizontal)
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
        }

        fn dispose(&self) {
            self.tile.unparent();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("selected-team").param_types([String::static_type()]).build()]);
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for TeamInformationScreen {}

    impl TeamInformationScreen {
        /// The string used to select all groups.
        const ALL_GROUPS_SELECTOR_STR: &str = "All groups";

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
            let model = self.setup_team_model(&search_entry, &team_selector_list, &group_selector);

            team_selector_list.bind_model(
                Some(&model),
                clone!(@weak team_selector_list => @default-panic, move |obj| {
                    let string_obj: &GroupTeamObject = obj.downcast_ref().unwrap();
                    Label::new(Some(&string_obj.team_name())).into()
                    // TODO: Also show group name to distinguish teams with same names in different groups
                }),
            );

            team_selector_list.connect_row_selected(clone!(@weak self as this, @weak model => move |_, row_opt| {
                if let Some(row) = row_opt {
                    let team: GroupTeamObject = model.item(row.index() as u32).and_downcast().unwrap();

                    // only override selected_team_obj if it is currently none or another string is stored
                    if this.selected_team_obj.borrow().is_none() || this.selected_team_obj.borrow().as_ref().unwrap() != &team {
                        *this.selected_team_obj.borrow_mut() = Some(team.clone());
                        this.obj().emit_selected_team(team.team_name().into());
                    }
                }
            }));

            team_selector_box.append(&team_selector_list);

            team_selector_box
        }

        fn create_player_name_box(&self) -> Widget {
            let player_name_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .css_name("player_name_box")
                .build();
            let team_name = Label::new(None);

            self.obj().connect_selected_team(clone!(@weak team_name => move |_, selected_team_name| {
                team_name.set_label(selected_team_name.as_str());
            }));

            player_name_box.append(&team_name);

            let player_name_list = FixIndexedList::<EntryBuffer>::with_default_objects(
                self.player_name_buffer.clone(),
                clone!(@weak self as this => @default-panic, move |_, buffer| {
                    this.create_player_row(buffer)
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

            let group_model = self.setup_group_model();
            let group_selector = DropDown::builder().model(&group_model).build();

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
        fn create_player_row(&self, buffer: &EntryBuffer) -> Widget {
            let entry = Entry::builder().buffer(buffer).halign(Align::Center).hexpand(true).xalign(0.5).build();
            entry.set_max_width_chars(1000);

            // TODO: implement
            /*entry.connect_text_notify(clone!(@weak self as this => move |entry| {
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
            }));*/

            entry.into()
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
            // retrieve the teams with their group and insert them into a ListStore
            let team_names = self.retrieve_team_names();
            let list_store = ListStore::new(GroupTeamObject::static_type());

            for (group_name, team_name) in team_names {
                list_store.append(&GroupTeamObject::new(group_name.into(), team_name.into()))
            }

            // filter by team names, i.e. by what was typed in the search entry
            let team_name_filter = {
                let team_name_expression = PropertyExpression::new(GroupTeamObject::static_type(), Expression::NONE, "team-name");
                let team_name_filter = StringFilter::builder()
                    .expression(team_name_expression)
                    .match_mode(gtk4::StringFilterMatchMode::Prefix)
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
                    let group: StringObject = group_selector.selected_item().and_downcast().unwrap();
                    {
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

            let model = FilterListModel::new(Some(list_store), Some(filter.clone()));

            // when the filter has changed, check whether the previously selected row passes through the filter and mark it as selected
            filter.connect_changed(clone!(@weak self as this, @weak filter, @weak list_box, @weak model => move |_, _| {
                let selected_team_obj = this.selected_team_obj.borrow();
                if let Some(team) = selected_team_obj.as_ref() {
                    // if selected team currently passes the filter, select its row in the ListBox
                    if filter.match_(team) {
                        // find index of team in model
                        let mut team_idx = None;
                        for idx in 0..model.n_items() {
                            if model.item(idx).and_downcast::<GroupTeamObject>().as_ref().unwrap() == team {
                                team_idx = Some(idx as i32);
                            }
                        };

                        assert!(team_idx.is_some());
                        // find row and select row
                        let row = list_box.row_at_index(team_idx.unwrap()).unwrap();
                        list_box.select_row(Some(&row));
                    }
                }
            }));

            model
        }

        fn setup_group_model(&self) -> StringList {
            let mut group_names = self.retrieve_group_names();

            // add possibility to select all groups
            group_names.insert(0, Self::ALL_GROUPS_SELECTOR_STR);

            let string_list = StringList::new(&group_names);
            string_list
        }

        fn retrieve_team_names(&self) -> Vec<(&str, &str)> {
            // TODO: Actually get the real team names
            vec![("Group A", "SV Musterverein"), ("Group B", "TuS Test"), ("Group A", "DJK Pfosten")]
        }

        fn retrieve_group_names(&self) -> Vec<&str> {
            // TODO: Actually get real group names
            vec!["Group A", "Group B"]
        }
    }
}

glib::wrapper! {
    pub struct TeamInformationScreen(ObjectSubclass<inner::TeamInformationScreen>)
        @extends Widget;
}

impl TeamInformationScreen {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }

    #[inline]
    fn connect_selected_team<F: Fn(&Self, String) + 'static>(&self, f: F) {
        self.connect_closure(
            "selected-team",
            true,
            closure_local!(move |team_info_screen: &Self, selected_team: String| {
                f(team_info_screen, selected_team);
            }),
        );
    }

    #[inline]
    fn emit_selected_team(&self, selected_team: String) {
        let _: () = self.emit_by_name("selected-team", &[&selected_team]);
    }
}
