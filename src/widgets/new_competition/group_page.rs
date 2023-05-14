use super::base_information::is_valid_name_character;
use crate::widgets::fix_indexed_list::FixIndexedList;
use gdk4::{
    glib::{clone, closure_local, once_cell::sync::Lazy, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
};
use gtk4::{glib, prelude::*, subclass::widget::*, Align, BoxLayout, Entry, EntryBuffer, Label, LayoutManager, Orientation, Widget};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct GroupPage {
        /// The  FixIndexedListBox displaying the rows. (a direct child)
        team_name_list: FixIndexedList<EntryBuffer>,
        /// A counter used to generate generic default team names. Only incremented.
        team_counter: Cell<u32>,
        /// The entry buffer which are currently in an invalid state, i.e. invalid chars were entered.
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

            self.team_name_list
                .connect_create_data_widget(clone!(@weak self as this => @default-panic, move |_, buffer| {
                    this.create_entry(buffer)
                }));

            self.team_name_list
                .connect_create_data_object(clone!(@weak self as this => @default-panic, move |_| {
                    let team_counter = this.team_counter.get() + 1;
                    this.team_counter.set(team_counter);
                    let generic_team_name = format!("Team {team_counter}");
                    let buffer = EntryBuffer::new(Some(generic_team_name));
                    buffer
                }));

            self.team_name_list.connect_row_removed(clone!(@weak self as this => move |_, buffer| {
                this.erroneous_entries.borrow_mut().remove(buffer);
                this.obj().emit_all_entries_valid(this.are_all_entries_valid());
            }));

            self.team_name_list.connect_create_append_widget(|_| Label::new(Some("Add Team")).into());

            self.team_name_list.set_allow_count_changes(true);
            self.team_name_list.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.team_name_list.unparent();
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
                team_name_list: FixIndexedList::new(),
                team_counter: Cell::new(0),
                erroneous_entries: RefCell::default(),
            }
        }

        fn create_entry(&self, entry: &EntryBuffer) -> Widget {
            let entry = Entry::builder().buffer(entry).halign(Align::Center).hexpand(true).xalign(0.5).build();

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

            entry.into()
        }

        fn are_all_entries_valid(&self) -> bool {
            self.erroneous_entries.borrow().is_empty()
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
