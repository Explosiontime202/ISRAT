use crate::{
    data::CompetitionData,
    widgets::navbar::{NavBar, NavBarCategoryTrait},
    ProgramState,
};
use gdk4::glib::{self, clone};
use gtk4::{traits::*, Application, ApplicationWindow};
use std::{cell::RefCell, rc::Rc};

use self::{base_information::BaseInformationScreen, team_information::TeamInformationScreen};

mod base_information;
mod group_page;
mod group_team_object;
mod team_information;
mod team_region_object;

pub fn create_new_competition_screen(application: &Application, program_state: &Rc<ProgramState>) {
    let window = ApplicationWindow::builder()
        .application(application)
        .default_width(1280)
        .default_height(720)
        .title("Create new Competition")
        .decorated(true)
        .modal(true)
        .build();
    window.present();

    let new_competition: Rc<RefCell<CompetitionData>> = Rc::default();
    // *new_competition.borrow_mut() = program_state.competition.read().unwrap().data.as_ref().unwrap().clone();

    let nav_bar = NavBar::<NewScreenNavBarCategory>::with_use_separators(false);

    // add screens
    let base_information = BaseInformationScreen::new(&new_competition);
    nav_bar.add_child(
        &base_information,
        "Base Information".to_string(),
        NewScreenNavBarCategory::BaseInformation,
    );

    let team_information = TeamInformationScreen::new(&new_competition);
    nav_bar.add_child(
        &team_information,
        "Team Information".to_string(),
        NewScreenNavBarCategory::TeamInformation,
    );

    nav_bar.connect_show_child_before(clone!(@weak team_information => move |_, child| {
        if &team_information == child {
            team_information.reload();
        }
    }));

    // hide TeamInformation by default
    nav_bar.hide_category(NewScreenNavBarCategory::TeamInformation);

    // connect signals to show / hide categories and children
    base_information.connect_next_screen(clone!(@weak nav_bar => move |_| {
        nav_bar.show_child("Team Information", NewScreenNavBarCategory::TeamInformation);
    }));

    base_information.connect_all_entries_valid(clone!(@weak nav_bar => move |_, all_valid| {
        if all_valid {
            nav_bar.show_category(NewScreenNavBarCategory::TeamInformation);
        } else {
            nav_bar.hide_category(NewScreenNavBarCategory::TeamInformation);
        }
    }));

    window.set_child(Some(&nav_bar));
    window.show();
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum NewScreenNavBarCategory {
    BaseInformation,
    TeamInformation,
}

impl NavBarCategoryTrait for NewScreenNavBarCategory {
    fn remaining_selections(_newly_selected: Self) -> Vec<Self> {
        vec![_newly_selected]
    }

    const NAME: &'static str = "NewScreenNavBarCategory";
    const NAV_BAR_NAME: &'static str = "NavBar_NewScreenNavBarCategory";
}
