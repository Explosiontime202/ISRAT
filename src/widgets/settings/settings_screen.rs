use crate::widgets::settings::create_settings;
use crate::widgets::tile::Tile;
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::traits::*;
use gtk4::{glib, subclass::widget::*, traits::WidgetExt, Box as GtkBox, FlowBox, Label, LayoutManager, Orientation, Widget};

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct SettingsScreen {
        flow_box: FlowBox,
        title: Label,
    }

    impl Default for SettingsScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SettingsScreen {
        const NAME: &'static str = "SettingsScreen";
        type Type = super::SettingsScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("home_screen");
        }
    }
    impl ObjectImpl for SettingsScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let settings = create_settings();
            for data in settings {
                let tile = Tile::new(data.category.to_string().as_str());
                let vbox = GtkBox::builder().orientation(gtk4::Orientation::Vertical).spacing(30).build();
                for setting in data.setting_widgets {
                    vbox.append(&setting);
                }
                tile.set_child(vbox);
                self.flow_box.insert(&tile, -1);
            }

            self.title.set_parent(&*obj);
            self.flow_box.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.flow_box.unparent();
            self.title.unparent();
        }
    }

    impl WidgetImpl for SettingsScreen {}

    impl SettingsScreen {
        fn new() -> Self {
            Self {
                flow_box: FlowBox::builder()
                    .max_children_per_line(2)
                    .min_children_per_line(2)
                    .orientation(gtk4::Orientation::Horizontal)
                    .selection_mode(gtk4::SelectionMode::None)
                    .homogeneous(true)
                    .build(),
                title: Label::builder().label("Settings").css_classes(["headline"]).build(),
            }
        }
    }
}

glib::wrapper! {
    pub struct SettingsScreen(ObjectSubclass<inner::SettingsScreen>)
        @extends Widget;
}

impl SettingsScreen {
    pub fn new() -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.set_hexpand(true);
        obj
    }
}
