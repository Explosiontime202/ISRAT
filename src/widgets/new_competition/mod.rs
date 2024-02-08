use self::{base_information::BaseInformationScreen, team_information::TeamInformationScreen};
use crate::{
    add_group_screens,
    data::{Competition, CompetitionData},
    open_competition_window, reload_group_screens,
    widgets::navbar::{NavBar, NavBarCategoryTrait},
    CompetitionPtr, ProgramState,
};
use gdk4::{
    gio::{Cancellable, CancellableFuture},
    glib::{self, clone, MainLoop},
};
use gtk4::{traits::*, AlertDialog, Application, ApplicationWindow};
use std::{cell::RefCell, rc::Rc, sync::RwLock};

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
    nav_bar.add_child(&base_information, "Base Information", NewScreenNavBarCategory::BaseInformation);

    let team_information = TeamInformationScreen::new(&new_competition);
    nav_bar.add_child_with_callback(
        &team_information,
        "Team Information",
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

    team_information.connect_next_screen(
        clone!(@weak application, @weak program_state, @weak new_competition, @weak window => move |_| {
            // finished creation => update old window or open new one

            if program_state.competition.read().expect("Competition is poisoned!").data.is_none() {
                open_in_this_window(&program_state, &new_competition);
                window.close();
            } else {
                let dialog = AlertDialog::builder()
                    .buttons(["New window","This window", "Cancel"])
                    .message("Test")
                    .cancel_button(2)
                    .default_button(0)
                    .modal(true)
                    .build();

                let l = MainLoop::new(None, false);
                let c = Cancellable::new();

                l.context().spawn_local(CancellableFuture::new(async move {
                    match dialog.choose_future(Some(&window)).await {
                        Ok(button_id) => {
                            println!("pressed button with id = {button_id}");
                            match button_id {
                                0 => open_in_new_window(&new_competition, &application, &window), // new window button
                                1 => {
                                    // this window button
                                    open_in_this_window(&program_state, &new_competition);
                                    window.close();
                                },
                                2 => (), // cancel button
                                _ => panic!("Unknown button of AlertDialog has been pressed!"),
                            }
                        },
                        Err(err) => eprintln!("{}", err.to_string())
                    }
                }, c.clone()));
            }
        }),
    );

    base_information.connect_all_entries_valid(clone!(@weak nav_bar => move |_, all_valid| {
        if all_valid {
            nav_bar.show_category(NewScreenNavBarCategory::TeamInformation);
        } else {
            nav_bar.hide_category(NewScreenNavBarCategory::TeamInformation);
        }
    }));

    window.set_child(Some(&nav_bar));
    window.set_visible(true);
}

fn open_in_new_window(new_competition: &Rc<RefCell<CompetitionData>>, application: &Application, window: &ApplicationWindow) {
    // generate matches for the new competition
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
    window.close();
}

fn open_in_this_window(program_state: &Rc<ProgramState>, new_competition: &Rc<RefCell<CompetitionData>>) {
    // generate matches for the new competition
    new_competition.borrow_mut().generate_matches();

    let has_prev_competition = program_state.competition.read().expect("Competition is poisoned!").data.is_some();

    if has_prev_competition {
        // TODO: need to save old data beforehand
    }

    let mut competition = program_state.competition.write().expect("Competition is poisoned!");

    // disable auto-save here
    if let Some(channel) = program_state.auto_save_channel.as_ref() {
        channel
            .send(crate::auto_save::AutoSaveMsg::Stop)
            .expect("Sending autosave channel stop message failed");
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
        channel
            .send(crate::auto_save::AutoSaveMsg::Continue)
            .expect("Sending autosave channel continue message failed");
    }

    drop(competition);

    if has_prev_competition {
        // only need to update screens & group selector buttons
        reload_group_screens(&program_state);
    } else {
        // need to add new screens for competition
        add_group_screens(&program_state);
    }
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
