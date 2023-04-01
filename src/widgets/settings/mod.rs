use std::rc::Rc;
use std::time::Duration;

use crate::auto_save::AutoSaveMsg;
use crate::ProgramState;
use gdk4::glib::clone;
use gdk4::prelude::*;
use gtk4::traits::ToggleButtonExt;
use gtk4::{glib, Adjustment, Orientation, PositionType, Scale, ToggleButton};
use gtk4::{traits::WidgetExt, traits::*, Box as GtkBox, CenterBox, DropDown, Expression, Inhibit, Label, StringList, Switch, Widget};

pub mod settings_screen;

pub fn create_settings(program_state: &Rc<ProgramState>) -> Vec<SettingsCategoryData> {
    let mut vec: Vec<SettingsCategoryData> = Vec::new();

    let appearance = SettingsCategoryData {
        category: SettingsCategory::Appearance,
        setting_widgets: create_appearance_settings(program_state),
    };
    vec.push(appearance);

    let behavior = SettingsCategoryData {
        category: SettingsCategory::Behavior,
        setting_widgets: create_behavior_settings(program_state),
    };
    vec.push(behavior);

    vec
}

pub fn create_quick_settings(program_state: &Rc<ProgramState>) -> Vec<Widget> {
    let mut vec = vec![create_new_rules_switch(), create_auto_save_interval_setting(program_state)];

    vec.insert(
        1,
        create_auto_save_switch(
            vec[1]
                .clone()
                .dynamic_cast::<CenterBox>()
                .unwrap()
                .last_child()
                .unwrap()
                .dynamic_cast::<DropDown>()
                .unwrap(),
            program_state,
        ),
    );
    vec
}

fn create_appearance_settings(_program_state: &Rc<ProgramState>) -> Vec<Widget> {
    vec![
        create_language_setting(),
        create_dark_light_mode_switch(),
        create_color_theme_setting(),
        create_text_size_setting(),
    ]
}

fn create_behavior_settings(program_state: &Rc<ProgramState>) -> Vec<Widget> {
    let mut vec = vec![
        create_auto_save_interval_setting(program_state),
        create_new_rules_switch(),
        create_fullscreen_switch(),
        create_keybindings_setting(),
    ];

    vec.insert(
        0,
        create_auto_save_switch(
            vec[0]
                .clone()
                .dynamic_cast::<CenterBox>()
                .unwrap()
                .last_child()
                .unwrap()
                .dynamic_cast::<DropDown>()
                .unwrap(),
            program_state,
        ),
    );
    vec
}

fn create_language_setting() -> Widget {
    // TODO: Change language of GUI accordingly
    create_settings_selector("Language", &["EN", "DE"], |_, sel_idx, sel_option| {
        println!("Language: Selected {sel_option} at idx {sel_idx}")
    })
}

fn create_dark_light_mode_switch() -> Widget {
    // TODO: Change style, i.e. use selected Dark/Light mode
    let toggle_button_left = ToggleButton::with_label("Dark Mode");
    toggle_button_left.set_active(true);
    let toggle_button_right = ToggleButton::with_label("Light Mode");
    toggle_button_left.set_group(Some(&toggle_button_right));
    let hbox = GtkBox::new(Orientation::Horizontal, 0);
    hbox.append(&toggle_button_left);
    hbox.append(&toggle_button_right);

    toggle_button_left.connect_clicked(|_| {
        println!("Style: Dark Mode!");
    });

    toggle_button_right.connect_clicked(|_| {
        println!("Style: Light Mode!");
    });

    create_setting_base_widget("Style", &hbox)
}

fn create_color_theme_setting() -> Widget {
    // TODO: Change color theme
    create_settings_selector(
        "Auto-save interval",
        &[
            "Color Theme A",
            "Color Theme B",
            "Color Theme C",
            "Color Theme D",
            "Color Theme E",
            "Color Theme F",
        ],
        |_, sel_idx, sel_option| println!("Color Theme: Selected {sel_option} at idx {sel_idx}"),
    )
}

fn create_text_size_setting() -> Widget {
    // TODO: Adjust text size
    // TODO: Use scale

    let scale = Scale::new(
        Orientation::Horizontal,
        Some(&Adjustment::builder().lower(0.5).upper(2.5).step_increment(0.1).build()),
    );
    scale.connect_change_value(|_, scroll_type, value| {
        println!("Text size: Set to {value} via {scroll_type}");
        Inhibit::default()
    });
    scale.set_draw_value(true);
    scale.set_value_pos(PositionType::Right);
    scale.set_value(1.0);
    scale.set_size_request(150, 0);

    create_setting_base_widget("Text size", &scale)
}

