#![windows_subsystem = "windows"]

use crate::widgets::enter_results::EnterResultScreen;
use adw::prelude::*;
use chrono::Duration;
use data::{
    read_write::{check_autosave_thread_messages, check_read_write_threads_messages, spawn_autosave_timer},
    Competition, CompetitionData, Team,
};
use gdk4::{
    gio::Menu,
    glib::{self, clone},
    Display,
};
use gtk4::{
    traits::{BoxExt, GtkApplicationExt, GtkWindowExt, WidgetExt},
    ApplicationWindow, CssProvider, StyleContext,
};
use state::{ProgramStage, ProgramState};
use std::{cell::RefCell, path::PathBuf, rc::Rc};
use widgets::{
    group_overview::GroupOverviewScreen,
    home_screen::HomeScreen,
    navbar::{NavBar, NavBarCategoryTrait},
    settings::settings_screen::SettingsScreen,
};

mod data;
mod state;
mod widgets;

type CompetitionPtr = Rc<RefCell<Competition>>;

fn main() -> glib::ExitCode {
    // initialize program state
    let mut program_state = ProgramState::new(ProgramStage::StartScreenStage, [1920.0, 1080.0]);

    // TODO: Make interval adjustable by using GUI settings or config in home directory
    spawn_autosave_timer(Duration::minutes(1), &mut program_state);

    // TODO: Remove for productive builds
    #[cfg(debug_assertions)]
    let competition = initial_state();

    let app = adw::Application::builder().application_id("de.explosiontime.Israt").build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_main_screen(app, Rc::clone(&competition)));

    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_main_screen(app: &adw::Application, competition: CompetitionPtr) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1920)
        .default_height(1080)
        .title("ISRAT")
        .show_menubar(true)
        .build();

    build_menu(app);

    let h_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    let v_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);

    build_navigation_bar(&h_box, competition);

    v_box.append(&h_box);

    window.set_child(Some(&v_box));
    window.show();
}

fn build_menu(app: &adw::Application) {
    let menu_bar = Menu::new();
    let file_menu = Menu::new();
    menu_bar.append_submenu(Some("File"), &file_menu);
    file_menu.append(Some("Test"), None);
    app.set_menubar(Some(&menu_bar));
}

fn build_navigation_bar(parent: &impl IsA<gtk4::Box>, competition: CompetitionPtr) {
    let nav_bar = NavBar::<MainNavBarCategory>::new();
    nav_bar.set_hexpand(true);
    nav_bar.set_hexpand_set(true);
    nav_bar.set_vexpand(true);
    nav_bar.set_vexpand_set(true);

    let home_screen = HomeScreen::new(&nav_bar);
    nav_bar.add_child_with_callback(&home_screen, String::from("Home Screen"), MainNavBarCategory::Main, |nav_bar, _, _| {
        nav_bar.hide_category(MainNavBarCategory::Group)
    });

    let settings_screen = SettingsScreen::new();
    nav_bar.add_child_with_callback(
        &settings_screen,
        String::from("Settings Screen"),
        MainNavBarCategory::Main,
        |nav_bar, _, _| nav_bar.hide_category(MainNavBarCategory::Group),
    );

    {
        if let Some(data) = competition.borrow().data.as_ref() {
            assert!(data.group_names.is_some());
            let group_overview = GroupOverviewScreen::new(Rc::clone(&competition));
            nav_bar.add_child_with_callback(
                &group_overview,
                String::from("Overview"),
                MainNavBarCategory::Group,
                clone!(@weak group_overview => move |_, _, _| group_overview.reload()),
            );

            let enter_results = EnterResultScreen::new(Rc::clone(&competition));
            nav_bar.add_child_with_callback(
                &enter_results,
                String::from("Enter results"),
                MainNavBarCategory::Group,
                clone!(@weak enter_results => move |_, _, _| enter_results.reload()),
            );

            for (group_idx, group) in data.group_names.as_ref().unwrap().iter().enumerate() {
                nav_bar.add_custom_nav_button(
                    group.as_str(),
                    MainNavBarCategory::GroupSelector,
                    clone!(@weak group_overview, @weak enter_results => move |nav_bar, _, _| {
                        group_overview.show_group(group_idx as u32);
                        enter_results.show_group(group_idx as u32);
                        if !nav_bar.is_category_shown(MainNavBarCategory::Group) {
                            nav_bar.show_category(MainNavBarCategory::Group);
                        }
                        let selected = nav_bar.get_selected_categories();
                        if !selected.contains(&MainNavBarCategory::Group) {
                            nav_bar.show_child("Overview", MainNavBarCategory::Group);
                        }
                    }),
                );
            }

            nav_bar.hide_category(MainNavBarCategory::Group);
        }
    }

    parent.append(&nav_bar);
}

