use crate::widgets::navbar::{NavBar, NavBarCategoryTrait};
use gdk4::glib::{self, clone};
use gtk4::{traits::*, Application, ApplicationWindow};

use self::{base_information::BaseInformationScreen, team_information::TeamInformationScreen};

mod base_information;
mod group_page;
mod team_information;
mod group_team_object;

pub fn create_new_competition_screen(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .default_width(1280)
        .default_height(720)
        .title("Create new Competition")
        .decorated(true)
        .modal(true)
        .build();
    window.present();

    let nav_bar = NavBar::<NewScreenNavBarCategory>::with_use_separators(false);

    // add screens
    let base_information = BaseInformationScreen::new();
    nav_bar.add_child(
        &base_information,
        "Base Information".to_string(),
        NewScreenNavBarCategory::BaseInformation,
    );

    let team_information = TeamInformationScreen::new();
    nav_bar.add_child(
        &team_information,
        "Team Information".to_string(),
        NewScreenNavBarCategory::TeamInformation,
    );

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
