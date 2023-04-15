use crate::widgets::navbar::{NavBar, NavBarCategoryTrait};
use gtk4::{traits::*, Application, ApplicationWindow};

use self::base_information::BaseInformationScreen;

mod base_information;
mod group_page;
mod team_information;
mod team_name_position_object;

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

    let base_information = BaseInformationScreen::new();
    nav_bar.add_child(
        &base_information,
        "Base Information".to_string(),
        NewScreenNavBarCategory::BaseInformation,
    );

    window.set_child(Some(&nav_bar));
    window.show();
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum NewScreenNavBarCategory {
    BaseInformation,
}

impl NavBarCategoryTrait for NewScreenNavBarCategory {
    fn remaining_selections(_newly_selected: Self) -> Vec<Self> {
        vec![_newly_selected]
    }

    const NAME: &'static str = "NewScreenNavBarCategory";
    const NAV_BAR_NAME: &'static str = "NavBar_NewScreenNavBarCategory";
}
