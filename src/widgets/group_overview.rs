use crate::widgets::{table::Table, tile::Tile};
use crate::CompetitionPtr;
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{glib, subclass::widget::*, traits::*, Box as GtkBox, FlowBox, Label, LayoutManager, Orientation, Widget};
use std::cell::RefCell;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct GroupOverviewScreen {
        /// the flow box storing the displayed tiles
        flow_box: FlowBox,
        /// the title of the screen: the group name
        title: Label,
        /// a reference to the tile holding the interim result table
        interim_result_tile: Tile,
        /// a reference to the table displaying the interim result
        interim_result_table: Table,
        /// a reference to the table displaying the next matches
        next_matches_table: Table,
        /// a reference to the label displaying the teams on break
        break_label: Label,
        /// a option to the competition, in a RefCell to allow interior mutability
        data: RefCell<Option<CompetitionPtr>>,
        /// the index of the currently displayed group
        group_idx: RefCell<u32>,
    }

    impl Default for GroupOverviewScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GroupOverviewScreen {
        const NAME: &'static str = "GroupOverviewScreen";
        type Type = super::GroupOverviewScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("group_overview");
        }
    }

    impl ObjectImpl for GroupOverviewScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            self.interim_result_tile.set_child(self.interim_result_table.clone());
            self.flow_box.insert(&self.interim_result_tile, -1);

            let next_matches_tile = Tile::new("Next Matches");
            let next_matches_box = GtkBox::new(Orientation::Vertical, 20);
            next_matches_box.append(&self.next_matches_table);
            next_matches_box.append(&self.break_label);
            next_matches_tile.set_child(next_matches_box);
            self.flow_box.insert(&next_matches_tile, -1);

            self.title.set_parent(&*obj);
            self.flow_box.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.title.unparent();
            self.flow_box.unparent();
        }
    }

    impl WidgetImpl for GroupOverviewScreen {
        fn show(&self) {
            self.reload();
        }
    }

    impl GroupOverviewScreen {
        fn new() -> Self {
            let flow_box = FlowBox::builder()
                .max_children_per_line(2)
                .min_children_per_line(2)
                .orientation(gtk4::Orientation::Horizontal)
                .selection_mode(gtk4::SelectionMode::None)
                .homogeneous(true)
                .build();

            let title = Label::builder().label("").css_classes(["headline"]).build();

            let interim_result_tile = Tile::new("");

            let next_matches_table = Table::with_expanding_column(vec!["Lane", "Team A", "Team B"], vec![false, true, true]);
            let interim_result_table = Table::with_expanding_column(
                vec!["Place", "Team", "Points", "Difference", "Stockpoints"],
                vec![false, true, false, false, false],
            );

            let break_label = Label::new(None);

            Self {
                flow_box,
                title,
                interim_result_tile,
                interim_result_table,
                next_matches_table,
                break_label,
                data: RefCell::default(),
                group_idx: RefCell::new(0),
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

            let group_idx: usize = *self.group_idx.borrow() as usize;
            let competition_borrow = self.data.borrow_mut();
            let competition = competition_borrow.as_ref().unwrap().borrow();

            debug_assert!(competition.data.is_some());
            let data = competition.data.as_ref().unwrap();

            // use possibly new group name
            self.title.set_label(data.group_names.as_ref().unwrap()[group_idx].as_str());

            // calculate current interim result
            let interim_result = data.calc_interim_result_for_group(group_idx);
            let teams = &data.teams.as_ref().unwrap()[group_idx];

            // update title of interim result table
            let new_interim_result_title = if data.current_batch[group_idx] == 0 {
                "Starter list".to_string()
            } else {
                format!("Interim result after Batch {}", data.current_batch[group_idx])
            };
            self.interim_result_tile.set_title(new_interim_result_title.as_str());

            // rebuild interim result table
            self.interim_result_table.clear_rows();
            for (entry_idx, interim_result_entry) in interim_result.iter().enumerate() {
                // create displayable strings from interim result entry
                let entry_idx_str = (entry_idx + 1).to_string();
                let points_str = format!("{} : {}", interim_result_entry.match_points[0], interim_result_entry.match_points[1]);
                let difference_str = if interim_result_entry.difference == 0 {
                    "0".to_string()
                } else {
                    format!("{:+}", interim_result_entry.difference)
                };
                let stock_points_str = format!("{} : {}", interim_result_entry.stock_points[0], interim_result_entry.stock_points[1]);

                // display interim result entry in table
                self.interim_result_table.add_row(vec![
                    entry_idx_str.as_str(),
                    teams[interim_result_entry.team_idx].name.as_str(),
                    points_str.as_str(),
                    difference_str.as_str(),
                    stock_points_str.as_str(),
                ]);
            }

            // rebuild next matches table
            let next_matches = data.next_matches_for_group(group_idx);
            self.next_matches_table.clear_rows();
            for next_match in next_matches {
                let lane_str = (next_match.lane + 1).to_string();
                self.next_matches_table.add_row(vec![
                    lane_str.as_str(),
                    teams[next_match.team_a].name.as_str(),
                    teams[next_match.team_b].name.as_str(),
                ]);
            }

            // update teams on break
            // hide label if no team is on break
            let break_teams = data.teams_on_break_for_group(group_idx);
            if break_teams.is_empty() {
                self.break_label.set_visible(false);
            } else {
                let break_team_names = break_teams.iter().map(|&team| team.name.as_str()).collect::<Vec<&str>>();

                let break_str = format!("Break: {}", break_team_names.join(", "));
                self.break_label.set_label(break_str.as_str());
                self.break_label.set_visible(true);
            }
        }
    }
}

glib::wrapper! {
    pub struct GroupOverviewScreen(ObjectSubclass<inner::GroupOverviewScreen>)
        @extends Widget;
}

impl GroupOverviewScreen {
    pub fn new(competition: CompetitionPtr) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.imp().set_data(competition);
        obj.set_hexpand(true);
        obj
    }

    ///
    /// Display the overview for the group with index `group_idx`.
    ///
    pub fn show_group(&self, group_idx: u32) {
        self.imp().set_group_idx(group_idx);
    }

    ///
    /// Reload the widget from the data pointer.
    /// 
    pub fn reload(&self) {
        self.imp().reload();
    }
}
