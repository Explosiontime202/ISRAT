#![windows_subsystem = "windows"]
#![allow(incomplete_features)]
#![feature(adt_const_params)]

use adw::prelude::*;
use auto_save::{spawn_autosave_thread, AutoSaveMsg, AutoSaveThread};
use data::{Competition, CompetitionData, Team};
use gdk4::{
    gio::Menu,
    glib::{self, clone, once_cell::sync::Lazy},
    Display,
};
use gtk4::{
    traits::{BoxExt, GtkApplicationExt, GtkWindowExt, WidgetExt},
    ApplicationWindow, CssProvider, StyleContext,
};
use std::{
    path::PathBuf,
    rc::Rc,
    sync::{mpsc::SyncSender, Arc, RwLock},
    time::Duration,
};
use widgets::{
    enter_results::EnterResultScreen,
    group_overview::GroupOverviewScreen,
    group_screen::GroupScreen,
    home_screen::HomeScreen,
    match_history::MatchHistoryScreen,
    navbar::{NavBar, NavBarCategoryTrait},
    settings::settings_screen::SettingsScreen,
};

mod auto_save;
mod data;
mod widgets;

type CompetitionPtr = Arc<RwLock<Competition>>;

static AUTO_SAVE_THREADS: Lazy<RwLock<Vec<AutoSaveThread>>> = Lazy::new(|| {
    return RwLock::from(Vec::new());
});

fn main() -> glib::ExitCode {
    #[cfg(debug_assertions)]
    let competition = initial_state();

    #[cfg(not(debug_assertions))]
    let competition = CompetitionPtr::default();

    let app = adw::Application::builder().application_id("de.explosiontime.Israt").build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| open_competition_window(app.upcast_ref(), Arc::clone(&competition)));

    let ret = app.run();

    let mut auto_save_threads = AUTO_SAVE_THREADS.write().expect("AUTO_SAVE_THREADS is poisoned!");
    while let Some(auto_save_thread) = auto_save_threads.pop() {
        auto_save_thread.stop();
    }
    ret
}