fn create_auto_save_switch(interval_drop_down: DropDown, program_state: &Rc<ProgramState>) -> Widget {
    let program_state_weak = Rc::downgrade(program_state);
    create_settings_switch("Auto-save", move |_, state| {
        println!("Switch 'Auto-save' is now {state}");
        interval_drop_down.set_sensitive(state);
        let msg = match state {
            true => AutoSaveMsg::Continue,
            false => AutoSaveMsg::Stop,
        };
        match program_state_weak.upgrade() {
            Some(program_state) => program_state.auto_save_channel.send(msg).unwrap(),
            None => eprintln!("Cannot send msg to auto-save!"),
        }
    })
}

fn create_auto_save_interval_setting(program_state: &Rc<ProgramState>) -> Widget {
    let auto_save_intervals = vec![
        ("15s", Duration::new(15, 0)),
        ("30s", Duration::new(30, 0)),
        ("1 min", Duration::new(60, 0)),
        ("2 min", Duration::new(120, 0)),
        ("5 min", Duration::new(300, 0)),
        ("10 min", Duration::new(600, 0)),
        ("20 min", Duration::new(1200, 0)),
    ];

    let test = auto_save_intervals.iter().map(|(s, _)| *s).collect::<Vec<&str>>();
    let program_state_weak = Rc::downgrade(program_state);
    create_settings_selector("Auto-save interval", &test, move |_, sel_idx, _| match program_state_weak.upgrade() {
        Some(program_state) => program_state
            .auto_save_channel
            .send(AutoSaveMsg::Interval(auto_save_intervals[sel_idx as usize].1))
            .unwrap(),
        None => eprintln!("Cannot send msg to auto-save!"),
    })
}

fn create_new_rules_switch() -> Widget {
    // TODO: Callback into backend
    create_settings_switch("Use new rules", |_, state| println!("Switch 'use new rules' is now {state}!"))
}

fn create_fullscreen_switch() -> Widget {
    // TODO: Switch to fullscreen
    create_settings_switch("Fullscreen", |_, state| println!("Switch 'Fullscreen' is now {state}"))
}

fn create_keybindings_setting() -> Widget {
    Label::new(Some("TODO: Key Bindings")).into()
}

fn create_settings_switch<F: Fn(&Switch, bool) + 'static>(text: &str, callback: F) -> Widget {
    let switch = Switch::builder().state(true).build();
    // TODO: Read current state from settings/backend and set accordingly
    switch.set_state(true);
    switch.connect_state_set(move |switch, state| {
        callback(switch, state);
        Inhibit::default()
    });
    create_setting_base_widget(text, &switch)
}

fn create_settings_selector<F: Fn(&DropDown, u32, String) + 'static>(text: &str, options: &[&str], callback: F) -> Widget {
    let drop_down = DropDown::new(Some(StringList::new(options)), Expression::NONE);

    // highlight selected item when showing the popover
    drop_down.last_child().unwrap().connect_show(clone!(@weak drop_down => move |_| {
        iter_drop_down_items(&drop_down, clone!(@weak drop_down => move |child, idx| {
            if idx == drop_down.selected() {
                child.add_css_class("selected");
            }
        }));
    }));

    drop_down.connect_selected_item_notify(move |drop_down| {
        let selected_idx = drop_down.selected();

        let selected_option = drop_down.selected_item().unwrap().property::<String>("string");
        callback(drop_down, selected_idx, selected_option);

        // find selected listitem and set background of it
        // clear background of others
        iter_drop_down_items(&drop_down, move |child, idx| {
            if idx == selected_idx {
                child.add_css_class("selected");
            } else {
                child.remove_css_class("selected");
            }
        });
    });
    create_setting_base_widget(text, &drop_down)
}

fn iter_drop_down_items<F: Fn(&Widget, u32) + 'static>(drop_down: &DropDown, f: F) {
    // traverse widget tree to get the list items
    let mut child_opt = drop_down
        .last_child() // Popover
        .unwrap()
        .first_child() // PopoverContent
        .unwrap()
        .first_child() // GtkBox
        .unwrap()
        .last_child() // ScrolledWindow
        .unwrap()
        .first_child() // ListView
        .unwrap()
        .first_child(); // first ListItem

    // iterate over all siblings and call callback
    let mut counter = 0u32;
    while let Some(child) = child_opt.as_ref() {
        f(&child, counter);

        counter += 1;
        child_opt = child.next_sibling();
    }
}

fn create_setting_base_widget(text: &str, child: &impl IsA<Widget>) -> Widget {
    let label = Label::new(Some(text));
    let setting = CenterBox::new();
    setting.set_start_widget(Some(&label));
    setting.set_end_widget(Some(child.as_ref()));
    setting.into()
}

pub struct SettingsCategoryData {
    pub category: SettingsCategory,
    pub setting_widgets: Vec<Widget>,
}

pub enum SettingsCategory {
    Appearance,
    Behavior,
}

impl ToString for SettingsCategory {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Appearance => "Appearance",
            Self::Behavior => "Behavior",
        })
    }
}
