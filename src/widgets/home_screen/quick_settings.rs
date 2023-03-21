use crate::widgets::tile::Tile;
use gdk4::subclass::prelude::*;
use gtk4::{glib, subclass::widget::*, traits::WidgetExt, traits::*, Box as GtkBox, Widget};

mod inner {

    use crate::widgets::settings::get_quick_settings;

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