fn open_competition_window(app: &gtk4::Application, competition: CompetitionPtr) {
    let auto_save_channel = spawn_autosave_thread(Duration::new(15, 0), Arc::downgrade(&competition));

    let program_state = Rc::new(ProgramState {
        competition: Arc::clone(&competition),
        auto_save_channel: Some(auto_save_channel),
    });

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

    build_navigation_bar(&h_box, program_state);

    v_box.append(&h_box);

    window.set_child(Some(&v_box));
    window.show();
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

fn build_menu(app: &gtk4::Application) {
    let menu_bar = Menu::new();
    let file_menu = Menu::new();
    menu_bar.append_submenu(Some("File"), &file_menu);
    file_menu.append(Some("Test"), None);
    app.set_menubar(Some(&menu_bar));
}

fn build_navigation_bar(parent: &impl IsA<gtk4::Box>, program_state: Rc<ProgramState>) {
    let nav_bar = NavBar::<MainNavBarCategory>::new();
    nav_bar.set_hexpand(true);
    nav_bar.set_hexpand_set(true);
    nav_bar.set_vexpand(true);
    nav_bar.set_vexpand_set(true);

    let home_screen = HomeScreen::new(&nav_bar, &program_state);
    nav_bar.add_child_with_callback(&home_screen, String::from("Home Screen"), MainNavBarCategory::Main, |nav_bar, _, _| {
        nav_bar.hide_category(MainNavBarCategory::Group)
    });

    let settings_screen = SettingsScreen::new(&program_state);
    nav_bar.add_child_with_callback(
        &settings_screen,
        String::from("Settings Screen"),
        MainNavBarCategory::Main,
        |nav_bar, _, _| nav_bar.hide_category(MainNavBarCategory::Group),
    );

    {
        if let Some(data) = program_state.competition.read().unwrap().data.as_ref() {
            let group_overview = GroupOverviewScreen::new(&program_state);
            nav_bar.add_child_with_callback(
                &group_overview,
                String::from("Overview"),
                MainNavBarCategory::Group,
                clone!(@weak group_overview => move |_, _, _| group_overview.reload()),
            );

            let enter_results = EnterResultScreen::new(&program_state);
            nav_bar.add_child_with_callback(
                &enter_results,
                String::from("Enter results"),
                MainNavBarCategory::Group,
                clone!(@weak enter_results => move |_, _, _| enter_results.reload()),
            );

            let match_history = MatchHistoryScreen::new(&program_state);
            nav_bar.add_child_with_callback(
                &match_history,
                String::from("Match History"),
                MainNavBarCategory::Group,
                clone!(@weak match_history => move |_, _, _| match_history.reload()),
            );

            for (group_idx, group) in data.groups.iter().enumerate() {
                nav_bar.add_custom_nav_button(
                    group.name.as_str(),
                    MainNavBarCategory::GroupSelector,
                    clone!(@weak group_overview, @weak enter_results, @weak match_history => move |nav_bar, _, _| {
                            group_overview.show_group(group_idx as u32);
                            enter_results.show_group(group_idx as u32);
                            match_history.show_group(group_idx as u32);

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

#[derive(Debug)]
pub struct ProgramState {
    competition: CompetitionPtr,
    auto_save_channel: Option<SyncSender<AutoSaveMsg>>,
}

impl Default for ProgramState {
    fn default() -> Self {
        Self {
            competition: Default::default(),
            auto_save_channel: Default::default(),
        }
    }
}

// TODO: Remove for productive builds
#[cfg(debug_assertions)]
fn initial_state() -> CompetitionPtr {
    use std::path::Path;

    use crate::data::{Group, MatchResult};

    let competition = CompetitionPtr::default();

    let mut comp_mut = competition.write().unwrap();

    let mut data = CompetitionData::default();

    data.name = String::from("Mustermeisterschaft");
    data.date_string = String::from("01.01.2022");
    data.time_string = String::from("07:00");
    data.location = String::from("Musterstadt");
    data.executor = String::from("SV Musterverein");
    data.organizer = String::from("Musterverband");
    data.referee = String::from("Max Muterschiedsrichter");
    data.competition_manager = String::from("Erika Musterwettbewerbsleiter");
    data.secretary = String::from("Max Musterschriftführer");
    data.additional_text = String::from("Der SV Musterverein bedankt sich für die Teilnahme\nund wünscht ein sichere Heimreise!");
    data.count_teams = 20;

    let mut start_char = 'A';
    for name in ["Gruppe BLAU", "Gruppe ROT"] {
        let mut group = Group {
            name: String::from(name),
            teams: Vec::new(),
            with_break: true,
            count_batches: 0,
            current_batch: 0,
            matches: Vec::new(),
        };
        let end_char = (start_char as u8 + 10) as char;
        for c in start_char..=end_char {
            let team = Team::with_player_names(
                &mut data,
                format!("Musterteam {c}"),
                String::from("202"),
                [
                    Some(format!("Mustername {c}.1")),
                    Some(format!("Mustername {c}.2")),
                    Some(format!("Mustername {c}.3")),
                    Some(format!("Mustername {c}.4")),
                    Some(format!("Mustername {c}.5")),
                    Some(format!("Mustername {c}.6")),
                ],
            );

            group.teams.push(team);
        }

        data.groups.push(group);
        start_char = (end_char as u8 + 1) as char;
    }

    data.generate_matches();
    comp_mut.current_interim_result = vec![Vec::new(), Vec::new()];

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

    data.groups[0]
        .matches
        .iter_mut()
        .filter(|_match| _match.batch == 0 && _match.result != MatchResult::Break)
        .enumerate()
        .for_each(|(idx, _match)| {
            _match.result = results[idx];
            _match.points = Some(points[idx]);
        });

    let mut hash_set = std::collections::HashSet::new();
    data.groups[0]
        .matches
        .iter()
        .filter(|&_match| _match.result != MatchResult::Break)
        .map(|_match| [_match.team_a.min(_match.team_b), _match.team_a.max(_match.team_b)])
        .for_each(|arr| {
            assert!(hash_set.insert(arr));
            assert_ne!(arr[0], arr[1]);
        });

    data.groups[0].current_batch = 1;

    comp_mut.data = Some(data);
    drop(comp_mut);
    competition
}
