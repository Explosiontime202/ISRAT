use crate::widgets::new_competition::create_new_competition_screen;
use crate::widgets::tile::Tile;
use crate::{widgets::common::img_from_bytes, ProgramState};
use chrono::{DateTime, Local};
use gdk4::{prelude::*, subclass::prelude::*};
use gtk4::{
    glib::{self, clone},
    subclass::widget::*,
    traits::*,
    Box as GtkBox, Button, Label, Widget, Window,
};
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct LastCompetitionsWidget {
        tile: Tile,
        data: RefCell<Rc<ProgramState>>,
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

            let vbox = GtkBox::builder().orientation(gtk4::Orientation::Vertical).spacing(30).build();

            // add last competition list items
            for last_competition in get_last_competitions() {
                let list_item = self.create_widget_for_last_competition(&last_competition);
                vbox.append(&list_item);
            }

            // add icon button to create a new competition
            {
                let new_button_icon = img_from_bytes(include_bytes!("../../../resources/icons/erstellen.png"));
                let new_button_text = Label::new(Some("Create new competition"));

                let new_button_v_box = GtkBox::new(gtk4::Orientation::Horizontal, 15);
                new_button_v_box.append(&new_button_icon);
                new_button_v_box.append(&new_button_text);

                let new_competition_button = Button::builder().child(&new_button_v_box).css_name("tile_button").build();
                new_competition_button.connect_clicked(clone!(@weak self as this => move |button| {
                    println!("New competition button clicked!");
                    let window = match button.root() {
                        Some(root) => root.downcast::<Window>().unwrap(),
                        None => return,
                    };
                    let data = this.data.borrow();
                    create_new_competition_screen(window.application().as_ref().unwrap(), &*data);
                }));

                vbox.append(&new_competition_button);
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
                data: RefCell::default(),
            }
        }

        fn create_widget_for_last_competition(&self, last_competition: &LastCompetition) -> Widget {
            let name = Label::builder().label(last_competition.name.as_str()).xalign(0.0).build();

            let info_text = format!(
                "{} - {}",
                last_competition.last_accessed.format("%d.%m.%Y - %H:%M"),
                last_competition.path.display()
            );
            let info = Label::builder().label(info_text.as_str()).xalign(0.0).css_classes(["dimmed"]).build();

            let vbox = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .hexpand(true)
                .spacing(0)
                .build();
            vbox.append(&name);
            vbox.append(&info);

            let hbox = GtkBox::new(gtk4::Orientation::Horizontal, 0);

            hbox.append(&vbox);

            hbox.into()
        }

        pub fn set_data(&self, data: Rc<ProgramState>) {
            *self.data.borrow_mut() = data;
        }
    }
}

glib::wrapper! {
    pub struct LastCompetitionsWidget(ObjectSubclass<inner::LastCompetitionsWidget>)
        @extends Widget;
}

impl LastCompetitionsWidget {
    pub fn new(data: &Rc<ProgramState>) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.imp().set_data(Rc::clone(data));
        obj
    }
}

fn get_last_competitions() -> Vec<LastCompetition> {
    // TODO: retrieve from backend, i.e. gio::Settings
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
