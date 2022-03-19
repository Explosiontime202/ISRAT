use imgui::*;
use screens::new_screen::NewScreenState;
use winit::window::Fullscreen;

mod screens;

mod support;

// TODO: Add auto saving

fn main() {
    let mut system = support::init("Ergebnis-Manager");

    // set borderless full screen at start
    system
        .display
        .gl_window()
        .window()
        .set_fullscreen(Some(Fullscreen::Borderless(None)));

    // ger monitor size
    let size = system.display.gl_window().window().inner_size();

    // initialize program state
    let mut state = ProgramState::new(
        ProgramStage::StartScreenStage,
        [size.width as f32, size.height as f32],
    );

    // set color theme
    let style = system.imgui.style_mut();
    style.colors[StyleColor::TitleBgActive as usize] = style.colors[StyleColor::TitleBg as usize];

    // start main loop
    system.main_loop(move |run, ui, window| {
        let size = window.inner_size();
        state.size = [size.width as f32, size.height as f32];

        let window_border_size_token = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
        let window_padding_token = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        Window::new("Ergebnis-Manager")
            .size(state.size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .no_decoration()
            .title_bar(true)
            .no_nav()
            .bring_to_front_on_focus(false)
            .resizable(false)
            .opened(run)
            .build(ui, || {
                screens::build(ui, &mut state);

                // Escape is pressed, exit fullscreen mode
                if ui.io().keys_down[36] {
                    window.set_fullscreen(None);
                }

                // F11 is pressed, enter fullscreen mode
                if ui.io().keys_down[47] {
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }

                //if let Some(key) = ui.io().keys_down.iter().position(|&k| k == true) {
                //    println!("pressed_key = {}", key);
                //}

                /*add_main_menu(ui);
                ui.text("Hello world!");
                ui.text("こんにちは世界！");
                ui.text("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
                let bg_color = ui.push_style_color(StyleColor::ChildBg, [1.0, 0.0, 0.0, 1.0]);*/
                /*Window::new("Hello Welt")
                .size([200.0, 200.0], Condition::Always)
                .no_decoration()
                .position([200.0, 100.0], Condition::Always)
                .build(ui, || {
                    let text_color =
                        ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);
                    ui.text(format!("Screen Size: ({:.1}, {:.1})", width, height));
                    let c = ui.style_color(StyleColor::Text);
                    ui.text(format!("{} {} {} {}", c[0], c[1], c[2], c[3]));
                    text_color.pop();
                });*/
                /*ChildWindow::new("Hello Welt")
                    .size([200.0, 200.0])
                    .build(ui, || {
                        let text_color =
                            ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);
                        ui.text(format!("Screen Size: ({:.1}, {:.1})", 0.0, 0.0));
                        let c = ui.style_color(StyleColor::Text);
                        ui.text(format!("{} {} {} {}", c[0], c[1], c[2], c[3]));
                        text_color.pop();
                    });
                bg_color.pop();*/
            });
        window_padding_token.pop();
        window_border_size_token.pop();
    });
}

// TODO: Use this method properly
fn add_main_menu(ui: &Ui) {
    if let Some(main_menu_bar_token) = ui.begin_main_menu_bar() {
        if let Some(file_menu_token) = ui.begin_menu("File") {
            if MenuItem::new("New").build(ui) {
                // TODO: Implement new call
            }
            if MenuItem::new("Open").build(ui) {
                // TODO: Implement open saved data
            }
            if MenuItem::new("Save").build(ui) {
                // TODO: Implement save data, same as "Save as" if no file to save is specified
            }
            if MenuItem::new("Save as").build(ui) {
                // TODO: Implement save data as file (specify file)
            }
            file_menu_token.end();
        }
        main_menu_bar_token.end();
    }
}

#[derive(Clone, Copy)]
pub enum ProgramStage {
    StartScreenStage,
    NewScreenStage,
}

pub struct CompetitionData {
    pub name: String,
    pub date_string: String,
    pub place: String,
    pub executor: String,
    pub organizer: String,
    pub count_teams: u32,
    pub team_distribution: [u32; 2], // count_groups x count_teams_per_group
    pub teams: Option<Vec<Vec<Team>>>, // for each group a vector of teams, ordered by ids
    pub group_names: Option<Vec<String>>, // a vector of the group names, ordered by id
}

impl CompetitionData {
    pub fn empty() -> CompetitionData {
        CompetitionData {
            name: String::from(""),
            date_string: String::from(""),
            place: String::from(""),
            executor: String::from(""),
            organizer: String::from(""),
            count_teams: 0,
            team_distribution: [0, 0],
            teams: None,
            group_names: None,
        }
    }
}

pub struct Team {
    pub name: String,
    pub player_names: [Option<String>; 6], // maximal 6 possible players per team
}

pub struct ProgramState {
    pub stage: ProgramStage,
    pub size: [f32; 2],
    pub competition_data: Option<CompetitionData>,
    pub new_screen_state: Option<NewScreenState>,
}

impl ProgramState {
    pub fn new(stage: ProgramStage, size: [f32; 2]) -> ProgramState {
        ProgramState {
            stage,
            size,
            competition_data: None,
            new_screen_state: None,
        }
    }
}
