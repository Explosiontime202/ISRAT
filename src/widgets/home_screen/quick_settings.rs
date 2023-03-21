use crate::widgets::common::img_from_bytes;
use crate::widgets::settings::get_quick_settings;
use crate::widgets::tile::Tile;
use gdk4::subclass::prelude::*;
use gtk4::{
    glib, subclass::widget::*, traits::WidgetExt, traits::*, Box as GtkBox, Button, Label, Widget,
};

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct QuickSettingsWidget {
        tile: Tile,
    }

    impl Default for QuickSettingsWidget {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for QuickSettingsWidget {
        const NAME: &'static str = "QuickSettings";
        type Type = super::QuickSettingsWidget;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("quick_settings");
        }
    }

    impl ObjectImpl for QuickSettingsWidget {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let vbox = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .spacing(30)
                .build();

            get_quick_settings()
                .iter()
                .for_each(|setting| vbox.append(setting));

            // add icon button to open to the settings screen
            {
                let settings_button_icon =
                    img_from_bytes(include_bytes!("../../../resources/icons/gear.png"));
                let settings_button_text = Label::new(Some("Open settings"));

                let settings_button_v_box = GtkBox::new(gtk4::Orientation::Horizontal, 15);
                settings_button_v_box.append(&settings_button_icon);
                settings_button_v_box.append(&settings_button_text);

                let open_settings_button = Button::builder()
                    .child(&settings_button_v_box)
                    .css_name("tile_button")
                    .build();
                open_settings_button.connect_clicked(|_| {
                    println!("Open settings button clicked!");
                    // TODO: switch to settings screen
                });

                vbox.append(&open_settings_button);
            }

            self.tile.set_child(vbox);
            self.tile.set_hexpand(true);
            self.tile.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.tile.unparent();
        }
    }

    impl WidgetImpl for QuickSettingsWidget {}

    impl QuickSettingsWidget {
        fn new() -> Self {
            Self {
                tile: Tile::new("Quick Settings"),
            }
        }
    }
}

glib::wrapper! {
    pub struct QuickSettingsWidget(ObjectSubclass<inner::QuickSettingsWidget>)
        @extends Widget;
}

impl QuickSettingsWidget {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}
