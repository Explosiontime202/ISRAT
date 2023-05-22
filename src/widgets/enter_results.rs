use crate::data::MatchID;
use crate::widgets::{table::Table, tile::Tile};
use crate::{CompetitionPtr, ProgramState};
use gdk4::glib::clone;
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{
    glib, subclass::widget::*, traits::*, Box as GtkBox, BoxLayout, Button, CenterBox, Dialog, Entry, Label, LayoutManager, Orientation, Widget,
    Window,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::rc::Rc;
use std::sync::Arc;

use super::group_screen::GroupScreen;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct EnterResultScreen {
        /// the title of the screen: the group name
        title: Label,
        /// a reference to the tile parenting the enter results table
        enter_tile: Tile,
        /// a reference to the box holding the enter result table and the submit button
        enter_tile_box: GtkBox,
        /// a reference to the table containing the enter result table
        enter_table: Table,
        /// a option to the competition, in a RefCell to allow interior mutability
        data: RefCell<Option<CompetitionPtr>>,
        /// the index of the currently displayed group
        group_idx: RefCell<u32>,
        /// references to the text entries
        entries: RefCell<HashMap<MatchID, [Entry; 2]>>,
    }

    impl Default for EnterResultScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EnterResultScreen {
        const NAME: &'static str = "EnterResultScreen";
        type Type = super::EnterResultScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<BoxLayout>();
            klass.set_css_name("enter_results");
        }
    }

    impl ObjectImpl for EnterResultScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.title.set_parent(&*obj);

            self.enter_tile_box.append(&self.enter_table);

            let submit_button = Button::builder().label("Submit results").css_name("highlighted_button").build();
            let submit_button_center_widget = CenterBox::new();
            submit_button_center_widget.set_end_widget(Some(&submit_button));
            self.enter_tile_box.append(&submit_button_center_widget);

            submit_button.connect_clicked(clone!(@weak self as this => move |_| {
                {
                    let entries = this.entries.borrow();
                    let entered_results : Vec<(MatchID, [Result<u32, ParseIntError>; 2])> = entries.iter().map(|(&idx, [entry_a, entry_b])| (idx, [entry_a.text().parse::<u32>(), entry_b.text().parse::<u32>()])).collect();
                    let erroneous_inputs : Vec<(MatchID, bool, bool)> =  entered_results.iter().filter_map(|(id, [res_a, res_b])| {
                        if res_a.is_err() || res_b.is_err() {
                            Some((*id, res_a.is_err(), res_b.is_err()))
                        } else {
                            None
                        }
                    }).collect();

                    if erroneous_inputs.is_empty() {
                        // all inputs are valid, we can forward the entered values to the data model

                        let match_results : HashMap<MatchID, [u32; 2]>= entered_results.iter().map(|(id, [res_a, res_b])| (*id, [*res_a.as_ref().unwrap(), *res_b.as_ref().unwrap()])).collect();
                        let group_idx = *this.group_idx.borrow();
                        let data_ptr = this.data.borrow();
                        let competition = &mut *data_ptr.as_ref().unwrap().write().unwrap();

                        match competition.data.as_mut() {
                            Some(data) => data.enter_match_results(group_idx as usize, match_results),
                            None => (),
                        };


                        // reset entry input fields
                        entries.iter().flat_map(|(_, entries)| entries).for_each(|entry| entry.set_text(""));
                    } else {
                        // the user did enter some non-digit values => show an error
                        // TODO: Show in dialog which inputs are erroneous
                        let error_text = "You entered non-digit values in some fields.\nPlease enter numbers in order to submit the results.";
                        let dialog_child = Label::builder().label(error_text).justify(gtk4::Justification::Center).build();
                        let popup = Dialog::builder().child(&dialog_child).modal(true).resizable(false).build();
                        let root_widget = this.obj().root().unwrap().downcast::<Window>().unwrap();
                        popup.set_transient_for(Some(&root_widget));
                        popup.show();
                    }
                }
                this.reload();
            }));

            self.enter_tile.set_child(self.enter_tile_box.clone());
            self.enter_tile.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.title.unparent();
            self.enter_tile.unparent();
        }
    }

    impl WidgetImpl for EnterResultScreen {
        fn show(&self) {
            self.reload();
        }
    }

    impl EnterResultScreen {
        fn new() -> Self {
            let title = Label::builder().label("").css_classes(["headline"]).build();
            let enter_tile = Tile::new("");

            let enter_table = Table::with_expanding_column(
                vec!["Lane", "Team A", "Team B", "Result A", "Result B"],
                vec![false, true, true, false, false],
            );
            let enter_tile_box = GtkBox::new(Orientation::Vertical, 10);

            Self {
                title,
                enter_tile,
                enter_table,
                enter_tile_box,
                data: RefCell::new(None),
                group_idx: RefCell::new(0),
                entries: RefCell::default(),
            }
        }

        ///
        /// Updates the pointer to the competition.
        ///
        pub fn set_data(&self, data: CompetitionPtr) {
            *self.data.borrow_mut() = Some(data);
        }

        ///
        /// Sets the group idx to `group_idx`.
        /// Also updates the child widgets accordingly.
        ///
        pub fn set_group_idx(&self, group_idx: u32) {
            *self.group_idx.borrow_mut() = group_idx;
            self.reload();
        }

        ///
        /// Reloads the data from the competition pointer and updates the widgets accordingly.
        ///
        pub fn reload(&self) {
            if self.data.borrow().is_none() {
                debug_assert!(false);
                return;
            }

            let mut entries = self.entries.borrow_mut();
            entries.clear();

            let group_idx: usize = *self.group_idx.borrow() as usize;
            let competition_borrow = self.data.borrow_mut();
            let competition = competition_borrow.as_ref().unwrap().read().unwrap();

            debug_assert!(competition.data.is_some());
            let data = competition.data.as_ref().unwrap();

            // use possibly new group name
            self.title.set_label(data.group_names[group_idx].as_str());

            let teams = &data.teams[group_idx];

            let next_matches = data.next_matches_for_group(group_idx);

            if next_matches.is_empty() {
                // no more matches available
                let text = Label::new(Some("All results have been entered."));
                self.enter_tile.set_child(text);
                return;
            }

            self.enter_tile.set_child(self.enter_tile_box.clone());

            // update title of interim result table
            let new_enter_result_title = format!("Enter results for batch {}", data.current_batch[group_idx] + 1);
            self.enter_tile.set_title(new_enter_result_title.as_str());

            // rebuild enter results table
            self.enter_table.clear_rows();
            for next_match in next_matches {
                let lane_str = (next_match.lane + 1).to_string();
                let team_a_name = teams[next_match.team_a].name.as_str();
                let team_b_name = teams[next_match.team_b].name.as_str();

                let team_a_entry = Self::create_new_entry();
                let team_b_entry = Self::create_new_entry();
                entries.insert(next_match.id, [team_a_entry.clone(), team_b_entry.clone()]);

                self.enter_table.add_widget_row(vec![
                    Label::new(Some(lane_str.as_str())).into(),
                    Label::builder().label(team_a_name).hexpand(true).build().into(),
                    Label::builder().label(team_b_name).hexpand(true).build().into(),
                    team_a_entry.into(),
                    team_b_entry.into(),
                ])
            }
        }

        fn create_new_entry() -> Entry {
            let entry = Entry::new();
            entry.connect_text_notify(|entry| {
                if !entry.text().chars().all(|c| c.is_ascii_digit()) {
                    // text contains invalid character, i.e. non-digit, set error marker
                    if !entry.css_classes().contains(&"error".into()) {
                        entry.error_bell();
                    }
                    entry.add_css_class("error");
                } else {
                    // text is valid, reset error marker
                    entry.remove_css_class("error");
                }
            });
            entry
        }
    }
}

glib::wrapper! {
    pub struct EnterResultScreen(ObjectSubclass<inner::EnterResultScreen>)
        @extends Widget;
}

impl EnterResultScreen {
    pub fn new(program_state: &Rc<ProgramState>) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.imp().set_data(Arc::clone(&program_state.competition));
        obj.set_hexpand(true);
        obj
    }
}

impl GroupScreen for EnterResultScreen {
    ///
    /// Display the enter results screen for the group with index `group_idx`.
    ///
    fn show_group(&self, group_idx: u32) {
        self.imp().set_group_idx(group_idx);
    }

    ///
    /// Reload the widget from the data pointer.
    ///
    fn reload(&self) {
        self.imp().reload();
    }
}