fn check_for_thread_messages(program_state: &mut ProgramState) {
    check_read_write_threads_messages(program_state);
    check_autosave_thread_messages(program_state);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MainNavBarCategory {
    Main,
    GroupSelector,
    Group,
}

impl NavBarCategoryTrait for MainNavBarCategory {
    fn remaining_selections(newly_selected: Self) -> Vec<Self> {
        match newly_selected {
            MainNavBarCategory::Main => vec![MainNavBarCategory::Main],
            MainNavBarCategory::GroupSelector | MainNavBarCategory::Group => {
                vec![MainNavBarCategory::GroupSelector, MainNavBarCategory::Group]
            }
        }
    }

    const NAME: &'static str = "MainNavBarCategory";
    const NAV_BAR_NAME: &'static str = "NavBar_MainNavBarCategory";
}

// TODO: Remove for productive builds
#[cfg(debug_assertions)]
fn initial_state() -> CompetitionPtr {
    use std::path::Path;

    use crate::data::MatchResult;

    let competition = CompetitionPtr::default();

    let mut comp_mut = competition.borrow_mut();

    comp_mut.data = Some(CompetitionData {
        name: String::from("Mustermeisterschaft"),
        date_string: String::from("01.01.2022"),
        place: String::from("Musterstadt"),
        executor: String::from("SV Musterverein"),
        organizer: String::from("Musterverband"),
        referee: String::from("Max Muterschiedsrichter"),
        competition_manager: String::from("Erika Musterwettbewerbsleiter"),
        clerk: String::from("Max Musterschriftführer"),
        additional_text: String::from("Der SV Musterverein bedankt sich für die Teilnahme\nund wünscht ein sichere Heimreise!"),
        count_teams: 20,
        count_groups: 2,
        team_distribution: [2, 10],
        teams: Some(vec![
            vec![
                Team {
                    name: String::from("Musterteam A"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername A.1")),
                        Some(String::from("Mustername A.2")),
                        Some(String::from("Mustername A.3")),
                        Some(String::from("Mustername A.4")),
                        None,
                        None,
                        //Some(String::from("Mustername A.5")),
                        //Some(String::from("Mustername A.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam B"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername B.1")),
                        Some(String::from("Mustername B.2")),
                        Some(String::from("Mustername B.3")),
                        Some(String::from("Mustername B.4")),
                        Some(String::from("Mustername B.5")),
                        Some(String::from("Mustername B.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam C"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername C.1")),
                        Some(String::from("Mustername C.2")),
                        Some(String::from("Mustername C.3")),
                        Some(String::from("Mustername C.4")),
                        Some(String::from("Mustername C.5")),
                        Some(String::from("Mustername C.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam D"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername D.1")),
                        Some(String::from("Mustername D.2")),
                        Some(String::from("Mustername D.3")),
                        Some(String::from("Mustername D.4")),
                        Some(String::from("Mustername D.5")),
                        Some(String::from("Mustername D.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam E"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername E.1")),
                        Some(String::from("Mustername E.2")),
                        Some(String::from("Mustername E.3")),
                        Some(String::from("Mustername E.4")),
                        Some(String::from("Mustername E.5")),
                        Some(String::from("Mustername E.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam F"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername F.1")),
                        Some(String::from("Mustername F.2")),
                        Some(String::from("Mustername F.3")),
                        Some(String::from("Mustername F.4")),
                        Some(String::from("Mustername F.5")),
                        Some(String::from("Mustername F.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam G"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername G.1")),
                        Some(String::from("Mustername G.2")),
                        Some(String::from("Mustername G.3")),
                        Some(String::from("Mustername G.4")),
                        Some(String::from("Mustername G.5")),
                        Some(String::from("Mustername G.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam H"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername H.1")),
                        Some(String::from("Mustername H.2")),
                        Some(String::from("Mustername H.3")),
                        Some(String::from("Mustername H.4")),
                        Some(String::from("Mustername H.5")),
                        Some(String::from("Mustername H.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam I"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername I.1")),
                        Some(String::from("Mustername I.2")),
                        Some(String::from("Mustername I.3")),
                        Some(String::from("Mustername I.4")),
                        Some(String::from("Mustername I.5")),
                        Some(String::from("Mustername I.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam J"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername J.1")),
                        Some(String::from("Mustername J.2")),
                        Some(String::from("Mustername J.3")),
                        Some(String::from("Mustername J.4")),
                        Some(String::from("Mustername J.5")),
                        Some(String::from("Mustername J.6")),
                    ],
                },
                /*Team {
                    name: String::from("Musterteam K"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername K.1")),
                        Some(String::from("Mustername K.2")),
                        Some(String::from("Mustername K.3")),
                        Some(String::from("Mustername K.4")),
                        Some(String::from("Mustername K.5")),
                        Some(String::from("Mustername K.6")),
                    ],
                },*/
            ],
            vec![
                Team {
                    name: String::from("Musterteam N"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername N.1")),
                        Some(String::from("Mustername N.2")),
                        Some(String::from("Mustername N.3")),
                        Some(String::from("Mustername N.4")),
                        Some(String::from("Mustername N.5")),
                        Some(String::from("Mustername N.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam O"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername O.1")),
                        Some(String::from("Mustername O.2")),
                        Some(String::from("Mustername O.3")),
                        Some(String::from("Mustername O.4")),
                        Some(String::from("Mustername O.5")),
                        Some(String::from("Mustername O.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam P"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername P.1")),
                        Some(String::from("Mustername P.2")),
                        Some(String::from("Mustername P.3")),
                        Some(String::from("Mustername P.4")),
                        Some(String::from("Mustername P.5")),
                        Some(String::from("Mustername P.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam Q"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername Q.1")),
                        Some(String::from("Mustername Q.2")),
                        Some(String::from("Mustername Q.3")),
                        Some(String::from("Mustername Q.4")),
                        Some(String::from("Mustername Q.5")),
                        Some(String::from("Mustername Q.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam R"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername R.1")),
                        Some(String::from("Mustername R.2")),
                        Some(String::from("Mustername R.3")),
                        Some(String::from("Mustername R.4")),
                        Some(String::from("Mustername R.5")),
                        Some(String::from("Mustername R.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam S"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername S.1")),
                        Some(String::from("Mustername S.2")),
                        Some(String::from("Mustername S.3")),
                        Some(String::from("Mustername S.4")),
                        Some(String::from("Mustername S.5")),
                        Some(String::from("Mustername S.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam T"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername T.1")),
                        Some(String::from("Mustername T.2")),
                        Some(String::from("Mustername T.3")),
                        Some(String::from("Mustername T.4")),
                        Some(String::from("Mustername T.5")),
                        Some(String::from("Mustername T.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam U"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername U.1")),
                        Some(String::from("Mustername U.2")),
                        Some(String::from("Mustername U.3")),
                        Some(String::from("Mustername U.4")),
                        Some(String::from("Mustername U.5")),
                        Some(String::from("Mustername U.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam V"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername V.1")),
                        Some(String::from("Mustername V.2")),
                        Some(String::from("Mustername V.3")),
                        Some(String::from("Mustername V.4")),
                        Some(String::from("Mustername V.5")),
                        Some(String::from("Mustername V.6")),
                    ],
                },
                Team {
                    name: String::from("Musterteam W"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername W.1")),
                        Some(String::from("Mustername W.2")),
                        Some(String::from("Mustername W.3")),
                        Some(String::from("Mustername W.4")),
                        Some(String::from("Mustername W.5")),
                        Some(String::from("Mustername W.6")),
                    ],
                },
                /*Team {
                    name: String::from("Musterteam X"),
                    region: String::from("202"),
                    player_names: [
                        Some(String::from("Mustername X.1")),
                        Some(String::from("Mustername X.2")),
                        Some(String::from("Mustername X.3")),
                        Some(String::from("Mustername X.4")),
                        Some(String::from("Mustername X.5")),
                        Some(String::from("Mustername X.6")),
                    ],
                },*/
            ],
        ]),
        group_names: Some(vec![String::from("Gruppe BLAU"), String::from("Gruppe ROT")]),
        matches: vec![],
        current_batch: vec![1, 0],
        with_break: true,
    });
    comp_mut.data.as_mut().unwrap().generate_matches();
    comp_mut.current_interim_result = vec![None, None];

    {
        let relative_path = Path::new(if cfg!(target_os = "windows") { r".\documents\" } else { "./documents" });
        if !relative_path.exists() {
            std::fs::create_dir_all(relative_path).expect("Directory creation failed!");
        }

        let abs_path = if cfg!(target_os = "windows") {
            let tmp = std::fs::canonicalize(relative_path).expect("Canonicalize failed!");
            let tmp2 = tmp.to_str().unwrap();
            let tmp3 = tmp2[4..tmp2.len()].to_string();
            let mut path_buf = PathBuf::new();
            path_buf.push(tmp3);
            path_buf
        } else {
            std::fs::canonicalize(relative_path).expect("Canonicalize failed!")
        };

        comp_mut.absolute_dir_path = Some(abs_path);

        comp_mut.absolute_file_path = Some(comp_mut.absolute_dir_path.as_ref().unwrap().join("mustermeisterschaft.json"));
    }

    let results = [
        MatchResult::WinnerA,
        MatchResult::WinnerB,
        MatchResult::Draw,
        MatchResult::WinnerB,
        MatchResult::WinnerA,
    ];
    let points = [[17, 13], [3, 11], [9, 9], [9, 13], [11, 5]];

    comp_mut.data.as_mut().unwrap().matches[0]
        .iter_mut()
        .filter(|_match| _match.batch == 0 && _match.result != MatchResult::Break)
        .enumerate()
        .for_each(|(idx, _match)| {
            _match.result = results[idx];
            _match.points = Some(points[idx]);
        });

    let mut hash_set = std::collections::HashSet::new();
    comp_mut.data.as_ref().unwrap().matches[0]
        .iter()
        .filter(|&_match| _match.result != MatchResult::Break)
        .map(|_match| [_match.team_a.min(_match.team_b), _match.team_a.max(_match.team_b)])
        .for_each(|arr| {
            assert!(hash_set.insert(arr));
            assert_ne!(arr[0], arr[1]);
        });
    drop(comp_mut);
    competition
}
