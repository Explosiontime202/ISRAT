use super::group_screen::GroupScreen;
use crate::data::MatchResult;
use crate::widgets::{table::Table, tile::Tile};
use crate::{CompetitionPtr, ProgramState};
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{glib, prelude::*, subclass::widget::*, BoxLayout, FlowBox, Label, LayoutManager, Orientation, ScrolledWindow, Widget};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct MatchHistoryScreen {
        /// the title of the screen: the group name
        title: Label,
        /// the flowbox holding all tiles
        flow_box: FlowBox,
        scrolled_window: ScrolledWindow,
        /// references to the tiles displaying the tables
        tiles: RefCell<Vec<Tile>>,
        /// a option to the competition, in a RefCell to allow interior mutability
        data: RefCell<Option<CompetitionPtr>>,
        /// the index of the currently displayed group
        group_idx: Cell<u32>,
    }

    impl Default for MatchHistoryScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MatchHistoryScreen {
        const NAME: &'static str = "MatchHistoryScreen";
        type Type = super::MatchHistoryScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<BoxLayout>();
            klass.set_css_name("match_history");
        }
    }

    impl ObjectImpl for MatchHistoryScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.title.set_parent(&*obj);
            self.scrolled_window.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.title.unparent();
            self.scrolled_window.unparent();
        }
    }

    impl WidgetImpl for MatchHistoryScreen {
        fn show(&self) {
            self.reload();
        }
    }

    impl MatchHistoryScreen {
        fn new() -> Self {
            let title = Label::builder().label("").css_classes(["headline"]).build();
            let flow_box = FlowBox::builder()
                .max_children_per_line(2)
                .min_children_per_line(2)
                .orientation(gtk4::Orientation::Horizontal)
                .selection_mode(gtk4::SelectionMode::None)
                .homogeneous(true)
                .build();
            let scrolled_window = ScrolledWindow::builder().child(&flow_box).vexpand(true).build();

            Self {
                title,
                flow_box,
                scrolled_window,
                tiles: RefCell::default(),
                data: RefCell::new(None),
                group_idx: Cell::new(0),
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
            self.group_idx.set(group_idx);
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

            let group_idx: usize = self.group_idx.get() as usize;
            let competition_borrow = self.data.borrow_mut();
            let competition = competition_borrow.as_ref().unwrap().read().unwrap();

            debug_assert!(competition.data.is_some());
            let data = competition.data.as_ref().unwrap();
            let team_names = &data.teams[group_idx];

            // use possibly new group name
            self.title.set_label(data.group_names[group_idx].as_str());

            let mut tiles = self.tiles.borrow_mut();

            // remove old tiles
            for tile in tiles.iter() {
                self.flow_box.remove(tile);
            }
            tiles.clear();

            // create new tiles for each batch
            for batch_idx in 0..data.get_current_batch(group_idx) {
                let mut matches = data.get_batch_for_group(group_idx, batch_idx);
                // make sure we display the matches in order
                matches.sort_unstable_by(|match_a, match_b| match_a.lane.cmp(&match_b.lane));
                let table = Table::with_expanding_column(
                    vec!["Lane", "Team A", "Team B", "Result A", "Result B"],
                    vec![false, true, true, false, false],
                );

                // add each match of the batch to the table
                for _match in matches {
                    if _match.result == MatchResult::Break {
                        continue;
                    }

                    let lane_str = (_match.lane + 1).to_string();
                    let team_name_a = team_names[_match.team_a].name.as_str();
                    let team_name_b = team_names[_match.team_b].name.as_str();
                    let result_a_str = _match.points.unwrap()[0].to_string();
                    let result_b_str = _match.points.unwrap()[1].to_string();
                    table.add_row(vec![&lane_str, team_name_a, team_name_b, &result_a_str, &result_b_str]);
                }

                // put widgets together
                let batch_str = format!("Batch {}", batch_idx + 1);
                let tile = Tile::new(batch_str.as_str());
                tile.set_child(table);
                self.flow_box.insert(&tile, -1);
                tiles.push(tile);
            }
        }
    }
}

glib::wrapper! {
    pub struct MatchHistoryScreen(ObjectSubclass<inner::MatchHistoryScreen>)
        @extends Widget;
}

impl MatchHistoryScreen {
    pub fn new(program_state: &Rc<ProgramState>) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.imp().set_data(Arc::clone(&program_state.competition));
        obj.set_hexpand(true);
        obj
    }
}

impl GroupScreen for MatchHistoryScreen {
    ///
    /// Set the index of the shown group and reload the widget.
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
