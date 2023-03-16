use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{
    glib, subclass::widget::*, traits::WidgetExt, BoxLayout, Label, LayoutManager, Orientation,
    Widget,
};
use std::cell::RefCell;

mod inner {
    use gtk4::traits::WidgetExt;

    use super::*;

    #[derive(Debug)]
    pub struct Tile {
        child: RefCell<Option<Widget>>,
        title: RefCell<Option<Label>>,
    }

    impl Default for Tile {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Tile {
        const NAME: &'static str = "Tile";
        type Type = super::Tile;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<BoxLayout>();
            klass.set_css_name("tile");
        }
    }

    impl WidgetImpl for Tile {}

    impl ObjectImpl for Tile {
        fn dispose(&self) {
            if let Some(child) = self.child.borrow().as_ref() {
                child.unparent();
            };

            if let Some(title) = self.title.borrow().as_ref() {
                title.unparent();
            };
        }
    }

    impl Tile {
        fn new() -> Self {
            Self {
                child: RefCell::new(None),
                title: RefCell::new(None),
            }
        }

        pub fn set_child(&self, child: impl IsA<Widget>) {
            assert!(self.title.borrow().is_some());
            child.add_css_class("tile_child");
            child.set_hexpand(true);
            child.set_vexpand(true);
            child.set_parent(&*self.obj());
            *self.child.borrow_mut() = Some(child.into());
        }

        pub fn set_title(&self, title: &str) {
            let mut label = self.title.borrow_mut();

            match label.as_mut() {
                Some(label) => label.set_text(title),
                None => {
                    let new_label = Label::new(Some(title));
                    new_label.add_css_class("tile_headline");
                    new_label.set_parent(&*self.obj());
                    *label = Some(new_label);
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct Tile(ObjectSubclass<inner::Tile>) @extends Widget;
}

impl Tile {
    pub fn new(title: &str) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);

        obj.set_hexpand(true);
        obj.set_vexpand(true);
        obj.imp().set_title(title);
        obj
    }

    pub fn set_child(&self, child: impl IsA<Widget>) {
        self.imp().set_child(child);
    }

    pub fn set_title(&self, title: &str) {
        self.imp().set_title(title);
    }
}
