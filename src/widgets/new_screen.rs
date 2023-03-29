use crate::widgets::new_screen;

use super::navbar::{NavBar, NavBarCategoryTrait};
use gdk4::glib::clone;
use gtk4::{glib, Inhibit};
use gtk4::{traits::*, Application, ApplicationWindow, Label};

/* mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct LastCompetitionsWidget {
        tile: Tile,
    }

    impl Default for LastCompetitionsWidget {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LastCompetitionsWidget {}
    impl ObjectImpl for LastCompetitionsWidget {}
    impl WidgetImpl for LastCompetitionsWidget {}

    impl LastCompetitionsWidget {}
}

glib::wrapper! {
    pub struct LastCompetitionsWidget(ObjectSubclass<inner::LastCompetitionsWidget>)
        @extends Widget;
}

impl LastCompetitionsWidget {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
} */

pub fn create_new_competition_screen(application: &Application, main_window: &ApplicationWindow) {
    let window = ApplicationWindow::builder()
        .application(application)
        .default_width(1280)
        .default_height(720)
        .title("New Competition")
        .decorated(true)
        .build();
    window.present();
    let nav_bar = NavBar::<NewScreenNavBarCategory>::new();

    let label1 = Label::new(Some("Base Information - Implementation"));
    nav_bar.add_child(
        &label1,
        "Base Information".to_string(),
        NewScreenNavBarCategory::A,
    );

    let label2 = Label::new(Some("Teams Information - Implementation"));
    nav_bar.add_child(
        &label2,
        "Teams Information".to_string(),
        NewScreenNavBarCategory::A,
    );

    window.set_child(Some(&nav_bar));

    {
        let m_w = main_window.clone();
        window.connect_close_request(move |_| {
            m_w.set_sensitive(true);
            m_w.set_can_target(true);
            Inhibit(false)
        });
    }
    window.show();
}

// TODO: Add real categories
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum NewScreenNavBarCategory {
    A,
}

impl NavBarCategoryTrait for NewScreenNavBarCategory {
    fn remaining_selections(_newly_selected: Self) -> Vec<Self> {
        vec![_newly_selected]
    }

    const NAME: &'static str = "NewScreenNavBarCategory";
    const NAV_BAR_NAME: &'static str = "NavBar_NewScreenNavBarCategory";
}
