// taken & adapted from https://github.com/gtk-rs/gtk4-rs/tree/master/book/listings/list_widgets/5/integer_object

use std::cell::{Cell, RefCell};

use glib::{Object, ParamSpec, Properties, Value};
use gtk4::{glib, prelude::*, subclass::prelude::*, EntryBuffer};

mod inner {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::TeamNamePositionObject)]
    pub struct TeamNamePositionObject {
        #[property(get, set)]
        position: Cell<u32>,

        #[property(get, set)]
        buffer: RefCell<EntryBuffer>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for TeamNamePositionObject {
        const NAME: &'static str = "TeamNamePositionObject";
        type Type = super::TeamNamePositionObject;
    }

    impl ObjectImpl for TeamNamePositionObject {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct TeamNamePositionObject(ObjectSubclass<inner::TeamNamePositionObject>);
}

impl TeamNamePositionObject {
    pub fn new(position: u32, buffer: EntryBuffer) -> Self {
        Object::builder().property("position", position).property("buffer", buffer).build()
    }
}
