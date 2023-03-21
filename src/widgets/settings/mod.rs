use gdk4::glib::clone;
use gdk4::prelude::*;
use gtk4::{
    traits::WidgetExt, CenterBox, DropDown, Expression, Inhibit, Label, StringList, Switch, Widget,
};

use gtk4::glib;

// pub fn get_settings() -> Vec<Widget> {}

pub fn get_quick_settings() -> Vec<Widget> {
    let mut vec: Vec<Widget> = Vec::new();

    {
        let label = Label::new(Some("Use new rules"));
        let switch = Switch::builder().state(true).build();
        switch.set_state(true);
        switch.connect_state_set(|_, state| {
            // TODO: Callback into backend
            println!("Switch 'use new rules' is now {state}!");
            Inhibit::default()
        });
        let rule_setting = CenterBox::new();
        rule_setting.set_start_widget(Some(&label));
        rule_setting.set_end_widget(Some(&switch));
        vec.push(rule_setting.into());
    }

    {
        let label = Label::new(Some("Auto-save"));
        let switch = Switch::builder().state(true).build();
        switch.connect_state_set(|_, state| {
            // TODO: Callback into backend, i.e. start or stop auto-save thread
            println!("Switch 'Auto-save' is now {state}!");
            Inhibit::default()
        });
        let auto_save_setting = CenterBox::new();
        auto_save_setting.set_start_widget(Some(&label));
        auto_save_setting.set_end_widget(Some(&switch));
        vec.push(auto_save_setting.into());
    }

    vec.push(create_auto_save_interval());

    return vec;
}

fn create_auto_save_interval() -> Widget {
    let label = Label::new(Some("Auto-save interval"));

    let drop_down = DropDown::new(
        Some(StringList::new(&["TEST A", "TEST B", "TEST C"])),
        Expression::NONE,
    );

    // highlight selected item when showing the popover
    drop_down
        .last_child()
        .unwrap()
        .connect_show(clone!(@weak drop_down => move |_| {
            iter_drop_down_items(&drop_down, clone!(@weak drop_down => move |child, idx| {
                if idx == drop_down.selected() {
                    child.add_css_class("selected");
                }
            }));
        }));

    drop_down.connect_selected_item_notify(|drop_down| {
        let selected_idx = drop_down.selected();
        println!(
            "Selected {} at idx {selected_idx}",
            drop_down
                .selected_item()
                .unwrap()
                .property::<String>("string")
        );

        // TODO: Set auto-save interval in backend

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

    let auto_save_interval_setting = CenterBox::new();
    auto_save_interval_setting.set_start_widget(Some(&label));
    auto_save_interval_setting.set_end_widget(Some(&drop_down));
    auto_save_interval_setting.into()
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
