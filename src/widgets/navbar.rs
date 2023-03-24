use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{
    glib, subclass::widget::*, traits::*, Box as GtkBox, Button, Label, Separator, Stack, Widget,
};

mod inner {
    use std::{
        cell::RefCell,
        collections::{BTreeMap, HashMap},
    };

    use super::*;

    #[derive(Debug)]
    pub struct NavBar {
        stack: Stack,
        separators: [Separator; 1],
        navigation_box: GtkBox,
        buttons: RefCell<BTreeMap<NavBarCategory, CategoryEntry>>,
    }

    impl Default for NavBar {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NavBar {
        const NAME: &'static str = "NavBarImpl";
        type Type = super::NavBar;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk4::BoxLayout>();

            klass.set_css_name("navbar");
        }
    }

    impl ObjectImpl for NavBar {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            self.navigation_box.set_parent(&*obj);
            self.separators[0].set_parent(&*obj);
            self.stack.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.stack.unparent();
            self.navigation_box.unparent();
            for separator in &self.separators {
                separator.unparent();
            }
        }
    }

    impl WidgetImpl for NavBar {}

    impl NavBar {
        pub fn new() -> Self {
            let separators = [Separator::builder().css_classes(["vertical"]).build()];

            separators[0].add_css_class("highlighted");
            separators[0].set_width_request(3);

            let buttons = RefCell::new(BTreeMap::new());
            let stack = Stack::builder()
                .transition_type(gtk4::StackTransitionType::Crossfade)
                .css_name("main_window")
                .build();

            let navigation_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .spacing(0)
                .vexpand(true)
                .vexpand_set(true)
                .baseline_position(gtk4::BaselinePosition::Center)
                .css_name("navbar_buttons_box")
                .build();

            navigation_box.add_css_class("elevated");

            Self {
                stack,
                navigation_box,
                separators,
                buttons,
            }
        }

        ///
        /// Add the given child to the navigation bars stack.
        /// Also a button labelled with name is added in the corresponding category.
        /// When the button is clicked, the child is shown.
        /// Name has to be unique for a category.
        ///
        pub fn add_child(&self, child: &impl IsA<Widget>, name: &String, category: NavBarCategory) {
            {
                let buttons_map = self.buttons.borrow();

                // init category if not present
                if !buttons_map.contains_key(&category) {
                    drop(buttons_map);
                    self.init_category(category);
                }
            }

            let mut buttons_map = self.buttons.borrow_mut();

            let category_entry = match buttons_map.get_mut(&category) {
                Some(val) => val,
                None => return,
            };

            // add new child to the stack
            self.stack.add_named(child, Some(name.as_str()));

            // and create button for it
            let label = Label::new(Some(name.as_str()));
            let button = Button::builder()
                .child(&label)
                .label(name.clone())
                .css_name("nav_button")
                .build();
            category_entry.button_box.append(&button);

            // show child if according button is clicked
            let stack = &self.stack;
            button.connect_clicked(glib::clone!(@weak stack => move |button| {
                let label = button.label().unwrap();
                println!("Pressed {}!", label);

                match stack.child_by_name(label.as_str()) {
                    Some(child) => stack.set_visible_child(&child),
                    None => panic!("Cannot find child for name {}, which should be shown", label),
                }
            }));

            // store new management data
            let res = category_entry.entries.insert(name.to_string(), button);

            debug_assert!(res.is_none());
        }

        ///
        /// Removes the child labelled by name in the category.
        /// Removes the whole category if it is empty afterwards.
        ///
        pub fn remove_child_by_name(&self, name: &String, category: NavBarCategory) {
            let mut buttons_map = self.buttons.borrow_mut();

            // remove child from stack
            let child = match self.stack.child_by_name(name.as_str()) {
                Some(child) => child,
                None => return,
            };
            self.stack.remove(&child);

            // remove management data and button
            let category_entry_opt = buttons_map.get_mut(&category);
            debug_assert!(category_entry_opt.is_some());
            let category_entry = category_entry_opt.unwrap();
            let button_opt = category_entry.entries.remove(name);
            debug_assert!(button_opt.is_some());
            category_entry.button_box.remove(&button_opt.unwrap());

            // remove whole category if not needed
            if category_entry.entries.len() == 0 {
                self.remove_category(category);
            }
        }

        ///
        /// Initializes a category, i.e. creating a separator to the previous buttons
        /// and a box to store the buttons for this category.
        ///
        fn init_category(&self, category: NavBarCategory) {
            let mut buttons_map = self.buttons.borrow_mut();

            // create separator to separate from previous category
            let separator = if buttons_map.len() > 0 {
                let sep = Separator::builder().margin_top(5).margin_bottom(5).build();
                sep.add_css_class("category_sep");
                self.navigation_box.append(&sep);
                Some(sep)
            } else {
                None
            };

            // create box where buttons of this category are stored and map it
            let button_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .spacing(10)
                .margin_top(0)
                .margin_bottom(0)
                .build();
            self.navigation_box.append(&button_box);

            let res = buttons_map.insert(
                category,
                CategoryEntry {
                    button_box,
                    separator,
                    entries: HashMap::new(),
                },
            );

            debug_assert!(res.is_none());
        }

        ///
        /// Removes a category, i.e. unmapping separator and button box.
        /// Also removes all remaining children from the stack.
        ///
        fn remove_category(&self, category: NavBarCategory) {
            let mut buttons_map = self.buttons.borrow_mut();
            let category_entry = match buttons_map.remove(&category) {
                Some(category_entry) => category_entry,
                None => return,
            };

            self.navigation_box.remove(&category_entry.button_box);

            if category_entry.separator.is_some() {
                self.navigation_box
                    .remove(category_entry.separator.as_ref().unwrap());
            }

            // remove remaining children from stack
            for (name, _) in &category_entry.entries {
                self.remove_child_by_name(name, category);
            }
        }

        pub fn show_child(&self, name: &str) {
            self.stack.set_visible_child_name(name);
        }
    }

    #[derive(Debug)]
    struct CategoryEntry {
        button_box: GtkBox,
        separator: Option<Separator>,
        entries: HashMap<String, Button>,
    }
}

glib::wrapper! {
    pub struct NavBar(ObjectSubclass<inner::NavBar>)
        @extends gtk4::Widget;
}

impl NavBar {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }

    pub fn add_child(&self, child: &impl IsA<Widget>, name: String, category: NavBarCategory) {
        self.imp().add_child(child, &String::from(name), category);
    }

    pub fn remove_child_by_name(&self, name: String, category: NavBarCategory) {
        self.imp().remove_child_by_name(&name, category);
    }

    pub fn show_child(&self, name: &str) {
        self.imp().show_child(name);
    }
}

// TODO: extend
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NavBarCategory {
    Main,
    Group,
}
