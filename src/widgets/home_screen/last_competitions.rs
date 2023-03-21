use std::path::PathBuf;

use crate::widgets::tile::Tile;
use chrono::{DateTime, Local};
use gdk4::subclass::prelude::*;
use gtk4::{glib, subclass::widget::*, traits::WidgetExt, traits::*, Box as GtkBox, Label, Widget};

mod inner {

    use super::*;

    #[derive(Debug)]
    pub struct LastCompetitionsWidget {
        tile: Tile,
    }

    impl Default for LastCompetitionsWidget {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LastCompetitionsWidget {
        const NAME: &'static str = "LastCompetitions";
        type Type = super::LastCompetitionsWidget;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("last_competition");
        }
    }

    impl ObjectImpl for LastCompetitionsWidget {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let vbox = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .spacing(30)
                .build();

            for last_competition in get_last_competitions() {
                let list_item = self.create_widget_for_last_competition(&last_competition);
                vbox.append(&list_item);
            }

            self.tile.set_child(vbox);
            self.tile.set_hexpand(true);
            self.tile.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.tile.unparent();
        }
    }

    impl WidgetImpl for LastCompetitionsWidget {}

    impl LastCompetitionsWidget {
        fn new() -> Self {
            Self {
                tile: Tile::new("Last Competition"),
            }
        }

        fn create_widget_for_last_competition(&self, last_competition: &LastCompetition) -> Widget {
            let name = Label::builder()
                .label(last_competition.name.as_str())
                .xalign(0.0)
                .build();

            let info_text = format!(
                "{} - {}",
                last_competition.last_accessed.format("%d.%m.%Y - %H:%M"),
                last_competition.path.display()
            );
            let info = Label::builder()
                .label(info_text.as_str())
                .xalign(0.0)
                .css_classes(["deactivated"])
                .build();

            let vbox = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .hexpand(true)
                .spacing(0)
                .build();
            vbox.append(&name);
            vbox.append(&info);

            let hbox = GtkBox::new(gtk4::Orientation::Horizontal, 0);

            hbox.append(&vbox);
            // hbox.append(&icon);

            hbox.into()
        }
    }
}

glib::wrapper! {
    pub struct LastCompetitionsWidget(ObjectSubclass<inner::LastCompetitionsWidget>)
        @extends Widget;
}

impl LastCompetitionsWidget {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}

fn get_last_competitions() -> Vec<LastCompetition> {
    vec![
        LastCompetition {
            name: String::from("Sample Competition"),
            path: "/home/user/Documents/sample/sample.json".into(),
            last_accessed: Local::now(),
        },
        LastCompetition {
            name: String::from("Sample Competition 2"),
            path: "/home/user/Documents/sample2/sample2.json".into(),
            last_accessed: Local::now(),
        },
        LastCompetition {
            name: String::from("Sample Competition 3"),
            path: "C:\\Users\\User\\Documents\\sample3\\smaple3.json".into(),
            last_accessed: Local::now(),
        },
        LastCompetition {
            name: String::from("Sample Competition 4"),
            path: "/home/sample_user/Downloads/sample4/sample4.json".into(),
            last_accessed: Local::now(),
        },
        LastCompetition {
            name: String::from("Sample Competition 5"),
            path: "F:\\Documents\\competitions\\sample5\\sample5.israt".into(),
            last_accessed: Local::now(),
        },
    ]
}

struct LastCompetition {
    name: String,
    path: PathBuf,
    last_accessed: DateTime<Local>,
}
