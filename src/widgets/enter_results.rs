use crate::widgets::{table::Table, tile::Tile};
use crate::CompetitionPtr;
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{
    glib, subclass::widget::*, traits::*, BoxLayout, Button, Entry, Label, LayoutManager, CenterBox, Box as GtkBox,
    Orientation, Widget
};
use std::cell::RefCell;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct EnterResultScreen {
        // the title of the screen: the group name
        title: Label,
        // a reference to the tile parenting the enter results table
        enter_tile: Tile,
        // a reference to the table containing the enter result table
        enter_table: Table,
        // a option to the competition, in a RefCell to allow interior mutability
        data: RefCell<Option<CompetitionPtr>>,
        // the index of the currently displayed group
        group_idx: RefCell<u32>,
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
            klass.set_css_name("group_overview");
        }
    }

    impl ObjectImpl for EnterResultScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.title.set_parent(&*obj);

            let enter_tile_box = GtkBox::new(Orientation::Vertical, 10);
            enter_tile_box.append(&self.enter_table);

            let submit_button = Button::builder().label("Submit results").css_name("highlighted_button").build();
            let submit_button_center_widget = CenterBox::new();
            submit_button_center_widget.set_end_widget(Some(&submit_button));
            enter_tile_box.append(&submit_button_center_widget);

            self.enter_tile.set_child(enter_tile_box);
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

            Self {
                title,
                enter_tile,
                enter_table,
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
        fn reload(&self) {
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
            self.title
                .set_label(data.group_names.as_ref().unwrap()[group_idx].as_str());

            let teams = &data.teams.as_ref().unwrap()[group_idx];

            // update title of interim result table
            let new_enter_result_title = format!(
                "Enter results for batch {}",
                data.current_batch[group_idx] + 1
            );
            self.enter_tile.set_title(new_enter_result_title.as_str());

            let next_matches = data.next_matches_for_group(group_idx);

            // rebuild enter results table
            self.enter_table.clear_rows();
            for next_match in next_matches {
                let lane_str = (next_match.lane + 1).to_string();
                let team_a_name = teams[next_match.team_a].name.as_str();
                let team_b_name = teams[next_match.team_b].name.as_str();

                self.enter_table.add_widget_row(vec![
                    Label::new(Some(lane_str.as_str())).into(),
                    Label::builder()
                        .label(team_a_name)
                        .hexpand(true)
                        .build()
                        .into(),
                    Label::builder()
                        .label(team_b_name)
                        .hexpand(true)
                        .build()
                        .into(),
                    Entry::new().into(),
                    Entry::new().into(),
                ])
            }
        }
    }
}

glib::wrapper! {
    pub struct EnterResultScreen(ObjectSubclass<inner::EnterResultScreen>)
        @extends Widget;
}

impl EnterResultScreen {
    pub fn new(competition: CompetitionPtr) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.imp().set_data(competition);
        obj.set_hexpand(true);
        obj
    }

    ///
    /// Display the enter results screen for the group with index `group_idx`.
    ///
    pub fn show_group(&self, group_idx: u32) {
        self.imp().set_group_idx(group_idx);
    }
}
