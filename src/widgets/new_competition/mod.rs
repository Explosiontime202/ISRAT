use crate::{
    data::{CompetitionData, Competition},
    widgets::navbar::{NavBar, NavBarCategoryTrait},
    ProgramState, CompetitionPtr, open_competition_window,
};
use gdk4::glib::{self, clone};
use gtk4::{traits::*, Application, ApplicationWindow};
use std::{cell::RefCell, rc::Rc, sync::RwLock};

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
    nav_bar.add_child_with_callback(
        &team_information,
        "Team Information".to_string(),
        NewScreenNavBarCategory::TeamInformation,
        clone!(@weak base_information, @weak team_information => move |_, _, _| {
            base_information.store_data();
            team_information.reload();
        }),
    );

    // hide TeamInformation by default
    nav_bar.hide_category(NewScreenNavBarCategory::TeamInformation);

    // connect signals to show / hide categories and children
    base_information.connect_next_screen(clone!(@weak nav_bar => move |_| {
        nav_bar.show_child("Team Information", NewScreenNavBarCategory::TeamInformation);
    }));

    team_information.connect_next_screen(clone!(@weak application, @weak new_competition, @weak window => move |_| {
        // finished creation
        // TODO: save data & ask to open new window or use the old one

        new_competition.borrow_mut().generate_matches();
        let competition = CompetitionPtr::from(RwLock::new(Competition {
            data: Some(new_competition.take()),
            spawned_threads: Vec::new(),
            current_interim_result: Vec::new(),
            absolute_dir_path: None,
            absolute_file_path: None,
        }));

        

        // open new competition window
        open_competition_window(&application, competition);
        
        // TODO: use this if no competition is currently opened in the competition window
        // disable auto-save here
        /*if let Some(channel) = program_state.auto_save_channel.as_ref() {
            channel.send(crate::auto_save::AutoSaveMsg::Stop).expect("Sending autosave channel stop message failed");
        }

        competition.absolute_dir_path = None;
        competition.absolute_file_path = None;
        competition.current_interim_result.clear();
        while let Some(thread) = competition.spawned_threads.pop() {
            match thread.join() {
                Ok(()) => (),
                Err(_) => println!("Failed to join thread!"),
            };
        }
        competition.data = Some(new_competition.take());

        // enable auto-save here
        if let Some(channel) = program_state.auto_save_channel.as_ref() {
            channel.send(crate::auto_save::AutoSaveMsg::Continue).expect("Sending autosave channel continue message failed");
        }*/
        window.close();
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
